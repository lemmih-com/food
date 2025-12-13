#![allow(non_snake_case)]
#![recursion_limit = "512"]

use food_lemmih_com_app::App;
use leptos::mount;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount::hydrate_body(App);

    // Mark hydration as complete and clear the error timeout
    mark_hydration_complete();
}

/// Marks hydration as complete by setting a global flag and clearing the error timeout.
/// This is called after successful hydration to prevent the error banner from showing.
#[wasm_bindgen]
pub fn mark_hydration_complete() {
    use wasm_bindgen::JsValue;
    use web_sys::window;

    if let Some(window) = window() {
        // Set the hydration complete flag
        let _ = js_sys::Reflect::set(
            &window,
            &JsValue::from_str("__HYDRATION_COMPLETE__"),
            &JsValue::from_bool(true),
        );

        // Clear the error timeout if it exists
        if let Ok(timeout_id) =
            js_sys::Reflect::get(&window, &JsValue::from_str("__HYDRATION_TIMEOUT_ID__"))
        {
            if let Some(id) = timeout_id.as_f64() {
                window.clear_timeout_with_handle(id as i32);
            }
        }

        // Remove any existing error banner (in case of hot reload)
        if let Some(document) = window.document() {
            if let Some(banner) = document.get_element_by_id("hydration-error-banner") {
                banner.remove();
            }
        }

        log::info!("Hydration completed successfully");
    }
}
