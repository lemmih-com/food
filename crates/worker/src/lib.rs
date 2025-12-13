#![allow(non_snake_case)]
#![recursion_limit = "512"]

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use food_lemmih_com_app::{shell, App, AuthState, SendD1Database, SendKvStore, SendR2Bucket};
use leptos::prelude::provide_context;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use leptos_config::LeptosOptions;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

// Register server functions at worker start
#[event(start)]
fn register() {
    // Auth server functions
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminLogin>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminValidate>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::AdminLogout>();
    // Ingredient server functions
    server_fn::axum::register_explicit::<food_lemmih_com_app::GetIngredients>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::CreateIngredient>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::UpdateIngredient>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::DeleteIngredient>();
    // Recipe server functions
    server_fn::axum::register_explicit::<food_lemmih_com_app::GetRecipes>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::CreateRecipe>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::UpdateRecipe>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::DeleteRecipe>();
    // Food log server functions
    server_fn::axum::register_explicit::<food_lemmih_com_app::GetFoodLogs>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::CreateFoodLog>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::UpdateFoodLog>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::DeleteFoodLog>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::UploadFoodImage>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::GetFoodImage>();
    server_fn::axum::register_explicit::<food_lemmih_com_app::DeleteFoodImage>();
}

/// Handler to serve images from R2 bucket
async fn serve_food_image(
    Path(key): Path<String>,
    axum::Extension(bucket): axum::Extension<SendR2Bucket>,
) -> impl IntoResponse {
    use send_wrapper::SendWrapper;

    // Fetch the image from R2 - do all R2 work in a single SendWrapper block
    let result: std::result::Result<Option<(Vec<u8>, String)>, worker::Error> =
        SendWrapper::new(async {
            let object = match bucket.inner().get(&key).execute().await? {
                Some(obj) => obj,
                None => return Ok(None),
            };

            let content_type = object
                .http_metadata()
                .content_type
                .unwrap_or_else(|| "image/jpeg".to_string());

            let body = match object.body() {
                Some(b) => b,
                None => return Ok(None),
            };

            let bytes = body.bytes().await?;
            Ok(Some((bytes, content_type)))
        })
        .await;

    match result {
        Ok(Some((bytes, content_type))) => (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, content_type),
                (
                    header::CACHE_CONTROL,
                    "public, max-age=31536000".to_string(),
                ),
            ],
            bytes,
        )
            .into_response(),
        Ok(None) => {
            log::warn!("Image not found: {}", key);
            StatusCode::NOT_FOUND.into_response()
        }
        Err(e) => {
            log::error!("R2 error fetching image {}: {:?}", key, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn router(env: Env) -> Router<()> {
    let leptos_options = LeptosOptions::builder()
        .output_name("client")
        .site_pkg_dir("pkg")
        .build();
    let routes = generate_route_list(App);

    // Get ADMIN_PIN from environment (try secret first, then var for dev/e2e)
    let admin_pin = env
        .secret("ADMIN_PIN")
        .ok()
        .map(|s| s.to_string())
        .or_else(|| env.var("ADMIN_PIN").ok().map(|s| s.to_string()))
        .unwrap_or_default();

    // Get KV namespace for auth tokens (wrapped for Send)
    let kv_store = env
        .kv("AUTH_TOKENS")
        .expect("AUTH_TOKENS KV namespace not bound");
    let kv_store = SendKvStore::new(kv_store);

    // Get D1 database for ingredients (wrapped for Send)
    let d1_db = env
        .d1("INGREDIENTS_DB")
        .expect("INGREDIENTS_DB D1 database not bound");
    let d1_db = SendD1Database::new(d1_db);

    // Get R2 bucket for food images (wrapped for Send)
    let r2_bucket = env
        .bucket("FOOD_IMAGES")
        .expect("FOOD_IMAGES R2 bucket not bound");
    let r2_bucket = SendR2Bucket::new(r2_bucket);

    let auth_state = AuthState::new(admin_pin);

    // Context provider function for server functions
    let provide_server_context = {
        let auth_state = auth_state.clone();
        let kv_store = kv_store.clone();
        let d1_db = d1_db.clone();
        let r2_bucket = r2_bucket.clone();
        move || {
            provide_context(auth_state.clone());
            provide_context(kv_store.clone());
            provide_context(d1_db.clone());
            provide_context(r2_bucket.clone());
        }
    };

    // Build the leptos routes with context provider for server functions
    Router::new()
        .route("/api/food-image/{key}", get(serve_food_image))
        .route(
            "/api/{*fn_name}",
            post({
                let provide_server_context = provide_server_context.clone();
                move |req| handle_server_fns_with_context(provide_server_context.clone(), req)
            }),
        )
        .leptos_routes_with_context(&leptos_options, routes, provide_server_context, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(axum::Extension(r2_bucket))
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
