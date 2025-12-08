#![recursion_limit = "512"]

use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::*,
};
use leptos_config::LeptosOptions;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

// ============================================================================
// Admin Authentication - Server-side State
// ============================================================================

/// Duration in seconds for token validity (12 hours)
const TOKEN_EXPIRY_SECS: u64 = 12 * 60 * 60;

/// Get current timestamp in seconds
/// On SSR (CloudFlare Workers): uses worker::Date::now()
/// On client (browser): uses js_sys::Date::now()
#[cfg(feature = "ssr")]
fn current_time_secs() -> u64 {
    (worker::Date::now().as_millis() / 1000) as u64
}

#[cfg(not(feature = "ssr"))]
fn current_time_secs() -> u64 {
    (js_sys::Date::now() / 1000.0) as u64
}

/// Generate a new random token
fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).expect("Failed to generate random bytes");
    hex::encode(bytes)
}

/// Shared authentication state (provided via context on the server)
/// Contains only the admin PIN - tokens are stored in CloudFlare KV
#[derive(Clone)]
pub struct AuthState {
    pub admin_pin: String,
}

impl AuthState {
    pub fn new(admin_pin: String) -> Self {
        Self { admin_pin }
    }
}

/// Wrapper around CloudFlare KV store that implements Send + Sync
/// Uses send_wrapper internally since CloudFlare Workers are single-threaded WASM
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct SendKvStore(std::sync::Arc<send_wrapper::SendWrapper<worker::kv::KvStore>>);

#[cfg(feature = "ssr")]
impl SendKvStore {
    pub fn new(kv: worker::kv::KvStore) -> Self {
        Self(std::sync::Arc::new(send_wrapper::SendWrapper::new(kv)))
    }

    pub fn inner(&self) -> &worker::kv::KvStore {
        &self.0
    }
}

// ============================================================================
// Server Functions
// ============================================================================

/// Response from login server function
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub expires_at: u64,
}

/// Response from validate server function
#[derive(Clone, Serialize, Deserialize)]
pub struct ValidateResult {
    pub valid: bool,
    pub expires_at: Option<u64>,
}

/// Server function to authenticate with PIN
/// Stores the token in CloudFlare KV with TTL
#[server]
pub async fn admin_login(pin: String) -> Result<LoginResult, ServerFnError> {
    use send_wrapper::SendWrapper;
    
    let auth_state = expect_context::<AuthState>();
    let kv = expect_context::<SendKvStore>();
    
    if auth_state.admin_pin.is_empty() {
        log::warn!("ADMIN_PIN not configured");
        return Err(ServerFnError::new("Invalid PIN"));
    }

    if pin != auth_state.admin_pin {
        return Err(ServerFnError::new("Invalid PIN"));
    }

    let token = generate_token();
    let now = current_time_secs();
    let expires_at = now + TOKEN_EXPIRY_SECS;

    // Store token in KV with expiration
    // Key: "token:{token}", Value: expiration timestamp
    let key = format!("token:{}", token);
    let put_builder = kv.inner()
        .put(&key, expires_at)
        .map_err(|e| ServerFnError::new(format!("KV error: {:?}", e)))?
        .expiration_ttl(TOKEN_EXPIRY_SECS);
    
    // Wrap the future in SendWrapper for single-threaded WASM
    SendWrapper::new(put_builder.execute())
        .await
        .map_err(|e| ServerFnError::new(format!("KV error: {:?}", e)))?;

    log::info!("Admin login successful, token generated");
    Ok(LoginResult { token, expires_at })
}

/// Server function to validate an existing token
/// Checks CloudFlare KV for the token
#[server]
pub async fn admin_validate(token: String) -> Result<ValidateResult, ServerFnError> {
    use send_wrapper::SendWrapper;
    
    let kv = expect_context::<SendKvStore>();
    let now = current_time_secs();
    
    let key = format!("token:{}", token);
    
    // Wrap the future in SendWrapper for single-threaded WASM
    let result = SendWrapper::new(kv.inner().get(&key).text()).await;
    
    match result {
        Ok(Some(expires_at_str)) => {
            if let Ok(expires_at) = expires_at_str.parse::<u64>() {
                if expires_at > now {
                    return Ok(ValidateResult {
                        valid: true,
                        expires_at: Some(expires_at),
                    });
                }
            }
            // Token expired or invalid format
            Ok(ValidateResult {
                valid: false,
                expires_at: None,
            })
        }
        Ok(None) => Ok(ValidateResult {
            valid: false,
            expires_at: None,
        }),
        Err(e) => {
            log::error!("KV error: {:?}", e);
            Ok(ValidateResult {
                valid: false,
                expires_at: None,
            })
        }
    }
}

/// Server function to logout (invalidate token)
/// Deletes the token from CloudFlare KV
#[server]
pub async fn admin_logout(token: String) -> Result<bool, ServerFnError> {
    use send_wrapper::SendWrapper;
    
    let kv = expect_context::<SendKvStore>();
    
    let key = format!("token:{}", token);
    
    // Wrap the future in SendWrapper for single-threaded WASM
    match SendWrapper::new(kv.inner().delete(&key)).await {
        Ok(()) => Ok(true),
        Err(e) => {
            log::error!("KV delete error: {:?}", e);
            Ok(false)
        }
    }
}

// ============================================================================
// Admin Authentication - Client-side State
// ============================================================================

const AUTH_STORAGE_KEY: &str = "admin_auth_token";

#[derive(Clone, Serialize, Deserialize)]
struct AuthToken {
    token: String,
    expires_at: u64,
}

/// Admin authentication state (client-side reactive state)
#[derive(Clone)]
struct AdminAuth {
    is_authenticated: RwSignal<bool>,
    token: RwSignal<Option<String>>,
    show_modal: RwSignal<bool>,
    error_message: RwSignal<Option<String>>,
}

impl AdminAuth {
    fn new() -> Self {
        Self {
            is_authenticated: RwSignal::new(false),
            token: RwSignal::new(None),
            show_modal: RwSignal::new(false),
            error_message: RwSignal::new(None),
        }
    }

    /// Load token from localStorage and validate it
    /// Only runs on client (browser), not on SSR (CloudFlare Workers)
    #[cfg(not(feature = "ssr"))]
    fn init(&self) {
        use gloo_storage::{LocalStorage, Storage};

        if let Ok(stored) = LocalStorage::get::<AuthToken>(AUTH_STORAGE_KEY) {
            // Check if token is expired locally first
            let now = current_time_secs();
            if stored.expires_at > now {
                self.token.set(Some(stored.token.clone()));
                // Validate with server asynchronously
                let auth = self.clone();
                let token = stored.token;
                wasm_bindgen_futures::spawn_local(async move {
                    auth.validate_token(&token).await;
                });
            } else {
                // Token expired, clear it
                let _ = LocalStorage::delete(AUTH_STORAGE_KEY);
            }
        }
    }

    #[cfg(feature = "ssr")]
    fn init(&self) {
        // SSR: no localStorage access
    }

    /// Validate token with the server using server function
    /// Only runs on client (browser), not on SSR (CloudFlare Workers)
    #[cfg(not(feature = "ssr"))]
    async fn validate_token(&self, token: &str) {
        use gloo_storage::{LocalStorage, Storage};

        match admin_validate(token.to_string()).await {
            Ok(result) => {
                if result.valid {
                    self.is_authenticated.set(true);
                    self.token.set(Some(token.to_string()));
                } else {
                    // Token invalid, clear everything
                    self.is_authenticated.set(false);
                    self.token.set(None);
                    let _ = LocalStorage::delete(AUTH_STORAGE_KEY);
                }
            }
            Err(e) => {
                log::error!("Failed to validate token: {:?}", e);
            }
        }
    }

    /// Attempt login with PIN using server function
    /// Only runs on client (browser), not on SSR (CloudFlare Workers)
    #[cfg(not(feature = "ssr"))]
    async fn login(&self, pin: &str) {
        use gloo_storage::{LocalStorage, Storage};

        self.error_message.set(None);

        match admin_login(pin.to_string()).await {
            Ok(result) => {
                // Store token in localStorage
                let auth_token = AuthToken {
                    token: result.token.clone(),
                    expires_at: result.expires_at,
                };
                let _ = LocalStorage::set(AUTH_STORAGE_KEY, &auth_token);

                self.token.set(Some(result.token));
                self.is_authenticated.set(true);
                self.show_modal.set(false);
            }
            Err(e) => {
                log::error!("Login failed: {:?}", e);
                self.error_message.set(Some("Invalid PIN".to_string()));
            }
        }
    }

    #[cfg(feature = "ssr")]
    async fn login(&self, _pin: &str) {
        // SSR: no-op
    }

    /// Logout and invalidate token using server function
    /// Only runs on client (browser), not on SSR (CloudFlare Workers)
    #[cfg(not(feature = "ssr"))]
    async fn logout(&self) {
        use gloo_storage::{LocalStorage, Storage};

        if let Some(token) = self.token.get() {
            let _ = admin_logout(token).await;
        }

        // Clear local state regardless of server response
        self.is_authenticated.set(false);
        self.token.set(None);
        let _ = LocalStorage::delete(AUTH_STORAGE_KEY);
    }

    #[cfg(feature = "ssr")]
    async fn logout(&self) {
        // SSR: no-op
    }

    fn open_modal(&self) {
        self.error_message.set(None);
        self.show_modal.set(true);
    }

    fn close_modal(&self) {
        self.show_modal.set(false);
        self.error_message.set(None);
    }
}



/// PIN Modal Content Component
#[component]
fn PinModalContent(
    pin_digits: [RwSignal<String>; 4],
    clear_pin: impl Fn() + Clone + 'static,
) -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let do_close = {
        let auth = auth.clone();
        let clear_pin = clear_pin.clone();
        move || {
            auth.close_modal();
            clear_pin();
        }
    };

    let do_submit = {
        let auth = auth.clone();
        let clear_pin = clear_pin.clone();
        move || {
            let pin: String = pin_digits.iter().map(|d| d.get()).collect();
            if pin.len() == 4 && pin.chars().all(|c| c.is_ascii_digit()) {
                let auth = auth.clone();
                let clear_pin = clear_pin.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    auth.login(&pin).await;
                    // Clear PIN after attempt (success or failure)
                    clear_pin();
                });
            } else {
                auth.error_message.set(Some("Please enter a 4-digit PIN".to_string()));
                clear_pin();
            }
        }
    };

    // Focus the first input when the modal opens
    Effect::new(move || {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id("pin-digit-0") {
                    if let Some(input) = element.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = input.focus();
                    }
                }
            }
        }
    });

    view! {
        <div
            id="pin-modal-backdrop"
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
            on:click={
                let do_close = do_close.clone();
                move |ev: web_sys::MouseEvent| {
                    if let Some(target) = ev.target() {
                        if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                            if element.id() == "pin-modal-backdrop" {
                                do_close();
                            }
                        }
                    }
                }
            }
        >
            <div class="w-full max-w-sm rounded-lg bg-white p-6 shadow-xl">
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="text-xl font-bold text-slate-900">"Admin Access"</h2>
                    <button
                        class="text-slate-500 hover:text-slate-700"
                        on:click={
                            let do_close = do_close.clone();
                            move |_| do_close()
                        }
                    >
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                <p class="mb-4 text-sm text-slate-600">"Enter your 4-digit admin PIN:"</p>

                <div class="mb-4 flex justify-center gap-3">
                    {pin_digits
                        .into_iter()
                        .enumerate()
                        .map(|(i, digit)| {
                            view! {
                                <input
                                    id=format!("pin-digit-{}", i)
                                    type="password"
                                    maxlength="1"
                                    inputmode="numeric"
                                    pattern="[0-9]*"
                                    class="h-14 w-12 rounded border border-slate-300 text-center text-2xl focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    prop:value=move || digit.get()
                                    on:input={
                                        let do_submit = do_submit.clone();
                                        move |ev| {
                                            let value: String = event_target_value(&ev)
                                                .chars()
                                                .filter(|c| c.is_ascii_digit())
                                                .take(1)
                                                .collect();
                                            digit.set(value.clone());

                                            // Auto-advance to next field or submit if complete
                                            if !value.is_empty() {
                                                if i < 3 {
                                                    // Focus next input
                                                    if let Some(window) = web_sys::window() {
                                                        if let Some(document) = window.document() {
                                                            if let Some(element) = document.get_element_by_id(&format!("pin-digit-{}", i + 1)) {
                                                                if let Some(input) = element.dyn_ref::<web_sys::HtmlElement>() {
                                                                    let _ = input.focus();
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    // Last digit entered, auto-submit
                                                    do_submit();
                                                }
                                            }
                                        }
                                    }
                                    on:keydown={
                                        move |ev: web_sys::KeyboardEvent| {
                                            // Handle backspace to go to previous field
                                            if ev.key() == "Backspace" && digit.get().is_empty() && i > 0 {
                                                if let Some(window) = web_sys::window() {
                                                    if let Some(document) = window.document() {
                                                        if let Some(element) = document.get_element_by_id(&format!("pin-digit-{}", i - 1)) {
                                                            if let Some(input) = element.dyn_ref::<web_sys::HtmlElement>() {
                                                                let _ = input.focus();
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                />
                            }
                        })
                        .collect_view()}
                </div>

                <Show when=move || auth.error_message.get().is_some()>
                    <p class="mb-4 text-sm text-red-600">
                        {move || auth.error_message.get().unwrap_or_default()}
                    </p>
                </Show>

                <div class="flex gap-3">
                    <button
                        class="flex-1 rounded bg-slate-200 px-4 py-2 font-medium text-slate-700 hover:bg-slate-300"
                        on:click={
                            let do_close = do_close.clone();
                            move |_| do_close()
                        }
                    >
                        "Cancel"
                    </button>
                    <button
                        class="flex-1 rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300"
                        on:click={
                            let do_submit = do_submit.clone();
                            move |_| do_submit()
                        }
                        disabled=move || {
                            pin_digits.iter().map(|d| d.get()).collect::<String>().len() != 4
                        }
                    >
                        "Unlock"
                    </button>
                </div>
            </div>
        </div>
    }
}

/// PIN Input Modal Component
#[component]
fn PinModal() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();
    let pin_digits: [RwSignal<String>; 4] = [
        RwSignal::new(String::new()),
        RwSignal::new(String::new()),
        RwSignal::new(String::new()),
        RwSignal::new(String::new()),
    ];

    let clear_pin = {
        let pin_digits = pin_digits;
        move || {
            for digit in pin_digits.iter() {
                digit.set(String::new());
            }
            // Refocus the first input after clearing
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id("pin-digit-0") {
                        if let Some(input) = element.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = input.focus();
                        }
                    }
                }
            }
        }
    };

    view! {
        <Show when=move || auth.show_modal.get()>
            <PinModalContent pin_digits=pin_digits clear_pin=clear_pin />
        </Show>
    }
}

/// Unlock Button Component
#[component]
fn UnlockButton() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    view! {
        <button
            class="flex items-center gap-2 rounded bg-slate-700 px-3 py-2 text-sm font-medium hover:bg-slate-600"
            on:click=move |_| auth.open_modal()
        >
            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
            </svg>
            "Unlock"
        </button>
    }
}

/// Logout Button Component
#[component]
fn LogoutButton() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    view! {
        <button
            class="flex items-center gap-2 rounded bg-green-700 px-3 py-2 text-sm font-medium hover:bg-green-600"
            on:click=move |_| {
                let auth = auth.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    auth.logout().await;
                });
            }
        >
            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"/>
            </svg>
            "Logout"
        </button>
    }
}

/// Admin Auth Button (Unlock/Logout)
#[component]
fn AdminAuthButton() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    view! {
        <Show
            when=move || auth.is_authenticated.get()
            fallback=|| view! { <UnlockButton /> }
        >
            <LogoutButton />
        </Show>
    }
}

// ============================================================================
// dyn_ref import for web_sys
// ============================================================================
use wasm_bindgen::JsCast;

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <nav class="bg-slate-800 text-white shadow-md">
            <div class="mx-auto max-w-7xl px-4">
                <div class="flex h-16 items-center justify-between">
                    <div class="flex items-center space-x-8">
                        <h1 class="text-xl font-bold">"food.lemmih.com"</h1>
                        <div class="flex space-x-4">
                            <A href="/" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Food Log"</A>
                            <A href="/ingredients" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Ingredients"</A>
                            <A href="/recipes" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Recipes"</A>
                            <A href="/settings" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Settings"</A>
                        </div>
                    </div>
                    <AdminAuthButton />
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Food Log"</h2>
            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-orange-400 to-pink-400 flex items-center justify-center">
                        <span class="text-6xl">"🍕"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Margherita Pizza"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-01"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Dinner"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"750 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Delicious homemade pizza with fresh mozzarella and basil. Perfect crispy crust!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-green-400 to-emerald-500 flex items-center justify-center">
                        <span class="text-6xl">"🥗"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Caesar Salad"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-02"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Lunch"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"350 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Fresh romaine with grilled chicken, parmesan, and homemade dressing. Very satisfying!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐"</span>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-red-400 to-rose-500 flex items-center justify-center">
                        <span class="text-6xl">"🍝"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Spaghetti Carbonara"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-03"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Dinner"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"650 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Classic Italian pasta with eggs, pecorino, and guanciale. Rich and creamy!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Ingredient category for organizing into separate tables
#[derive(Clone, Copy, PartialEq, Eq)]
enum IngredientCategory {
    Protein,
    Carbs,
    Veggies,
    Other,
}

impl IngredientCategory {
    fn title(&self) -> &'static str {
        match self {
            IngredientCategory::Protein => "Proteins",
            IngredientCategory::Carbs => "Carbs",
            IngredientCategory::Veggies => "Vegetables",
            IngredientCategory::Other => "Other",
        }
    }
}

/// All nutrient values are per 100g
#[derive(Clone, Copy)]
struct Ingredient {
    name: &'static str,
    category: IngredientCategory,
    // Nutrients per 100g
    calories: f32,      // kcal
    protein: f32,       // g
    fat: f32,           // g
    saturated_fat: f32, // g
    carbs: f32,         // g
    sugar: f32,         // g
    fiber: f32,         // g
    salt: f32,          // mg
    // Package info
    package_size_g: f32, // grams
    package_price: f32,  // price in local currency
    store: &'static str,
}

impl Ingredient {
    /// Get nutrient value per 100 kcal
    fn per_calorie(&self, value_per_100g: f32) -> f32 {
        if self.calories > 0.0 {
            (value_per_100g / self.calories) * 100.0
        } else {
            0.0
        }
    }
}

/// Which column to sort by
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SortColumn {
    #[default]
    Name,
    Calories,
    Protein,
    Fat,
    SaturatedFat,
    Carbs,
    Sugar,
    Fiber,
    Salt,
    PackageSize,
    Price,
}

/// Sort direction
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    fn next(self) -> Self {
        match self {
            SortDirection::None => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::None,
        }
    }

    fn indicator(&self) -> &'static str {
        match self {
            SortDirection::None => "",
            SortDirection::Ascending => " \u{25B2}",  // ▲
            SortDirection::Descending => " \u{25BC}", // ▼
        }
    }
}

/// Whether to show nutrients per 100g or per 100kcal
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum NutrientView {
    #[default]
    Per100g,
    Per100kcal,
}

fn get_ingredients() -> Vec<Ingredient> {
    vec![
        // Proteins
        Ingredient {
            name: "Chicken Breast",
            category: IngredientCategory::Protein,
            calories: 165.0,
            protein: 31.0,
            fat: 3.6,
            saturated_fat: 1.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 74.0,
            package_size_g: 500.0,
            package_price: 8.99,
            store: "Whole Foods",
        },
        Ingredient {
            name: "Salmon",
            category: IngredientCategory::Protein,
            calories: 208.0,
            protein: 20.0,
            fat: 13.0,
            saturated_fat: 3.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 59.0,
            package_size_g: 400.0,
            package_price: 12.99,
            store: "Costco",
        },
        Ingredient {
            name: "Eggs (dozen)",
            category: IngredientCategory::Protein,
            calories: 155.0,
            protein: 13.0,
            fat: 11.0,
            saturated_fat: 3.3,
            carbs: 1.1,
            sugar: 1.1,
            fiber: 0.0,
            salt: 124.0,
            package_size_g: 720.0, // ~60g per egg x 12
            package_price: 4.99,
            store: "All stores",
        },
        Ingredient {
            name: "Ground Beef (lean)",
            category: IngredientCategory::Protein,
            calories: 250.0,
            protein: 26.0,
            fat: 15.0,
            saturated_fat: 6.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 75.0,
            package_size_g: 500.0,
            package_price: 7.99,
            store: "Safeway",
        },
        Ingredient {
            name: "Tofu (firm)",
            category: IngredientCategory::Protein,
            calories: 144.0,
            protein: 17.0,
            fat: 9.0,
            saturated_fat: 1.3,
            carbs: 3.0,
            sugar: 1.0,
            fiber: 2.0,
            salt: 14.0,
            package_size_g: 400.0,
            package_price: 2.99,
            store: "Trader Joe's",
        },
        // Carbs
        Ingredient {
            name: "Brown Rice",
            category: IngredientCategory::Carbs,
            calories: 112.0,
            protein: 2.6,
            fat: 0.9,
            saturated_fat: 0.2,
            carbs: 24.0,
            sugar: 0.4,
            fiber: 1.8,
            salt: 1.0,
            package_size_g: 907.0, // 2lb
            package_price: 3.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Pasta (whole wheat)",
            category: IngredientCategory::Carbs,
            calories: 124.0,
            protein: 5.3,
            fat: 0.5,
            saturated_fat: 0.1,
            carbs: 25.0,
            sugar: 0.6,
            fiber: 4.5,
            salt: 4.0,
            package_size_g: 454.0, // 1lb
            package_price: 2.49,
            store: "Safeway",
        },
        Ingredient {
            name: "Oats (rolled)",
            category: IngredientCategory::Carbs,
            calories: 389.0,
            protein: 16.9,
            fat: 6.9,
            saturated_fat: 1.2,
            carbs: 66.0,
            sugar: 0.0,
            fiber: 10.6,
            salt: 2.0,
            package_size_g: 510.0,
            package_price: 4.49,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Quinoa",
            category: IngredientCategory::Carbs,
            calories: 120.0,
            protein: 4.4,
            fat: 1.9,
            saturated_fat: 0.2,
            carbs: 21.0,
            sugar: 0.9,
            fiber: 2.8,
            salt: 7.0,
            package_size_g: 340.0,
            package_price: 5.99,
            store: "Whole Foods",
        },
        Ingredient {
            name: "Bread (whole grain)",
            category: IngredientCategory::Carbs,
            calories: 247.0,
            protein: 13.0,
            fat: 4.2,
            saturated_fat: 0.8,
            carbs: 41.0,
            sugar: 6.0,
            fiber: 7.0,
            salt: 450.0,
            package_size_g: 680.0,
            package_price: 4.99,
            store: "Safeway",
        },
        // Vegetables
        Ingredient {
            name: "Broccoli",
            category: IngredientCategory::Veggies,
            calories: 34.0,
            protein: 2.8,
            fat: 0.4,
            saturated_fat: 0.0,
            carbs: 7.0,
            sugar: 1.7,
            fiber: 2.6,
            salt: 33.0,
            package_size_g: 350.0,
            package_price: 2.49,
            store: "Safeway",
        },
        Ingredient {
            name: "Spinach (fresh)",
            category: IngredientCategory::Veggies,
            calories: 23.0,
            protein: 2.9,
            fat: 0.4,
            saturated_fat: 0.0,
            carbs: 3.6,
            sugar: 0.4,
            fiber: 2.2,
            salt: 79.0,
            package_size_g: 142.0, // 5oz bag
            package_price: 3.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Bell Peppers",
            category: IngredientCategory::Veggies,
            calories: 31.0,
            protein: 1.0,
            fat: 0.3,
            saturated_fat: 0.0,
            carbs: 6.0,
            sugar: 4.2,
            fiber: 2.1,
            salt: 4.0,
            package_size_g: 300.0, // 3-pack
            package_price: 3.99,
            store: "Whole Foods",
        },
        Ingredient {
            name: "Carrots",
            category: IngredientCategory::Veggies,
            calories: 41.0,
            protein: 0.9,
            fat: 0.2,
            saturated_fat: 0.0,
            carbs: 10.0,
            sugar: 4.7,
            fiber: 2.8,
            salt: 69.0,
            package_size_g: 454.0, // 1lb bag
            package_price: 1.99,
            store: "All stores",
        },
        Ingredient {
            name: "Tomatoes (canned)",
            category: IngredientCategory::Veggies,
            calories: 18.0,
            protein: 0.9,
            fat: 0.1,
            saturated_fat: 0.0,
            carbs: 4.0,
            sugar: 2.6,
            fiber: 1.0,
            salt: 9.0,
            package_size_g: 400.0,
            package_price: 1.49,
            store: "All stores",
        },
        // Other
        Ingredient {
            name: "Olive Oil",
            category: IngredientCategory::Other,
            calories: 884.0,
            protein: 0.0,
            fat: 100.0,
            saturated_fat: 14.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 2.0,
            package_size_g: 500.0, // 500ml bottle
            package_price: 9.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Butter",
            category: IngredientCategory::Other,
            calories: 717.0,
            protein: 0.9,
            fat: 81.0,
            saturated_fat: 51.0,
            carbs: 0.1,
            sugar: 0.1,
            fiber: 0.0,
            salt: 714.0,
            package_size_g: 227.0, // 1/2 lb
            package_price: 4.99,
            store: "All stores",
        },
        Ingredient {
            name: "Greek Yogurt",
            category: IngredientCategory::Other,
            calories: 59.0,
            protein: 10.0,
            fat: 0.7,
            saturated_fat: 0.1,
            carbs: 3.6,
            sugar: 3.2,
            fiber: 0.0,
            salt: 36.0,
            package_size_g: 450.0,
            package_price: 5.49,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Cheese (cheddar)",
            category: IngredientCategory::Other,
            calories: 403.0,
            protein: 25.0,
            fat: 33.0,
            saturated_fat: 21.0,
            carbs: 1.3,
            sugar: 0.5,
            fiber: 0.0,
            salt: 621.0,
            package_size_g: 227.0,
            package_price: 5.99,
            store: "Costco",
        },
        Ingredient {
            name: "Honey",
            category: IngredientCategory::Other,
            calories: 304.0,
            protein: 0.3,
            fat: 0.0,
            saturated_fat: 0.0,
            carbs: 82.0,
            sugar: 82.0,
            fiber: 0.0,
            salt: 4.0,
            package_size_g: 340.0,
            package_price: 7.99,
            store: "Whole Foods",
        },
    ]
}

#[component]
fn SortableHeader(
    col: SortColumn,
    label: &'static str,
    width_class: &'static str,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    on_click: impl Fn(SortColumn) + 'static,
) -> impl IntoView {
    view! {
        <th
            class=format!("px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider cursor-pointer hover:bg-slate-100 select-none {}", width_class)
            on:click=move |_| on_click(col)
        >
            <span class="inline-flex items-center gap-1">
                {label}
                <span class="w-3 inline-block text-center">
                    {move || {
                        if sort_column.get() == col {
                            sort_direction.get().indicator()
                        } else {
                            ""
                        }
                    }}
                </span>
            </span>
        </th>
    }
}

#[component]
fn IngredientTable(
    title: &'static str,
    category: IngredientCategory,
    view_mode: ReadSignal<NutrientView>,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    on_header_click: impl Fn(SortColumn) + Clone + 'static,
) -> impl IntoView {
    let get_sorted_ingredients = move || {
        let mut ingredients: Vec<Ingredient> = get_ingredients()
            .into_iter()
            .filter(|i| i.category == category)
            .collect();

        let dir = sort_direction.get();
        let col = sort_column.get();
        let view = view_mode.get();

        if dir != SortDirection::None {
            ingredients.sort_by(|a, b| {
                let get_value = |ing: &Ingredient| -> f32 {
                    let raw = match col {
                        SortColumn::Name => return 0.0, // Handle separately
                        SortColumn::Calories => ing.calories,
                        SortColumn::Protein => ing.protein,
                        SortColumn::Fat => ing.fat,
                        SortColumn::SaturatedFat => ing.saturated_fat,
                        SortColumn::Carbs => ing.carbs,
                        SortColumn::Sugar => ing.sugar,
                        SortColumn::Fiber => ing.fiber,
                        SortColumn::Salt => ing.salt,
                        SortColumn::PackageSize => ing.package_size_g,
                        SortColumn::Price => ing.package_price,
                    };
                    if view == NutrientView::Per100kcal
                        && !matches!(
                            col,
                            SortColumn::PackageSize | SortColumn::Price | SortColumn::Calories
                        )
                    {
                        ing.per_calorie(raw)
                    } else {
                        raw
                    }
                };

                if col == SortColumn::Name {
                    let cmp = a.name.cmp(b.name);
                    return if dir == SortDirection::Ascending {
                        cmp
                    } else {
                        cmp.reverse()
                    };
                }

                let val_a = get_value(a);
                let val_b = get_value(b);
                let cmp = val_a
                    .partial_cmp(&val_b)
                    .unwrap_or(std::cmp::Ordering::Equal);
                if dir == SortDirection::Ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            });
        } else {
            // Default: sort by name ascending
            ingredients.sort_by(|a, b| a.name.cmp(b.name));
        }

        ingredients
    };

    // Column width classes
    let w_name = "w-36"; // Ingredient name
    let w_pkg = "w-20"; // Package size
    let w_price = "w-16"; // Price
    let w_cal = "w-20"; // Calories
    let w_nutr = "w-16"; // Nutrient columns (protein, fat, etc.)
    let w_salt = "w-20"; // Salt (needs more space for mg)
    let w_store = "w-28"; // Store

    let cell_class = "px-3 py-3 whitespace-nowrap text-slate-700";

    view! {
        <div class="mb-8">
            <h3 class="mb-3 text-xl font-semibold text-slate-800">{title}</h3>
            <div class="rounded-lg bg-white shadow-md overflow-hidden overflow-x-auto">
                <table class="w-full table-fixed divide-y divide-slate-200 text-sm">
                    <thead class="bg-slate-50">
                        <tr>
                            <SortableHeader col=SortColumn::Name label="Ingredient" width_class=w_name sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::PackageSize label="Package" width_class=w_pkg sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Price label="Price" width_class=w_price sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Calories label="Calories" width_class=w_cal sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Protein label="Protein" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Fat label="Fat" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::SaturatedFat label="Sat. Fat" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Carbs label="Carbs" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Sugar label="Sugar" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Fiber label="Fiber" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Salt label="Salt" width_class=w_salt sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <th class=format!("px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider {}", w_store)>"Store"</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <For
                            each=get_sorted_ingredients
                            key=|ing| ing.name
                            let:ing
                        >
                            {
                                let ing_clone = ing.clone();
                                view! {
                                    <tr class="hover:bg-slate-50">
                                        <td class=format!("{} font-medium text-slate-900 truncate", cell_class)>{ing.name}</td>
                                        <td class=cell_class>{format!("{}g", ing.package_size_g)}</td>
                                        <td class=cell_class>{format!("${:.2}", ing.package_price)}</td>
                                        <td class=cell_class>
                                            {move || {
                                                let cal = if view_mode.get() == NutrientView::Per100kcal { 100.0 } else { ing_clone.calories };
                                                format!("{:.0} kcal", cal)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.protein) } else { ing_clone.protein };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.fat) } else { ing_clone.fat };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.saturated_fat) } else { ing_clone.saturated_fat };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.carbs) } else { ing_clone.carbs };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.sugar) } else { ing_clone.sugar };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.fiber) } else { ing_clone.fiber };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_clone.per_calorie(ing_clone.salt) } else { ing_clone.salt };
                                                format!("{:.0}mg", val)
                                            }}
                                        </td>
                                        <td class=format!("{} truncate", cell_class)>{ing.store}</td>
                                    </tr>
                                }
                            }
                        </For>
                    </tbody>
                </table>
            </div>
            <p class="mt-2 text-xs text-slate-500">
                {move || {
                    let suffix = if view_mode.get() == NutrientView::Per100kcal { "/100kcal" } else { "/100g" };
                    format!("* Nutrient values shown {}", suffix)
                }}
            </p>
        </div>
    }
}

#[component]
fn Ingredients() -> impl IntoView {
    let (view_mode, set_view_mode) = signal(NutrientView::Per100g);
    let (sort_column, set_sort_column) = signal(SortColumn::Name);
    let (sort_direction, set_sort_direction) = signal(SortDirection::None);

    let handle_header_click = move |col: SortColumn| {
        if sort_column.get() == col {
            set_sort_direction.set(sort_direction.get().next());
        } else {
            set_sort_column.set(col);
            set_sort_direction.set(SortDirection::Descending);
        }
    };

    view! {
        <div class="mx-auto max-w-7xl py-6">
            <div class="mb-6 flex items-center justify-between">
                <h2 class="text-3xl font-bold text-slate-900">"Ingredient List"</h2>
                <div class="flex items-center gap-3 bg-white rounded-lg px-4 py-2 shadow-sm">
                    <span class="text-sm font-medium text-slate-700">"View nutrients:"</span>
                    <button
                        class=move || {
                            let base = "px-3 py-1 text-sm font-medium rounded transition-colors";
                            if view_mode.get() == NutrientView::Per100g {
                                format!("{} bg-blue-600 text-white", base)
                            } else {
                                format!("{} bg-slate-100 text-slate-700 hover:bg-slate-200", base)
                            }
                        }
                        on:click=move |_| set_view_mode.set(NutrientView::Per100g)
                    >
                        "per 100g"
                    </button>
                    <button
                        class=move || {
                            let base = "px-3 py-1 text-sm font-medium rounded transition-colors";
                            if view_mode.get() == NutrientView::Per100kcal {
                                format!("{} bg-blue-600 text-white", base)
                            } else {
                                format!("{} bg-slate-100 text-slate-700 hover:bg-slate-200", base)
                            }
                        }
                        on:click=move |_| set_view_mode.set(NutrientView::Per100kcal)
                    >
                        "per 100kcal"
                    </button>
                </div>
            </div>

            <IngredientTable
                title=IngredientCategory::Protein.title()
                category=IngredientCategory::Protein
                view_mode=view_mode
                sort_column=sort_column
                sort_direction=sort_direction
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Carbs.title()
                category=IngredientCategory::Carbs
                view_mode=view_mode
                sort_column=sort_column
                sort_direction=sort_direction
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Veggies.title()
                category=IngredientCategory::Veggies
                view_mode=view_mode
                sort_column=sort_column
                sort_direction=sort_direction
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Other.title()
                category=IngredientCategory::Other
                view_mode=view_mode
                sort_column=sort_column
                sort_direction=sort_direction
                on_header_click=handle_header_click
            />
        </div>
    }
}

// Recipe data structures
#[derive(Clone, PartialEq)]
struct RecipeNutrition {
    calories: i32,
    protein: i32,
    carbs: i32,
    fat: i32,
    sat_fat: f32,
    salt: f32,
    fiber: i32,
}

#[derive(Clone, PartialEq)]
struct Recipe {
    name: &'static str,
    meal_type: &'static str,
    tags: &'static [&'static str],
    prep_time: &'static str,
    cook_time: &'static str,
    servings: i32,
    ingredients: &'static [&'static str],
    instructions: &'static [&'static str],
    nutrition: RecipeNutrition,
}

fn get_meal_type_color(meal_type: &str) -> (&'static str, &'static str) {
    match meal_type {
        "Breakfast" => ("bg-yellow-100", "text-yellow-800"),
        "Lunch" => ("bg-orange-100", "text-orange-800"),
        "Dinner" => ("bg-indigo-100", "text-indigo-800"),
        "Sandwich" => ("bg-amber-100", "text-amber-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

fn get_tag_color(tag: &str) -> (&'static str, &'static str) {
    match tag {
        "High Protein" => ("bg-blue-100", "text-blue-800"),
        "Low Carb" => ("bg-purple-100", "text-purple-800"),
        "Heart Healthy" | "Omega-3" => ("bg-blue-100", "text-blue-800"),
        "Mediterranean" | "Healthy" => ("bg-green-100", "text-green-800"),
        "Plant-Based" | "Vegan" => ("bg-green-100", "text-green-800"),
        "Quick" => ("bg-yellow-100", "text-yellow-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

const EXAMPLE_RECIPES: &[Recipe] = &[
    Recipe {
        name: "Grilled Chicken Bowl",
        meal_type: "Lunch",
        tags: &["High Protein", "Low Carb", "Healthy"],
        prep_time: "15 min",
        cook_time: "20 min",
        servings: 2,
        ingredients: &[
            "300g chicken breast",
            "200g brown rice",
            "150g broccoli",
            "1 tbsp olive oil",
            "Salt and pepper to taste",
        ],
        instructions: &[
            "Season chicken with salt and pepper",
            "Grill chicken for 6-8 minutes per side",
            "Cook rice according to package directions",
            "Steam broccoli for 5 minutes",
            "Assemble bowl and drizzle with olive oil",
        ],
        nutrition: RecipeNutrition {
            calories: 520,
            protein: 45,
            carbs: 50,
            fat: 12,
            sat_fat: 2.5,
            salt: 1.2,
            fiber: 6,
        },
    },
    Recipe {
        name: "Salmon with Vegetables",
        meal_type: "Dinner",
        tags: &["Omega-3", "Heart Healthy", "Mediterranean"],
        prep_time: "10 min",
        cook_time: "25 min",
        servings: 2,
        ingredients: &[
            "400g salmon fillet",
            "200g broccoli",
            "1 lemon",
            "2 tbsp olive oil",
            "Garlic, herbs, salt, pepper",
        ],
        instructions: &[
            "Preheat oven to 400°F (200°C)",
            "Place salmon on baking sheet",
            "Drizzle with olive oil and lemon juice",
            "Add broccoli around salmon",
            "Bake for 15-20 minutes",
        ],
        nutrition: RecipeNutrition {
            calories: 480,
            protein: 40,
            carbs: 10,
            fat: 30,
            sat_fat: 5.0,
            salt: 0.8,
            fiber: 4,
        },
    },
    Recipe {
        name: "Veggie Stir Fry",
        meal_type: "Dinner",
        tags: &["Vegan", "Plant-Based", "Quick"],
        prep_time: "10 min",
        cook_time: "15 min",
        servings: 2,
        ingredients: &[
            "200g broccoli",
            "150g bell peppers",
            "100g snap peas",
            "200g brown rice",
            "2 tbsp soy sauce",
            "1 tbsp sesame oil",
        ],
        instructions: &[
            "Cook rice according to package directions",
            "Heat sesame oil in wok",
            "Add vegetables and stir-fry for 5-7 minutes",
            "Add soy sauce and cook 2 more minutes",
            "Serve over rice",
        ],
        nutrition: RecipeNutrition {
            calories: 380,
            protein: 8,
            carbs: 65,
            fat: 9,
            sat_fat: 1.5,
            salt: 2.0,
            fiber: 8,
        },
    },
];

#[component]
fn RecipeCard(
    recipe: Recipe,
    active_filters: ReadSignal<Vec<String>>,
    on_tag_click: Callback<String>,
) -> impl IntoView {
    let (meal_bg, meal_text) = get_meal_type_color(recipe.meal_type);
    let meal_type = recipe.meal_type.to_string();

    view! {
        <div class="rounded-lg bg-white p-6 shadow-md">
            <div class="mb-4 flex items-center justify-between">
                <h3 class="text-2xl font-bold text-slate-900">{recipe.name}</h3>
                <button
                    class=format!("rounded px-3 py-1 text-sm font-medium cursor-pointer hover:opacity-80 {} {}", meal_bg, meal_text)
                    on:click={
                        let meal_type = meal_type.clone();
                        move |_| on_tag_click.run(meal_type.clone())
                    }
                >
                    {recipe.meal_type}
                </button>
            </div>
            <div class="mb-4">
                <p class="mb-2 text-sm text-slate-600">
                    {format!("Prep: {} | Cook: {} | Servings: {}", recipe.prep_time, recipe.cook_time, recipe.servings)}
                </p>
                <div class="flex flex-wrap gap-2">
                    {recipe.tags.iter().map(|tag| {
                        let (bg, text) = get_tag_color(tag);
                        let tag_str = tag.to_string();
                        let is_active = {
                            let tag_str = tag_str.clone();
                            move || active_filters.get().contains(&tag_str)
                        };
                        view! {
                            <button
                                class=move || format!(
                                    "rounded px-2 py-1 text-xs cursor-pointer hover:opacity-80 {} {} {}",
                                    bg,
                                    text,
                                    if is_active() { "ring-2 ring-offset-1 ring-slate-400" } else { "" }
                                )
                                on:click={
                                    let tag_str = tag_str.clone();
                                    move |_| on_tag_click.run(tag_str.clone())
                                }
                            >
                                {*tag}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            <h4 class="mb-2 font-semibold text-slate-900">"Ingredients:"</h4>
            <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
                {recipe.ingredients.iter().map(|ingredient| view! { <li>{*ingredient}</li> }).collect_view()}
            </ul>
            <h4 class="mb-2 font-semibold text-slate-900">"Instructions:"</h4>
            <ol class="list-inside list-decimal space-y-1 text-slate-700">
                {recipe.instructions.iter().map(|instruction| view! { <li>{*instruction}</li> }).collect_view()}
            </ol>
            <div class="mt-4 rounded bg-slate-50 p-3">
                <p class="text-sm font-medium text-slate-900">
                    {format!(
                        "Nutrition per serving: {} kcal | {}g protein | {}g carbs | {}g fat",
                        recipe.nutrition.calories,
                        recipe.nutrition.protein,
                        recipe.nutrition.carbs,
                        recipe.nutrition.fat
                    )}
                </p>
                <p class="text-sm text-slate-600 mt-1">
                    {format!(
                        "Sat. fat: {}g | Salt: {}g | Fiber: {}g",
                        recipe.nutrition.sat_fat,
                        recipe.nutrition.salt,
                        recipe.nutrition.fiber
                    )}
                </p>
            </div>
        </div>
    }
}

#[component]
fn Recipes() -> impl IntoView {
    let (active_filters, set_active_filters) = signal(Vec::<String>::new());

    let toggle_filter = Callback::new(move |tag: String| {
        set_active_filters.update(|filters| {
            if let Some(pos) = filters.iter().position(|t| t == &tag) {
                filters.remove(pos);
            } else {
                filters.push(tag);
            }
        });
    });

    let filtered_recipes = move || {
        let filters = active_filters.get();
        if filters.is_empty() {
            EXAMPLE_RECIPES.to_vec()
        } else {
            EXAMPLE_RECIPES
                .iter()
                .filter(|recipe| {
                    filters.iter().all(|filter| {
                        recipe.meal_type == filter || recipe.tags.contains(&filter.as_str())
                    })
                })
                .cloned()
                .collect()
        }
    };

    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Recipes"</h2>

            // Active filters display
            <Show when=move || !active_filters.get().is_empty()>
                <div class="mb-4 flex flex-wrap items-center gap-2">
                    <span class="text-sm font-medium text-slate-600">"Active filters:"</span>
                    <For
                        each=move || active_filters.get()
                        key=|tag| tag.clone()
                        children=move |tag: String| {
                            let tag_for_click = tag.clone();
                            let tag_for_display = tag.clone();
                            view! {
                                <button
                                    class="inline-flex items-center gap-1 rounded bg-slate-200 px-2 py-1 text-sm text-slate-700 hover:bg-slate-300 cursor-pointer"
                                    on:click=move |_| toggle_filter.run(tag_for_click.clone())
                                >
                                    {tag_for_display}
                                    <span class="font-bold">"×"</span>
                                </button>
                            }
                        }
                    />
                    <button
                        class="text-sm text-slate-500 hover:text-slate-700 underline cursor-pointer"
                        on:click=move |_| set_active_filters.set(Vec::new())
                    >
                        "Clear all"
                    </button>
                </div>
            </Show>

            <div class="grid gap-6 lg:grid-cols-2">
                <For
                    each=filtered_recipes
                    key=|recipe| recipe.name
                    children=move |recipe: Recipe| {
                        view! {
                            <RecipeCard
                                recipe=recipe
                                active_filters=active_filters
                                on_tag_click=toggle_filter
                            />
                        }
                    }
                />
            </div>

            // Show message when no recipes match
            <Show when=move || filtered_recipes().is_empty()>
                <div class="text-center py-8 text-slate-500">
                    "No recipes match the selected filters."
                </div>
            </Show>
        </div>
    }
}

// Conversion factor: 1g salt = 393.4mg sodium (sodium is ~39.34% of salt by weight)
// So 2300mg sodium ≈ 5.84g salt, and 5g salt ≈ 1967mg sodium

// Calories per gram for macros
const CALORIES_PER_GRAM_PROTEIN: f64 = 4.0;
const CALORIES_PER_GRAM_CARBS: f64 = 4.0;
const CALORIES_PER_GRAM_FAT: f64 = 9.0;

// Macro types for locking
#[derive(Clone, Copy, PartialEq, Eq)]
enum Macro {
    Protein,
    Carbs,
    Fat,
}

#[component]
fn Settings() -> impl IntoView {
    // Daily calorie goal
    let (daily_calories, set_daily_calories) = signal(2000_i32);

    // Macro distribution (must sum to 100)
    let (protein_pct, set_protein_pct) = signal(30_i32);
    let (carbs_pct, set_carbs_pct) = signal(40_i32);
    let (fat_pct, set_fat_pct) = signal(30_i32);

    // Which macro is locked (only one can be locked at a time)
    let (locked_macro, set_locked_macro) = signal(Option::<Macro>::None);

    // Function to adjust macros when one changes, keeping total at 100%
    // The changed macro and any locked macro are preserved; the third adjusts
    let adjust_macros = move |changed: Macro, new_value: i32| {
        let new_value = new_value.clamp(5, 90); // Ensure reasonable bounds
        let locked = locked_macro.get();

        // Unlock the macro being changed
        if locked == Some(changed) {
            set_locked_macro.set(None);
        }
        let locked = if locked == Some(changed) {
            None
        } else {
            locked
        };

        let (current_protein, current_carbs, current_fat) =
            (protein_pct.get(), carbs_pct.get(), fat_pct.get());

        match changed {
            Macro::Protein => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Carbs) => {
                        // Carbs locked, adjust fat
                        let new_fat = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_fat;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        // Fat locked, adjust carbs
                        let new_carbs = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_carbs;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
                        // No lock or protein was locked (now unlocked), distribute to carbs and fat proportionally
                        let old_other_total = current_carbs + current_fat;
                        if old_other_total > 0 {
                            let carbs_ratio = current_carbs as f64 / old_other_total as f64;
                            let new_carbs = (remaining as f64 * carbs_ratio).round() as i32;
                            let new_fat = remaining - new_carbs;
                            set_protein_pct.set(new_value);
                            set_carbs_pct.set(new_carbs.clamp(5, 90));
                            set_fat_pct.set(new_fat.clamp(5, 90));
                        } else {
                            set_protein_pct.set(new_value);
                            set_carbs_pct.set(remaining / 2);
                            set_fat_pct.set(remaining - remaining / 2);
                        }
                    }
                }
            }
            Macro::Carbs => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Protein) => {
                        // Protein locked, adjust fat
                        let new_fat = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_fat;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        // Fat locked, adjust protein
                        let new_protein = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
                        // No lock, distribute to protein and fat proportionally
                        let old_other_total = current_protein + current_fat;
                        if old_other_total > 0 {
                            let protein_ratio = current_protein as f64 / old_other_total as f64;
                            let new_protein = (remaining as f64 * protein_ratio).round() as i32;
                            let new_fat = remaining - new_protein;
                            set_protein_pct.set(new_protein.clamp(5, 90));
                            set_carbs_pct.set(new_value);
                            set_fat_pct.set(new_fat.clamp(5, 90));
                        } else {
                            set_protein_pct.set(remaining / 2);
                            set_carbs_pct.set(new_value);
                            set_fat_pct.set(remaining - remaining / 2);
                        }
                    }
                }
            }
            Macro::Fat => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Protein) => {
                        // Protein locked, adjust carbs
                        let new_carbs = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_carbs;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    Some(Macro::Carbs) => {
                        // Carbs locked, adjust protein
                        let new_protein = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    _ => {
                        // No lock, distribute to protein and carbs proportionally
                        let old_other_total = current_protein + current_carbs;
                        if old_other_total > 0 {
                            let protein_ratio = current_protein as f64 / old_other_total as f64;
                            let new_protein = (remaining as f64 * protein_ratio).round() as i32;
                            let new_carbs = remaining - new_protein;
                            set_protein_pct.set(new_protein.clamp(5, 90));
                            set_carbs_pct.set(new_carbs.clamp(5, 90));
                            set_fat_pct.set(new_value);
                        } else {
                            set_protein_pct.set(remaining / 2);
                            set_carbs_pct.set(remaining - remaining / 2);
                            set_fat_pct.set(new_value);
                        }
                    }
                }
            }
        }
    };

    // Computed grams for each macro
    let protein_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = protein_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_PROTEIN).round() as i32
    });
    let carbs_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = carbs_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_CARBS).round() as i32
    });
    let fat_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = fat_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_FAT).round() as i32
    });

    // Function to update macro from grams input
    let adjust_macros_from_grams = move |changed: Macro, grams: i32| {
        let cals = daily_calories.get() as f64;
        if cals <= 0.0 {
            return;
        }
        let cal_per_gram = match changed {
            Macro::Protein => CALORIES_PER_GRAM_PROTEIN,
            Macro::Carbs => CALORIES_PER_GRAM_CARBS,
            Macro::Fat => CALORIES_PER_GRAM_FAT,
        };
        let new_pct = ((grams as f64 * cal_per_gram / cals) * 100.0).round() as i32;
        adjust_macros(changed, new_pct);
    };

    // Pie chart arc path calculation
    let pie_chart_paths = Memo::new(move |_| {
        let protein = protein_pct.get() as f64;
        let carbs = carbs_pct.get() as f64;
        let fat = fat_pct.get() as f64;

        // Center and radius for the pie chart
        let cx = 60.0_f64;
        let cy = 60.0_f64;
        let r = 50.0_f64;

        // Calculate angles (in radians, starting from top)
        let protein_angle = protein / 100.0 * 2.0 * std::f64::consts::PI;
        let carbs_angle = carbs / 100.0 * 2.0 * std::f64::consts::PI;
        let fat_angle = fat / 100.0 * 2.0 * std::f64::consts::PI;

        // Starting angle (top of circle = -PI/2)
        let start = -std::f64::consts::PI / 2.0;

        // Helper to create arc path
        let arc_path = |start_angle: f64, end_angle: f64| -> String {
            let x1 = cx + r * start_angle.cos();
            let y1 = cy + r * start_angle.sin();
            let x2 = cx + r * end_angle.cos();
            let y2 = cy + r * end_angle.sin();
            let large_arc = if (end_angle - start_angle).abs() > std::f64::consts::PI {
                1
            } else {
                0
            };
            format!("M {cx} {cy} L {x1} {y1} A {r} {r} 0 {large_arc} 1 {x2} {y2} Z")
        };

        let protein_end = start + protein_angle;
        let carbs_end = protein_end + carbs_angle;
        let fat_end = carbs_end + fat_angle;

        (
            arc_path(start, protein_end),
            arc_path(protein_end, carbs_end),
            arc_path(carbs_end, fat_end),
        )
    });

    // Salt/Sodium: store internally as sodium in mg
    let (sodium_mg, set_sodium_mg) = signal(2300_i32);

    // Computed salt in grams (linked to sodium)
    let salt_grams = Memo::new(move |_| {
        // 1g salt = 393.4mg sodium; salt_g = sodium_mg / 393.4
        (sodium_mg.get() as f64 / 393.4 * 10.0).round() / 10.0
    });

    // Saturated fat: store as grams, compute percentage
    let (sat_fat_grams, set_sat_fat_grams) = signal(20_i32);
    let sat_fat_pct = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        if cals <= 0.0 {
            return 0.0;
        }
        let fat_cals = sat_fat_grams.get() as f64 * CALORIES_PER_GRAM_FAT;
        (fat_cals / cals * 100.0 * 10.0).round() / 10.0
    });

    // Fiber minimum
    let (fiber_min, set_fiber_min) = signal(25_i32);

    // Preset loader
    let load_preset = move |preset: &str| {
        match preset {
            "usda" => {
                // dietaryguidelines.gov (USDA) - 2000 cal, 10-35% protein, 45-65% carbs, 20-35% fat
                // Using middle-ground values; sodium 2300mg, sat fat <10%, fiber 28g
                set_daily_calories.set(2000);
                set_protein_pct.set(20);
                set_carbs_pct.set(55);
                set_fat_pct.set(25);
                set_sodium_mg.set(2300);
                set_sat_fat_grams.set(22); // ~10% of 2000 cal
                set_fiber_min.set(28);
            }
            "aha" => {
                // AHA (American Heart Association) - focuses on heart health
                // 2000 cal, lower sat fat (<6%), sodium <2300mg (ideally 1500mg), fiber 25-30g
                set_daily_calories.set(2000);
                set_protein_pct.set(25);
                set_carbs_pct.set(50);
                set_fat_pct.set(25);
                set_sodium_mg.set(1500);
                set_sat_fat_grams.set(13); // ~6% of 2000 cal
                set_fiber_min.set(30);
            }
            "nhs" => {
                // NHS (UK) - 2000 cal for women, 2500 for men; using 2000
                // Sat fat <11%, salt <6g (~2360mg sodium), fiber 30g
                set_daily_calories.set(2000);
                set_protein_pct.set(20);
                set_carbs_pct.set(50);
                set_fat_pct.set(30);
                set_sodium_mg.set(2360);
                set_sat_fat_grams.set(24); // ~11% of 2000 cal
                set_fiber_min.set(30);
            }
            _ => {}
        }
    };

    view! {
        <div class="mx-auto max-w-4xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Settings"</h2>

            // Preset buttons
            <div class="mb-6 rounded-lg bg-white p-6 shadow-md">
                <h3 class="mb-4 text-xl font-semibold text-slate-900">"Load Preset"</h3>
                <div class="flex flex-wrap gap-3">
                    <button
                        class="rounded bg-emerald-600 px-4 py-2 text-sm font-semibold text-white hover:bg-emerald-700"
                        on:click=move |_| load_preset("usda")
                    >
                        "USDA Dietary Guidelines"
                    </button>
                    <button
                        class="rounded bg-red-600 px-4 py-2 text-sm font-semibold text-white hover:bg-red-700"
                        on:click=move |_| load_preset("aha")
                    >
                        "AHA Guidelines"
                    </button>
                    <button
                        class="rounded bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-700"
                        on:click=move |_| load_preset("nhs")
                    >
                        "NHS Guidelines"
                    </button>
                </div>
            </div>

            <div class="space-y-6">
                // Daily Goals
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Goals"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Target Calories per Day"</label>
                            <input
                                type="number"
                                prop:value=move || daily_calories.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        set_daily_calories.set(val.max(0));
                                    }
                                }
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Your target calorie intake per day"</p>
                        </div>
                    </div>
                </div>

                // Macro Distribution
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Macro Distribution"</h3>

                    <div class="flex flex-col md:flex-row gap-6">
                        // Pie Chart
                        <div class="flex-shrink-0 flex flex-col items-center">
                            <svg width="120" height="120" viewBox="0 0 120 120">
                                // Protein slice (blue)
                                <path
                                    d=move || pie_chart_paths.get().0
                                    fill="#2563eb"
                                />
                                // Carbs slice (green)
                                <path
                                    d=move || pie_chart_paths.get().1
                                    fill="#16a34a"
                                />
                                // Fat slice (orange)
                                <path
                                    d=move || pie_chart_paths.get().2
                                    fill="#ea580c"
                                />
                            </svg>
                            // Legend
                            <div class="mt-2 flex gap-3 text-xs">
                                <div class="flex items-center gap-1">
                                    <div class="h-3 w-3 rounded-sm bg-blue-600"></div>
                                    <span>"Protein"</span>
                                </div>
                                <div class="flex items-center gap-1">
                                    <div class="h-3 w-3 rounded-sm bg-green-600"></div>
                                    <span>"Carbs"</span>
                                </div>
                                <div class="flex items-center gap-1">
                                    <div class="h-3 w-3 rounded-sm bg-orange-600"></div>
                                    <span>"Fat"</span>
                                </div>
                            </div>
                        </div>

                        // Macro inputs
                        <div class="flex-1 space-y-3">
                            // Info message
                            <div class="rounded bg-blue-50 px-3 py-2 text-xs text-blue-800">
                                "Lock a macro to prevent it from auto-adjusting when others change"
                            </div>

                            // Protein row
                            <div class="flex items-center gap-2">
                                <button
                                    class="flex h-7 w-7 items-center justify-center rounded border text-sm hover:bg-slate-100"
                                    class=("border-blue-500", move || locked_macro.get() == Some(Macro::Protein))
                                    class=("bg-blue-50", move || locked_macro.get() == Some(Macro::Protein))
                                    class=("border-slate-300", move || locked_macro.get() != Some(Macro::Protein))
                                    title=move || if locked_macro.get() == Some(Macro::Protein) { "Click to unlock" } else { "Click to lock" }
                                    on:click=move |_| {
                                        if locked_macro.get() == Some(Macro::Protein) {
                                            set_locked_macro.set(None);
                                        } else {
                                            set_locked_macro.set(Some(Macro::Protein));
                                        }
                                    }
                                >
                                    {move || if locked_macro.get() == Some(Macro::Protein) { "🔒" } else { "🔓" }}
                                </button>
                                <div class="h-3 w-3 rounded-sm bg-blue-600"></div>
                                <span class="w-24 text-sm font-medium text-slate-700">"Protein"</span>
                                <input
                                    type="number"
                                    min="5"
                                    max="90"
                                    prop:value=move || protein_pct.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros(Macro::Protein, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"%"</span>
                                <input
                                    type="number"
                                    min="0"
                                    prop:value=move || protein_grams.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros_from_grams(Macro::Protein, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"g"</span>
                            </div>

                            // Carbohydrates row
                            <div class="flex items-center gap-2">
                                <button
                                    class="flex h-7 w-7 items-center justify-center rounded border text-sm hover:bg-slate-100"
                                    class=("border-green-500", move || locked_macro.get() == Some(Macro::Carbs))
                                    class=("bg-green-50", move || locked_macro.get() == Some(Macro::Carbs))
                                    class=("border-slate-300", move || locked_macro.get() != Some(Macro::Carbs))
                                    title=move || if locked_macro.get() == Some(Macro::Carbs) { "Click to unlock" } else { "Click to lock" }
                                    on:click=move |_| {
                                        if locked_macro.get() == Some(Macro::Carbs) {
                                            set_locked_macro.set(None);
                                        } else {
                                            set_locked_macro.set(Some(Macro::Carbs));
                                        }
                                    }
                                >
                                    {move || if locked_macro.get() == Some(Macro::Carbs) { "🔒" } else { "🔓" }}
                                </button>
                                <div class="h-3 w-3 rounded-sm bg-green-600"></div>
                                <span class="w-24 text-sm font-medium text-slate-700">"Carbs"</span>
                                <input
                                    type="number"
                                    min="5"
                                    max="90"
                                    prop:value=move || carbs_pct.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros(Macro::Carbs, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"%"</span>
                                <input
                                    type="number"
                                    min="0"
                                    prop:value=move || carbs_grams.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros_from_grams(Macro::Carbs, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"g"</span>
                            </div>

                            // Fat row
                            <div class="flex items-center gap-2">
                                <button
                                    class="flex h-7 w-7 items-center justify-center rounded border text-sm hover:bg-slate-100"
                                    class=("border-orange-500", move || locked_macro.get() == Some(Macro::Fat))
                                    class=("bg-orange-50", move || locked_macro.get() == Some(Macro::Fat))
                                    class=("border-slate-300", move || locked_macro.get() != Some(Macro::Fat))
                                    title=move || if locked_macro.get() == Some(Macro::Fat) { "Click to unlock" } else { "Click to lock" }
                                    on:click=move |_| {
                                        if locked_macro.get() == Some(Macro::Fat) {
                                            set_locked_macro.set(None);
                                        } else {
                                            set_locked_macro.set(Some(Macro::Fat));
                                        }
                                    }
                                >
                                    {move || if locked_macro.get() == Some(Macro::Fat) { "🔒" } else { "🔓" }}
                                </button>
                                <div class="h-3 w-3 rounded-sm bg-orange-600"></div>
                                <span class="w-24 text-sm font-medium text-slate-700">"Fat"</span>
                                <input
                                    type="number"
                                    min="5"
                                    max="90"
                                    prop:value=move || fat_pct.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros(Macro::Fat, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"%"</span>
                                <input
                                    type="number"
                                    min="0"
                                    prop:value=move || fat_grams.get()
                                    on:input=move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                            adjust_macros_from_grams(Macro::Fat, val);
                                        }
                                    }
                                    class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
                                />
                                <span class="text-sm text-slate-500">"g"</span>
                            </div>
                        </div>
                    </div>
                </div>

                // Daily Limits
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Limits"</h3>
                    <div class="space-y-4">
                        // Salt/Sodium with linked inputs
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Salt Limit"</label>
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"Sodium (mg)"</label>
                                    <input
                                        type="number"
                                        prop:value=move || sodium_mg.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                set_sodium_mg.set(val.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"Salt (g)"</label>
                                    <input
                                        type="number"
                                        step="0.1"
                                        prop:value=move || salt_grams.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                // Convert salt grams back to sodium mg
                                                let sodium = (val * 393.4).round() as i32;
                                                set_sodium_mg.set(sodium.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                            </div>
                            <p class="mt-1 text-xs text-slate-500">"Recommended: 2300mg sodium / 5.8g salt (US), 2360mg / 6g (UK)"</p>
                        </div>

                        // Saturated Fat with dual inputs (grams and percentage)
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Saturated Fat Limit"</label>
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"Grams"</label>
                                    <input
                                        type="number"
                                        prop:value=move || sat_fat_grams.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                set_sat_fat_grams.set(val.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"% of Daily Calories"</label>
                                    <input
                                        type="number"
                                        step="0.1"
                                        prop:value=move || sat_fat_pct.get()
                                        on:input=move |ev| {
                                            if let Ok(pct) = event_target_value(&ev).parse::<f64>() {
                                                // Convert percentage to grams
                                                let cals = daily_calories.get() as f64;
                                                let grams = (pct / 100.0 * cals / CALORIES_PER_GRAM_FAT).round() as i32;
                                                set_sat_fat_grams.set(grams.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                            </div>
                            <p class="mt-1 text-xs text-slate-500">"Recommended: Less than 10% of daily calories (AHA recommends <6% for heart health)"</p>
                        </div>
                    </div>
                </div>

                // Daily Minimums
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Minimums"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Minimum Fiber Intake (g)"</label>
                            <input
                                type="number"
                                prop:value=move || fiber_min.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        set_fiber_min.set(val.max(0));
                                    }
                                }
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Recommended: 25-30g per day for adults"</p>
                        </div>
                    </div>
                </div>

                <div class="flex justify-end">
                    <button class="rounded bg-blue-600 px-6 py-2 font-semibold text-white hover:bg-blue-700">
                        "Save Settings"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize admin auth context
    let auth = AdminAuth::new();
    auth.init();
    provide_context(auth);

    view! {
        <Router>
            <Navigation />
            <PinModal />
            <main class="min-h-screen bg-slate-100 px-4">
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/ingredients") view=Ingredients />
                    <Route path=path!("/recipes") view=Recipes />
                    <Route path=path!("/settings") view=Settings />
                </Routes>
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/styles.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <title>"food.lemmih.com"</title>
            </head>
            <body class="bg-slate-100 text-slate-900">
                <App/>
            </body>
        </html>
    }
}
