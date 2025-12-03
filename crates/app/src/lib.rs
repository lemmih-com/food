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

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment = move |_| *set_count.write() += 1;

    view! {
        <section class="mx-auto max-w-2xl space-y-6 py-12 text-center">
            <h1 class="text-4xl font-bold tracking-tight text-slate-900">
                "food.lemmih.com"
            </h1>
            <p class="text-slate-600">
                "Minimal Leptos skeleton rendered server-side on Cloudflare Workers."
            </p>
            <div class="flex items-center justify-center gap-4">
                <a
                    class="rounded bg-slate-900 px-4 py-2 font-semibold text-white hover:bg-slate-700"
                    href="/"
                >
                    "Refresh"
                </a>
                <button
                    class="rounded border border-slate-300 bg-white px-4 py-2 font-semibold text-slate-800 shadow-sm hover:bg-slate-50"
                    on:click=increment
                >
                    "Clicks: "
                    <span class="font-mono">{count}</span>
                </button>
            </div>
        </section>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main class="min-h-screen bg-slate-100 px-4">
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=Home />
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
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/styles.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <title>"food.lemmih.com"</title>
            </head>
            <body class="bg-slate-100 text-slate-900">
                <App/>
            </body>
        </html>
    }
}
