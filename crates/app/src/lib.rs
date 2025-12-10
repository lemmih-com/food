#![recursion_limit = "512"]

pub mod auth;
pub mod ingredients;
pub mod pages;
pub mod recipes;
pub mod settings;
pub mod theme;

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
use theme::ThemeContext;

const THEME_OVERRIDES: &str = r#"
:root { color-scheme: light; }
.dark { color-scheme: dark; }

body {
  background: #f1f5f9;
  color: #0f172a;
  transition: background-color 200ms ease, color 200ms ease;
}

.dark body {
  background: radial-gradient(circle at 15% 20%, #0b1220 0%, #050910 55%);
  color: #e2e8f0;
}

.dark .bg-white { background-color: #0f172a; }
.dark .bg-slate-50 { background-color: #0b1220; }
.dark .bg-slate-100 { background-color: #0d1423; }
.dark .bg-slate-200 { background-color: #111827; }
.dark .text-slate-900 { color: #e2e8f0; }
.dark .text-slate-800 { color: #e2e8f0; }
.dark .text-slate-700 { color: #cbd5e1; }
.dark .text-slate-600 { color: #cbd5e1; }
.dark .text-slate-500 { color: #94a3b8; }
.dark .border-slate-300 { border-color: #334155; }
.dark .border-slate-200 { border-color: #334155; }
.dark .divide-slate-200 > :not([hidden]) ~ :not([hidden]) { border-color: #334155; }
.dark input,
.dark select,
.dark textarea {
  background-color: #0f172a;
  color: #e2e8f0;
  border-color: #334155;
}
.dark ::placeholder { color: #64748b; }
.dark .hover\:bg-slate-50:hover { background-color: #1e293b; }
.dark .hover\:bg-slate-100:hover { background-color: #1f2937; }
.dark .hover\:bg-slate-200:hover { background-color: #1f2937; }
.dark .bg-blue-100 { background-color: #1e3a8a; color: #cbd5ff; }
.dark .bg-green-100 { background-color: #0f3e2c; color: #bbf7d0; }
.dark .bg-yellow-100 { background-color: #713f12; color: #fde68a; }
.dark .bg-orange-100 { background-color: #7c2d12; color: #fdba74; }
.dark .bg-amber-100 { background-color: #78350f; color: #fef3c7; }
.dark .bg-purple-100 { background-color: #3b0764; color: #e9d5ff; }
.dark .text-yellow-500 { color: #fbbf24; }
"#;

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
    CreateIngredient, DeleteIngredient, GetIngredients, Ingredient, IngredientCategory,
    UpdateIngredient,
};
// Re-export recipe server functions
pub use recipes::{
    DeleteRecipe, GetRecipes, Recipe, RecipeIngredient, RecipeIngredientInput, RecipeInput,
    UpsertRecipe,
};

use auth::PinModal;
use ingredients::Ingredients;
use pages::{Home, Navigation};
use recipes::Recipes;
use settings::Settings;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let theme = ThemeContext::new();
    theme.register_dom_sync();
    provide_context(theme);

    // Initialize admin auth context
    let auth = AdminAuth::new();
    auth.init();
    provide_context(auth);

    view! {
      <Router>
        <Navigation />
        <PinModal />
        <main class="min-h-screen bg-slate-100 px-4 text-slate-900 transition-colors duration-300 dark:bg-slate-950 dark:text-slate-100">
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
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <link rel="stylesheet" href="/pkg/styles.css" />
          <style>{THEME_OVERRIDES}</style>
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
          <title>"food.lemmih.com"</title>
        </head>
        <body class="bg-slate-100 text-slate-900 transition-colors duration-300 dark:bg-slate-950 dark:text-slate-100">
          <App />
        </body>
      </html>
    }
}
