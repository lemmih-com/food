#![allow(non_snake_case)]
#![recursion_limit = "512"]

use axum::{routing::post, Router};
use console_error_panic_hook;
use console_log;
use food_lemmih_com_app::{shell, App, AuthState, SendKvStore};
use leptos::prelude::provide_context;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use leptos_config::LeptosOptions;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

// Register server functions at worker start
#[event(start)]
fn register() {
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminLogin>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminValidate>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminLogout>();
}

fn router(env: Env) -> Router<()> {
    let leptos_options = LeptosOptions::builder()
        .output_name("client")
        .site_pkg_dir("pkg")
        .build();
    let routes = generate_route_list(App);
    for route in routes.iter() {
        log::info!("Registering Leptos route {}", route.path());
    }

    // Get ADMIN_PIN from environment
    let admin_pin = env
        .secret("ADMIN_PIN")
        .ok()
        .map(|s| s.to_string())
        .unwrap_or_default();

    // Get KV namespace for auth tokens (wrapped for Send)
    let kv_store = env.kv("AUTH_TOKENS").expect("AUTH_TOKENS KV namespace not bound");
    let kv_store = SendKvStore::new(kv_store);

    let auth_state = AuthState::new(admin_pin);

    // Context provider function for server functions
    let provide_server_context = {
        let auth_state = auth_state.clone();
        let kv_store = kv_store.clone();
        move || {
            provide_context(auth_state.clone());
            provide_context(kv_store.clone());
        }
    };

    // Build the leptos routes with context provider for server functions
    Router::new()
        .route("/api/*fn_name", post({
            let provide_server_context = provide_server_context.clone();
            move |req| handle_server_fns_with_context(provide_server_context.clone(), req)
        }))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            provide_server_context,
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .with_state(leptos_options)
}

#[event(fetch)]
pub async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let method = req.method().clone();
    let path = req.uri().path().to_string();
    log::info!("fetch called for {} {}", method, path);

    Ok(router(env).call(req).await?)
}
