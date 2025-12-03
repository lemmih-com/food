#![allow(non_snake_case)]

use axum::{Extension, Router};
use console_error_panic_hook;
use console_log;
use food_lemmih_com_app::{shell, App};
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_config::LeptosOptions;
use std::sync::Arc;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

fn router(env: Env) -> Router<()> {
    let leptos_options = LeptosOptions::builder()
        .output_name("client")
        .site_pkg_dir("pkg")
        .build();
    let routes = generate_route_list(App);
    for route in routes.iter() {
        log::info!("Registering Leptos route {}", route.path());
    }

    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)))
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
