//! Admin Authentication module
//!
//! Contains server-side auth state, server functions for login/logout/validate,
//! and client-side auth components (PinModal, UnlockButton, LogoutButton).

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use wasm_bindgen::JsCast;

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
    let put_builder = kv
        .inner()
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
pub struct AdminAuth {
    pub is_authenticated: RwSignal<bool>,
    pub token: RwSignal<Option<String>>,
    pub show_modal: RwSignal<bool>,
    pub error_message: RwSignal<Option<String>>,
}

impl AdminAuth {
    pub fn new() -> Self {
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
    pub fn init(&self) {
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
    pub fn init(&self) {
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
    pub async fn login(&self, pin: &str) {
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
    pub async fn login(&self, _pin: &str) {
        // SSR: no-op
    }

    /// Logout and invalidate token using server function
    /// Only runs on client (browser), not on SSR (CloudFlare Workers)
    #[cfg(not(feature = "ssr"))]
    pub async fn logout(&self) {
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
    pub async fn logout(&self) {
        // SSR: no-op
    }

    pub fn open_modal(&self) {
        self.error_message.set(None);
        self.show_modal.set(true);
    }

    pub fn close_modal(&self) {
        self.show_modal.set(false);
        self.error_message.set(None);
    }
}

impl Default for AdminAuth {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Components
// ============================================================================

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
                auth.error_message
                    .set(Some("Please enter a 4-digit PIN".to_string()));
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
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
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
                        if !value.is_empty() {
                          if i < 3 {
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
                            do_submit();
                          }
                        }
                      }
                    }
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
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
                  />
                }
              })
              .collect_view()}
          </div>

          <Show when=move || auth.error_message.get().is_some()>
            <p class="mb-4 text-sm text-red-600">{move || auth.error_message.get().unwrap_or_default()}</p>
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
              disabled=move || { pin_digits.iter().map(|d| d.get()).collect::<String>().len() != 4 }
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
pub fn PinModal() -> impl IntoView {
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
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
          />
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
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"
          />
        </svg>
        "Logout"
      </button>
    }
}

/// Admin Auth Button (Unlock/Logout)
#[component]
pub fn AdminAuthButton() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    view! {
      <Show when=move || auth.is_authenticated.get() fallback=|| view! { <UnlockButton /> }>
        <LogoutButton />
      </Show>
    }
}
