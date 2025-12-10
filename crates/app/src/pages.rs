//! Pages module
//!
//! Contains the Navigation component and Home page.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::AdminAuthButton;
use crate::theme::ThemeToggle;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
      <nav class="sticky top-0 z-40 border-b border-slate-200/70 bg-white/80 text-slate-900 shadow-sm backdrop-blur dark:border-slate-800 dark:bg-slate-900/80 dark:text-slate-100">
        <div class="mx-auto max-w-7xl px-4">
          <div class="flex h-16 items-center justify-between">
            <div class="flex items-center space-x-8">
              <h1 class="text-xl font-bold tracking-tight">"food.lemmih.com"</h1>
              <div class="flex space-x-4">
                <A
                  href="/"
                  attr:class="rounded px-3 py-2 text-sm font-medium transition-colors hover:bg-slate-100 dark:hover:bg-slate-800"
                >
                  "Food Log"
                </A>
                <A
                  href="/ingredients"
                  attr:class="rounded px-3 py-2 text-sm font-medium transition-colors hover:bg-slate-100 dark:hover:bg-slate-800"
                >
                  "Ingredients"
                </A>
                <A
                  href="/recipes"
                  attr:class="rounded px-3 py-2 text-sm font-medium transition-colors hover:bg-slate-100 dark:hover:bg-slate-800"
                >
                  "Recipes"
                </A>
                <A
                  href="/settings"
                  attr:class="rounded px-3 py-2 text-sm font-medium transition-colors hover:bg-slate-100 dark:hover:bg-slate-800"
                >
                  "Settings"
                </A>
              </div>
            </div>
            <div class="flex items-center gap-3">
              <ThemeToggle />
              <AdminAuthButton />
            </div>
          </div>
        </div>
      </nav>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="mx-auto max-w-7xl py-6">
        <h2 class="mb-6 text-3xl font-bold text-slate-900 dark:text-slate-100">"Food Log"</h2>
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <div class="rounded-2xl border border-slate-200/80 bg-white/90 p-6 shadow-lg shadow-slate-200/50 transition hover:-translate-y-1 hover:shadow-xl dark:border-slate-800 dark:bg-slate-900/80 dark:shadow-black/40">
            <div class="mb-4 h-48 rounded bg-gradient-to-br from-orange-400 to-pink-400 flex items-center justify-center">
              <span class="text-6xl">"🍕"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Margherita Pizza"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-300">"Date: 2025-12-01"</p>
            <div class="mb-3 flex gap-2">
              <span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800">"Dinner"</span>
              <span class="rounded-full bg-green-100 px-2 py-1 text-xs font-semibold text-green-800">"750 kcal"</span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Delicious homemade pizza with fresh mozzarella and basil. Perfect crispy crust!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-300">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
            </div>
          </div>

          <div class="rounded-2xl border border-slate-200/80 bg-white/90 p-6 shadow-lg shadow-slate-200/50 transition hover:-translate-y-1 hover:shadow-xl dark:border-slate-800 dark:bg-slate-900/80 dark:shadow-black/40">
            <div class="mb-4 h-48 rounded bg-gradient-to-br from-green-400 to-emerald-500 flex items-center justify-center">
              <span class="text-6xl">"🥗"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Caesar Salad"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-300">"Date: 2025-12-02"</p>
            <div class="mb-3 flex gap-2">
              <span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800">"Lunch"</span>
              <span class="rounded-full bg-green-100 px-2 py-1 text-xs font-semibold text-green-800">"350 kcal"</span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Fresh romaine with grilled chicken, parmesan, and homemade dressing. Very satisfying!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-300">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐"</span>
            </div>
          </div>

          <div class="rounded-2xl border border-slate-200/80 bg-white/90 p-6 shadow-lg shadow-slate-200/50 transition hover:-translate-y-1 hover:shadow-xl dark:border-slate-800 dark:bg-slate-900/80 dark:shadow-black/40">
            <div class="mb-4 h-48 rounded bg-gradient-to-br from-red-400 to-rose-500 flex items-center justify-center">
              <span class="text-6xl">"🍝"</span>
            </div>
            <h3 class="mb-2 text-xl font-semibold text-slate-900 dark:text-slate-100">"Spaghetti Carbonara"</h3>
            <p class="mb-2 text-sm text-slate-600 dark:text-slate-300">"Date: 2025-12-03"</p>
            <div class="mb-3 flex gap-2">
              <span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-semibold text-blue-800">"Dinner"</span>
              <span class="rounded-full bg-green-100 px-2 py-1 text-xs font-semibold text-green-800">"650 kcal"</span>
            </div>
            <p class="text-slate-700 dark:text-slate-200">
              "Classic Italian pasta with eggs, pecorino, and guanciale. Rich and creamy!"
            </p>
            <div class="mt-4 flex items-center justify-between text-sm">
              <span class="text-slate-600 dark:text-slate-300">"Rating:"</span>
              <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
            </div>
          </div>
        </div>
      </div>
    }
}
