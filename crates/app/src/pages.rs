//! Pages module
//!
//! Contains the Navigation component and Home page.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::AdminAuthButton;

#[component]
pub fn Navigation() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);
    let links: [(&str, &str); 4] = [
        ("/", "Food Log"),
        ("/ingredients", "Ingredients"),
        ("/recipes", "Recipes"),
        ("/settings", "Settings"),
    ];

    view! {
      <nav class="bg-slate-800 text-white shadow-md">
        <div class="mx-auto max-w-7xl px-4">
          <div class="flex h-16 items-center justify-between">
            <div class="flex items-center gap-8">
              <h1 class="text-xl font-bold">"food.lemmih.com"</h1>
              <div class="hidden space-x-4 sm:flex">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! {
                      <A href=href attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">
                        {label}
                      </A>
                    }
                  })
                  .collect_view()}
              </div>
            </div>

            <div class="flex items-center gap-3">
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
            <div id="primary-navigation" class="space-y-2 border-t border-slate-700 pb-4 pt-4 sm:hidden">
              <div class="space-y-1">
                {links
                  .iter()
                  .map(|&(href, label)| {
                    view! {
                      <A href=href attr:class="block rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">
                        {label}
                      </A>
                    }
                  })
                  .collect_view()}
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
pub fn Home() -> impl IntoView {
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
            <p class="text-slate-700">
              "Delicious homemade pizza with fresh mozzarella and basil. Perfect crispy crust!"
            </p>
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
            <p class="text-slate-700">
              "Fresh romaine with grilled chicken, parmesan, and homemade dressing. Very satisfying!"
            </p>
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
