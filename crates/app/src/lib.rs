#![recursion_limit = "512"]

pub mod auth;
pub mod ingredients;
pub mod pages;
pub mod recipes;
pub mod settings;

#[cfg(not(feature = "ssr"))]
use gloo_storage::{LocalStorage, Storage};
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
use serde::{Deserialize, Serialize};

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

use auth::PinModal;
use ingredients::Ingredients;
use pages::{Home, Navigation};
use recipes::Recipes;
use settings::Settings;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn toggle(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

const THEME_STORAGE_KEY: &str = "food_theme";

fn load_theme() -> Theme {
    #[cfg(feature = "ssr")]
    {
        let _ = THEME_STORAGE_KEY;
        Theme::Light
    }

    #[cfg(not(feature = "ssr"))]
    {
        LocalStorage::get(THEME_STORAGE_KEY).unwrap_or(Theme::Light)
    }
}

fn persist_theme(theme: Theme) {
    #[cfg(feature = "ssr")]
    {
        let _ = theme;
        let _ = THEME_STORAGE_KEY;
    }

    #[cfg(not(feature = "ssr"))]
    let _ = LocalStorage::set(THEME_STORAGE_KEY, theme);
}

fn sync_dom_theme(theme: Theme) {
    #[cfg(feature = "ssr")]
    let _ = theme;

    #[cfg(not(feature = "ssr"))]
    {
        use web_sys::window;

        if let Some(document) = window().and_then(|win| win.document()) {
            if let Some(el) = document.document_element() {
                let classes = el.class_list();
                let _ = classes.remove_1("dark");
                if theme == Theme::Dark {
                    let _ = classes.add_1("dark");
                }
            }

            if let Some(body) = document.body() {
                let (bg, text) = match theme {
                    Theme::Light => ("bg-slate-100", "text-slate-900"),
                    Theme::Dark => ("bg-slate-950", "text-slate-100"),
                };
                body.set_class_name(&format!("{bg} {text} antialiased"));
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize admin auth context
    let auth = AdminAuth::new();
    auth.init();
    provide_context(auth);

    let initial_theme = load_theme();
    let (theme, set_theme) = signal(initial_theme);
    provide_context(ThemeState { theme, set_theme });

    Effect::new(move |_| {
        let current = theme.get();
        persist_theme(current);
        sync_dom_theme(current);
    });

    view! {
      <Router>
        <Navigation />
        <PinModal />
        <main class=move || match theme.get() {
          Theme::Light => "min-h-screen bg-slate-100 px-4 text-slate-900",
          Theme::Dark => "min-h-screen bg-slate-950 px-4 text-slate-100",
        }>
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
          <AutoReload options=options.clone() />
          <HydrationScripts options />
          <MetaTags />
          <title>"food.lemmih.com"</title>
        </head>
        <body class="bg-slate-100 text-slate-900">
          <App />
        </body>
      </html>
    }
}
