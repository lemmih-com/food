//! Pages module
//!
//! Contains the Navigation component and Home page.

use leptos::prelude::*;

use crate::auth::AdminAuthButton;
use crate::components::NavLink;

// ============================================================================
// Dark Mode
// ============================================================================

#[cfg(not(feature = "ssr"))]
const THEME_STORAGE_KEY: &str = "theme";

/// Dark mode state - stored in context for app-wide access
#[derive(Clone, Copy)]
pub struct DarkMode {
    pub is_dark: RwSignal<bool>,
}

impl DarkMode {
    pub fn new() -> Self {
        Self {
            is_dark: RwSignal::new(false),
        }
    }

    /// Initialize dark mode from localStorage or system preference
    #[cfg(not(feature = "ssr"))]
    pub fn init(&self) {
        use gloo_storage::{LocalStorage, Storage};

        // Check localStorage first
        if let Ok(stored) = LocalStorage::get::<String>(THEME_STORAGE_KEY) {
            let is_dark = stored == "dark";
            self.is_dark.set(is_dark);
            self.apply_theme(is_dark);
            return;
        }

        // Fall back to system preference
        let prefers_dark = web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
            .flatten()
            .map(|m| m.matches())
            .unwrap_or(false);

        self.is_dark.set(prefers_dark);
        self.apply_theme(prefers_dark);
    }

    #[cfg(feature = "ssr")]
    pub fn init(&self) {
        // SSR: no localStorage or DOM access
    }

    /// Toggle between light and dark mode
    #[cfg(not(feature = "ssr"))]
    pub fn toggle(&self) {
        use gloo_storage::{LocalStorage, Storage};

        let new_is_dark = !self.is_dark.get();
        self.is_dark.set(new_is_dark);

        // Save to localStorage
        let theme = if new_is_dark { "dark" } else { "light" };
        let _ = LocalStorage::set(THEME_STORAGE_KEY, theme);

        self.apply_theme(new_is_dark);
    }

    #[cfg(feature = "ssr")]
    pub fn toggle(&self) {
        // SSR: no-op
    }

    /// Apply theme by toggling dark class on document element
    #[cfg(not(feature = "ssr"))]
    fn apply_theme(&self, is_dark: bool) {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let class_list = html.class_list();
                if is_dark {
                    let _ = class_list.add_1("dark");
                } else {
                    let _ = class_list.remove_1("dark");
                }
            }
        }
    }
}

impl Default for DarkMode {
    fn default() -> Self {
        Self::new()
    }
}

/// Compact dark mode toggle button with animated sun/moon icons
#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let dark_mode = expect_context::<DarkMode>();

    view! {
      <button
        type="button"
        class="relative h-8 w-8 rounded-full p-1.5 text-slate-200 hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-white transition-colors"
        attr:aria-label=move || if dark_mode.is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
        on:click=move |_| dark_mode.toggle()
      >
        // Sun icon - shown in dark mode (click to go light)
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="2"
          stroke="currentColor"
          class=move || {
            if dark_mode.is_dark.get() {
              "absolute inset-1.5 h-5 w-5 transition-all duration-300 ease-in-out rotate-0 scale-100 opacity-100"
            } else {
              "absolute inset-1.5 h-5 w-5 transition-all duration-300 ease-in-out -rotate-90 scale-0 opacity-0 pointer-events-none"
            }
          }
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
          />
        </svg>
        // Moon icon - shown in light mode (click to go dark)
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="2"
          stroke="currentColor"
          class=move || {
            if dark_mode.is_dark.get() {
              "absolute inset-1.5 h-5 w-5 transition-all duration-300 ease-in-out rotate-90 scale-0 opacity-0 pointer-events-none"
            } else {
              "absolute inset-1.5 h-5 w-5 transition-all duration-300 ease-in-out rotate-0 scale-100 opacity-100"
            }
          }
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"
          />
        </svg>
      </button>
    }
}

#[component]
pub fn Navigation() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let links: [(&str, &str); 5] = [
        ("/", "Food Log"),
        ("/ingredients", "Ingredients"),
        ("/recipes", "Recipes"),
        ("/settings", "Settings"),
        ("/about", "About"),
    ];

    view! {
      <nav class="bg-slate-800 text-white shadow-md dark:bg-slate-950">
        <div class="mx-auto max-w-7xl px-4">
          <div class="flex h-16 items-center justify-between">
            <div class="flex items-center gap-8">
              <h1 class="text-xl font-bold">"food.lemmih.com"</h1>
              <div class="hidden space-x-4 sm:flex">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! { <NavLink href=href label=label /> }
                  })
                  .collect_view()}
              </div>
            </div>

            <div class="flex items-center gap-3">
              <DarkModeToggle />
              <div class="hidden sm:flex">
                <AdminAuthButton />
              </div>

              <button
                type="button"
                class="inline-flex items-center justify-center rounded-md p-2 text-slate-200 hover:bg-slate-700 dark:hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-white sm:hidden"
                attr:aria-controls="primary-navigation"
                attr:aria-expanded=move || menu_open.get().to_string()
                on:click=move |_| set_menu_open.update(|open| *open = !*open)
              >
                <span class="sr-only">"Toggle navigation"</span>
                <Show
                  when=move || menu_open.get()
                  fallback=move || {
                    view! {
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-6 w-6"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                        />
                      </svg>
                    }
                  }
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="h-6 w-6"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                  </svg>
                </Show>
              </button>
            </div>
          </div>

          <Show when=move || menu_open.get()>
            <div
              id="primary-navigation"
              class="space-y-2 border-t border-slate-700 dark:border-slate-800 pb-4 pt-4 sm:hidden"
            >
              <div class="space-y-1">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! { <NavLink href=href label=label mobile=true /> }
                  })
                  .collect_view()}
              </div>
              <div class="border-t border-slate-700 dark:border-slate-800 pt-3">
                <AdminAuthButton />
              </div>
            </div>
          </Show>
        </div>
      </nav>
    }
}
