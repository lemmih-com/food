use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    theme: RwSignal<Theme>,
}

impl ThemeContext {
    pub fn new() -> Self {
        Self {
            theme: RwSignal::new(initial_theme()),
        }
    }

    pub fn theme(&self) -> ReadSignal<Theme> {
        self.theme.read_only()
    }

    pub fn is_dark(&self) -> Signal<bool> {
        Signal::derive({
            let theme = self.theme;
            move || theme.get() == Theme::Dark
        })
    }

    pub fn toggle(&self) {
        self.set_theme(self.theme.get().toggle());
    }

    pub fn set_theme(&self, next: Theme) {
        self.theme.set(next);
        persist_theme(next);
    }

    /// Syncs the DOM classes and localStorage when the theme changes.
    /// Should be called once after providing the context.
    pub fn register_dom_sync(&self) {
        #[cfg(not(feature = "ssr"))]
        {
            let theme = self.theme;
            Effect::new(move |_| {
                persist_theme(theme.get());
            });
        }
    }
}

#[cfg(feature = "ssr")]
fn initial_theme() -> Theme {
    Theme::Light
}

#[cfg(not(feature = "ssr"))]
fn initial_theme() -> Theme {
    use gloo_storage::{LocalStorage, Storage};

    if let Ok(saved) = LocalStorage::get::<String>(THEME_STORAGE_KEY) {
        if saved == Theme::Dark.as_str() {
            return Theme::Dark;
        }
    }

    match web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
        .flatten()
    {
        Some(media) if media.matches() => Theme::Dark,
        _ => Theme::Light,
    }
}

#[cfg(feature = "ssr")]
fn persist_theme(_theme: Theme) {}

#[cfg(not(feature = "ssr"))]
const THEME_STORAGE_KEY: &str = "ybi:theme";

#[cfg(not(feature = "ssr"))]
fn persist_theme(theme: Theme) {
    use gloo_storage::{LocalStorage, Storage};

    let _ = LocalStorage::set(THEME_STORAGE_KEY, theme.as_str());
    sync_dom_theme(theme);
}

#[cfg(not(feature = "ssr"))]
fn sync_dom_theme(theme: Theme) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let _ = root.class_list().remove_2("light", "dark");
                let _ = root.class_list().add_1(theme.as_str());
            }
            if let Some(body) = document.body() {
                let _ = body.class_list().remove_2("light", "dark");
                let _ = body.class_list().add_1(theme.as_str());
            }
        }
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = expect_context::<ThemeContext>();
    let is_dark = theme.is_dark();

    view! {
      <button
        type="button"
        class="group flex items-center gap-2 rounded-full border border-slate-200 bg-white/80 px-3 py-2 text-sm font-medium text-slate-700 shadow-sm ring-1 ring-black/5 transition hover:-translate-y-[1px] hover:shadow-md dark:border-slate-700 dark:bg-slate-800/80 dark:text-slate-100 dark:ring-white/5"
        aria-label="Toggle light or dark theme"
        on:click=move |_| theme.toggle()
      >
        <span class="hidden text-xs uppercase tracking-wide text-slate-500 dark:text-slate-300 sm:inline">
          {move || if is_dark.get() { "Dark" } else { "Light" }}
        </span>
        <div class="relative h-7 w-12 rounded-full bg-gradient-to-r from-amber-200 via-orange-300 to-amber-400 shadow-inner transition-all duration-300 dark:from-slate-700 dark:via-indigo-700 dark:to-slate-900">
          <span
            class="absolute left-1 top-1 h-5 w-5 rounded-full bg-white shadow-md transition-transform duration-300 group-hover:shadow-lg dark:bg-slate-950"
            class=("translate-x-5", move || is_dark.get())
          ></span>
          <svg
            class="absolute left-1.5 top-1.5 h-4 w-4 text-amber-700 transition-opacity duration-300"
            class=("opacity-0", move || is_dark.get())
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 4v1m5 6h1m-1.64 4.36.7.7M4 12H3m2.34 4.36-.7.7M7 7l-.7-.7M17 7l.7-.7M12 19v1m-4.36-2.34-.7.7"
            />
          </svg>
          <svg
            class="absolute right-1.5 top-1.5 h-4 w-4 text-indigo-100 transition-opacity duration-300"
            class=("opacity-0", move || !is_dark.get())
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 12.79A9 9 0 1111.21 3a7 7 0 0010.58 9.79z"
            />
          </svg>
        </div>
      </button>
    }
}
