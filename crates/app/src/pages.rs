//! Pages module
//!
//! Contains the Navigation component and Home page.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{auth::AdminAuthButton, Theme, ThemeState};

#[component]
pub fn Navigation() -> impl IntoView {
    let theme_state = expect_context::<ThemeState>();
    let (menu_open, set_menu_open) = signal(false);
    let links: [(&str, &str); 4] = [
        ("/", "Food Log"),
        ("/ingredients", "Ingredients"),
        ("/recipes", "Recipes"),
        ("/settings", "Settings"),
    ];
    let is_dark = move || theme_state.theme.get() == Theme::Dark;

    view! {
      <nav class=move || match theme_state.theme.get() {
        Theme::Light => "border-b border-slate-200 bg-white text-slate-900 shadow-md",
        Theme::Dark => "border-b border-slate-800 bg-slate-900 text-slate-100 shadow-lg",
      }>
        <div class="mx-auto max-w-7xl px-4">
          <div class="flex h-16 items-center justify-between">
            <div class="flex items-center gap-8">
              <h1 class="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100">"food.lemmih.com"</h1>
              <div class="hidden space-x-4 sm:flex">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! {
                      <A
                        href=href
                        attr:class="rounded px-3 py-2 text-sm font-medium transition hover:bg-slate-100 hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-slate-50"
                      >
                        {label}
                      </A>
                    }
                  })
                  .collect_view()}
              </div>
            </div>

            <div class="flex items-center gap-3">
              <ThemeToggle />

              <div class="hidden sm:flex">
                <AdminAuthButton />
              </div>

              <button
                type="button"
                class="inline-flex items-center justify-center rounded-md p-2 text-slate-200 hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-white sm:hidden"
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
              class=move || {
                format!(
                  "space-y-2 pb-4 pt-4 sm:hidden {} {}",
                  if is_dark() { "border-t border-slate-800" } else { "border-t border-slate-200" },
                  if is_dark() { "bg-slate-900/80" } else { "bg-white/90" },
                )
              }
            >
              <div class="space-y-1">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! {
                      <A
                        href=href
                        attr:class="block rounded px-3 py-2 text-sm font-medium transition hover:bg-slate-100 hover:text-slate-900 dark:hover:bg-slate-800 dark:hover:text-slate-50"
                      >
                        {label}
                      </A>
                    }
                  })
                  .collect_view()}
              </div>
              <div class="border-t border-slate-700 pt-3">
                <ThemeToggle />
              </div>
              <div class="border-t border-slate-700 pt-3">
                <AdminAuthButton />
              </div>
            </div>
          </Show>
        </div>
      </nav>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme_state = expect_context::<ThemeState>();
    let is_dark = move || theme_state.theme.get() == Theme::Dark;

    let pill_classes = move || {
        if is_dark() {
            "bg-gradient-to-r from-slate-600 via-slate-700 to-slate-900"
        } else {
            "bg-gradient-to-r from-amber-300 via-amber-400 to-orange-500"
        }
    };

    view! {
      <button
        type="button"
        class=move || {
          format!(
            "group relative inline-flex items-center gap-2 rounded-full border px-2.5 py-1.5 text-xs font-semibold transition focus:outline-none focus:ring-2 focus:ring-emerald-400/70 {}",
            if is_dark() {
              "border-slate-700/80 bg-slate-900/90 text-slate-100 shadow-lg shadow-emerald-500/10 hover:-translate-y-0.5 hover:bg-slate-800"
            } else {
              "border-amber-200/80 bg-white/90 text-slate-900 shadow-md shadow-amber-400/20 hover:-translate-y-0.5 hover:bg-amber-50"
            },
          )
        }
        on:click=move |_| theme_state.set_theme.update(|theme| *theme = theme.toggle())
        title="Toggle light and dark mode"
      >
        <div class=move || {
          format!(
            "relative flex h-8 w-14 items-center overflow-hidden rounded-full transition-colors {}",
            pill_classes(),
          )
        }>
          <span
            class=move || {
              format!(
                "absolute left-2 flex h-4 w-4 items-center justify-center transition-opacity {}",
                if is_dark() { "opacity-0" } else { "opacity-100" },
              )
            }
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              class="h-5 w-5 text-amber-700 drop-shadow"
            >
              <path d="M12 4.5a1 1 0 0 1 1 1V7a1 1 0 1 1-2 0V5.5a1 1 0 0 1 1-1Z" />
              <path d="M6.136 6.136a1 1 0 0 1 1.414 0l1.06 1.06a1 1 0 1 1-1.414 1.415l-1.06-1.06a1 1 0 0 1 0-1.415Z" />
              <path d="M4.5 12a1 1 0 0 1 1-1H7a1 1 0 1 1 0 2H5.5a1 1 0 0 1-1-1Z" />
              <path d="M6.136 17.864a1 1 0 0 1 0-1.414l1.06-1.06a1 1 0 1 1 1.414 1.414l-1.06 1.06a1 1 0 0 1-1.414 0Z" />
              <path d="M12 17a1 1 0 0 1 1 1v1.5a1 1 0 1 1-2 0V18a1 1 0 0 1 1-1Z" />
              <path d="M17.864 17.864a1 1 0 0 1-1.414 0l-1.06-1.06a1 1 0 1 1 1.414-1.415l1.06 1.06a1 1 0 0 1 0 1.415Z" />
              <path d="M17 12a1 1 0 0 1 1-1h1.5a1 1 0 1 1 0 2H18a1 1 0 0 1-1-1Z" />
              <path d="M16.45 6.95a1 1 0 0 1 0-1.414l1.062-1.063a1 1 0 1 1 1.414 1.414L17.864 6.95a1 1 0 0 1-1.415 0Z" />
              <circle cx="12" cy="12" r="3" />
            </svg>
          </span>
          <span
            class=move || {
              format!(
                "absolute right-2 flex h-4 w-4 items-center justify-center transition-opacity {}",
                if is_dark() { "opacity-100" } else { "opacity-0" },
              )
            }
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              class="h-5 w-5 text-amber-100 drop-shadow"
            >
              <path d="M20.354 15.354a8.5 8.5 0 1 1-11.707-11.707 8.5 8.5 0 0 0 11.707 11.707Z" />
            </svg>
          </span>
          <div
            class=move || {
              format!(
                "absolute left-1 top-1 flex h-6 w-6 items-center justify-center rounded-full shadow-md ring-1 transition-transform duration-200 {}",
                if is_dark() {
                  "translate-x-6 bg-slate-950 text-amber-200 ring-slate-700/70"
                } else {
                  "translate-x-0 bg-white text-amber-500 ring-white/40"
                },
              )
            }
            aria-hidden="true"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              class=move || {
                format!("h-4 w-4 transition-opacity {}", if is_dark() { "opacity-100" } else { "opacity-80" })
              }
            >
              <path d="M12 2a1 1 0 0 1 .894.553l.021.047A10 10 0 1 1 3.4 16.32a.75.75 0 0 1 .986-1.094 8.5 8.5 0 1 0 9.3-13.009A1 1 0 0 1 12 2Z" />
            </svg>
          </div>
        </div>
      </button>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="mx-auto max-w-7xl py-6">
        <h2 class="mb-6 text-3xl font-bold text-slate-900 dark:text-slate-100">"Food Log"</h2>
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <div class="rounded-lg border border-slate-200 bg-white p-6 shadow-md dark:border-slate-800 dark:bg-slate-900">
            <div class="mb-4 flex h-48 items-center justify-center rounded bg-gradient-to-br from-orange-400 to-pink-400">
              <span class="text-6xl">"🍕"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Margherita Pizza"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-400">"Date: 2025-12-01"</p>
            <div class="mb-3 flex gap-2 text-xs">
              <span class="rounded bg-blue-100 px-2 py-1 text-blue-800 dark:bg-blue-500/20 dark:text-blue-100">
                "Dinner"
              </span>
              <span class="rounded bg-green-100 px-2 py-1 text-green-800 dark:bg-emerald-500/20 dark:text-emerald-100">
                "750 kcal"
              </span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Delicious homemade pizza with fresh mozzarella and basil. Perfect crispy crust!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-400">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
            </div>
          </div>

          <div class="rounded-lg border border-slate-200 bg-white p-6 shadow-md dark:border-slate-800 dark:bg-slate-900">
            <div class="mb-4 flex h-48 items-center justify-center rounded bg-gradient-to-br from-green-400 to-emerald-500">
              <span class="text-6xl">"🥗"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Caesar Salad"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-400">"Date: 2025-12-02"</p>
            <div class="mb-3 flex gap-2 text-xs">
              <span class="rounded bg-blue-100 px-2 py-1 text-blue-800 dark:bg-blue-500/20 dark:text-blue-100">
                "Lunch"
              </span>
              <span class="rounded bg-green-100 px-2 py-1 text-green-800 dark:bg-emerald-500/20 dark:text-emerald-100">
                "350 kcal"
              </span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Fresh romaine with grilled chicken, parmesan, and homemade dressing. Very satisfying!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-400">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐"</span>
            </div>
          </div>

          <div class="rounded-lg border border-slate-200 bg-white p-6 shadow-md dark:border-slate-800 dark:bg-slate-900">
            <div class="mb-4 flex h-48 items-center justify-center rounded bg-gradient-to-br from-red-400 to-rose-500">
              <span class="text-6xl">"🍝"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Spaghetti Carbonara"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-400">"Date: 2025-12-03"</p>
            <div class="mb-3 flex gap-2 text-xs">
              <span class="rounded bg-blue-100 px-2 py-1 text-blue-800 dark:bg-blue-500/20 dark:text-blue-100">
                "Dinner"
              </span>
              <span class="rounded bg-green-100 px-2 py-1 text-green-800 dark:bg-emerald-500/20 dark:text-emerald-100">
                "650 kcal"
              </span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Classic Italian pasta with eggs, pecorino, and guanciale. Rich and creamy!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-400">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
            </div>
          </div>
        </div>
      </div>
    }
}
