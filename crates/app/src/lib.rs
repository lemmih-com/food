#![recursion_limit = "512"]

pub mod auth;
pub mod cache;
pub mod components;
pub mod food_log;
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

// Re-export food log types for worker registration
#[cfg(feature = "ssr")]
pub use food_log::SendR2Bucket;
pub use food_log::{
    CreateFoodLog, DeleteFoodImage, DeleteFoodLog, FoodLog, GetFoodImage, GetFoodLogs,
    UpdateFoodLog, UploadFoodImage,
};

use auth::PinModal;
use food_log::FoodLogs;
use ingredients::Ingredients;
use pages::{DarkMode, Navigation};
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
            <Route path=path!("/") view=FoodLogs />
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
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
          <title>"food.lemmih.com"</title>
        </head>
        <body class="bg-slate-100 text-slate-900 dark:bg-slate-900 dark:text-slate-100 transition-colors">
          <App />
        </body>
      </html>
    }
}
