#![recursion_limit = "512"]

pub mod auth;
pub mod cache;
pub mod ingredients;
pub mod pages;
pub mod recipes;
pub mod settings;

use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::*,
};
use leptos_config::LeptosOptions;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

// Re-export public types from modules
pub use auth::{AdminAuth, AuthState, LoginResult, ValidateResult};
// Re-export server function types for worker registration
#[cfg(feature = "ssr")]
pub use auth::SendKvStore;
pub use auth::{AdminLogin, AdminLogout, AdminValidate};

// Re-export ingredient types for worker registration
#[cfg(feature = "ssr")]
pub use ingredients::SendD1Database;
pub use ingredients::{
    BulkUpsertIngredients, CreateIngredient, DeleteIngredient, GetIngredients, Ingredient,
    UpdateIngredient,
};

// Re-export recipe types for worker registration
pub use recipes::{CreateRecipe, DeleteRecipe, GetRecipes, Recipe, UpdateRecipe};

use auth::PinModal;
use ingredients::Ingredients;
use pages::{DarkMode, Home, Navigation};
use recipes::Recipes;
use settings::Settings;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize admin auth context
    let auth = AdminAuth::new();
    auth.init();
    provide_context(auth);

    // Initialize dark mode context
    let dark_mode = DarkMode::new();
    dark_mode.init();
    provide_context(dark_mode);

    view! {
      <Router>
        <Navigation />
        <PinModal />
        <main class="min-h-screen bg-slate-100 px-4 dark:bg-slate-900 transition-colors">
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

/// Inline script that detects hydration failures and shows a user-friendly error message.
/// This helps diagnose mobile-specific WASM loading issues.
const HYDRATION_ERROR_SCRIPT: &str = r#"
(function() {
  // Set a timeout to check if hydration completed
  // WASM loading typically takes 2-5 seconds, so we wait 10 seconds to be safe
  var HYDRATION_TIMEOUT_MS = 10000;
  
  window.__HYDRATION_COMPLETE__ = false;
  
  // Store the original hydrate function location for detection
  var timeoutId = setTimeout(function() {
    if (!window.__HYDRATION_COMPLETE__) {
      console.error('[Hydration Error] WASM hydration did not complete within ' + HYDRATION_TIMEOUT_MS + 'ms');
      
      // Create and show error banner
      var banner = document.createElement('div');
      banner.id = 'hydration-error-banner';
      banner.style.cssText = 'position:fixed;top:0;left:0;right:0;background:#dc2626;color:white;padding:12px 16px;text-align:center;z-index:9999;font-family:system-ui,sans-serif;font-size:14px;';
      banner.innerHTML = '<strong>Interactive features failed to load.</strong> Try refreshing the page. If the problem persists, try a different browser. <button onclick="this.parentElement.remove()" style="margin-left:12px;background:white;color:#dc2626;border:none;padding:4px 12px;border-radius:4px;cursor:pointer;font-weight:bold;">Dismiss</button>';
      
      // Insert at the beginning of body
      if (document.body) {
        document.body.insertBefore(banner, document.body.firstChild);
      }
      
      // Log additional debug info
      console.error('[Hydration Debug] User Agent:', navigator.userAgent);
      console.error('[Hydration Debug] WASM supported:', typeof WebAssembly !== 'undefined');
      if (typeof WebAssembly !== 'undefined') {
        console.error('[Hydration Debug] WASM streaming supported:', typeof WebAssembly.instantiateStreaming === 'function');
      }
    }
  }, HYDRATION_TIMEOUT_MS);
  
  // Store timeout ID so it can be cleared on successful hydration
  window.__HYDRATION_TIMEOUT_ID__ = timeoutId;
})();
"#;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="stylesheet" href="/pkg/styles.css" />
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          // Hydration error detection script - runs early to set up timeout
          <script inner_html=HYDRATION_ERROR_SCRIPT></script>
          <MetaTags />
          <title>"food.lemmih.com"</title>
        </head>
        <body class="bg-slate-100 text-slate-900 dark:bg-slate-900 dark:text-slate-100 transition-colors">
          <App />
        </body>
      </html>
    }
}
