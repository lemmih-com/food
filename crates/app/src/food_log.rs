//! Food Log module
//!
//! Contains food log data structures, D1 database operations, R2 image storage,
//! and the Food Log page components with image cropping functionality.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use wasm_bindgen::JsCast;

use crate::auth::AdminAuth;
#[cfg(not(feature = "ssr"))]
use crate::cache::{get_cache, set_cache, FOOD_LOGS_CACHE_KEY, RECIPES_CACHE_KEY};
use crate::recipes::{get_recipes, Recipe};

// ============================================================================
// Data Types
// ============================================================================

/// Crop coordinates for an image (stored as percentages 0-100)
/// The crop represents which portion of the original image to show.
/// x, y are the top-left corner position as percentages.
/// width, height are the size of the crop area as percentages.
/// rotation is in degrees (0, 90, 180, 270).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImageCrop {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[serde(default)]
    pub rotation: i32, // 0, 90, 180, or 270 degrees
}

impl Default for ImageCrop {
    fn default() -> Self {
        Self::new()
    }
}

/// The aspect ratio used for displaying food log images (width/height)
/// This matches the card display which uses a wide rectangle
pub const CROP_ASPECT_RATIO: f32 = 2.5; // width is 2.5x height (roughly matching h-48 on typical card width)

impl ImageCrop {
    pub fn new() -> Self {
        // Default crop: centered, using the full width with appropriate height for aspect ratio
        // For a 2.5:1 aspect ratio on a square-ish image, height would be 40% of width
        // Start with a reasonably sized crop box
        Self {
            x: 10.0,
            y: 20.0,
            width: 80.0,
            height: 32.0, // 80 / 2.5 = 32
            rotation: 0,
        }
    }

    /// Create a crop that covers the full image (for backwards compatibility)
    pub fn full() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            rotation: 0,
        }
    }

    /// Rotate the crop by 90 degrees clockwise
    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 90) % 360;
    }

    /// Rotate the crop by 90 degrees counter-clockwise
    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 270) % 360;
    }

    /// Zoom in (decrease crop area while maintaining aspect ratio)
    pub fn zoom_in(&mut self) {
        let new_width = (self.width * 0.9).max(20.0); // minimum 20% width
        let new_height = new_width / CROP_ASPECT_RATIO;
        // Adjust position to keep centered
        let dx = (self.width - new_width) / 2.0;
        let dy = (self.height - new_height) / 2.0;
        self.x = (self.x + dx).clamp(0.0, 100.0 - new_width);
        self.y = (self.y + dy).clamp(0.0, 100.0 - new_height);
        self.width = new_width;
        self.height = new_height;
    }

    /// Zoom out (increase crop area while maintaining aspect ratio)
    pub fn zoom_out(&mut self) {
        let new_width = (self.width * 1.1).min(100.0);
        let new_height = (new_width / CROP_ASPECT_RATIO).min(100.0);
        // Adjust width if height would exceed 100%
        let new_width = if new_height >= 100.0 {
            100.0 * CROP_ASPECT_RATIO
        } else {
            new_width
        }
        .min(100.0);
        let new_height = new_width / CROP_ASPECT_RATIO;
        // Adjust position to keep centered and within bounds
        let dx = (self.width - new_width) / 2.0;
        let dy = (self.height - new_height) / 2.0;
        self.x = (self.x + dx).clamp(0.0, 100.0 - new_width);
        self.y = (self.y + dy).clamp(0.0, 100.0 - new_height);
        self.width = new_width;
        self.height = new_height;
    }
}

/// A food log entry
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FoodLog {
    pub id: Option<i64>,
    pub recipe_id: Option<i64>,
    pub recipe_name: Option<String>,
    pub image_key: Option<String>,
    pub logged_at: String,
    pub rating: Option<i32>,
    pub notes: String,
    pub crop: ImageCrop,
}

impl FoodLog {
    pub fn new_empty() -> Self {
        Self {
            id: None,
            recipe_id: None,
            recipe_name: None,
            image_key: None,
            logged_at: String::new(),
            rating: None,
            notes: String::new(),
            crop: ImageCrop::new(),
        }
    }

    /// Get the image URL for display (if image exists)
    pub fn image_url(&self) -> Option<String> {
        self.image_key
            .as_ref()
            .map(|key| format!("/api/food-image/{}", key))
    }
}

impl Default for FoodLog {
    fn default() -> Self {
        Self::new_empty()
    }
}

// ============================================================================
// R2 Bucket Wrapper (SSR only)
// ============================================================================

/// Wrapper around CloudFlare R2 bucket that implements Send + Sync
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct SendR2Bucket(std::sync::Arc<send_wrapper::SendWrapper<worker::Bucket>>);

#[cfg(feature = "ssr")]
impl SendR2Bucket {
    pub fn new(bucket: worker::Bucket) -> Self {
        Self(std::sync::Arc::new(send_wrapper::SendWrapper::new(bucket)))
    }

    pub fn inner(&self) -> &worker::Bucket {
        &self.0
    }
}

// ============================================================================
// Server Functions
// ============================================================================

#[cfg(feature = "ssr")]
use crate::ingredients::SendD1Database;

/// Fetch all food logs from D1 database
#[server]
pub async fn get_food_logs() -> Result<Vec<FoodLog>, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let logs = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "SELECT fl.id, fl.recipe_id, r.name as recipe_name, fl.image_key, fl.logged_at, 
                    fl.rating, fl.notes, fl.crop_x, fl.crop_y, fl.crop_width, fl.crop_height, fl.crop_rotation
             FROM food_logs fl
             LEFT JOIN recipes r ON fl.recipe_id = r.id
             ORDER BY fl.logged_at DESC",
        );
        let results = stmt.all().await?;
        let rows: Vec<serde_json::Value> = results.results::<serde_json::Value>()?;

        let logs: Vec<FoodLog> = rows
            .into_iter()
            .map(|row| FoodLog {
                id: row.get("id").and_then(|v| v.as_i64()),
                recipe_id: row.get("recipe_id").and_then(|v| v.as_i64()),
                recipe_name: row
                    .get("recipe_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                image_key: row
                    .get("image_key")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                logged_at: row
                    .get("logged_at")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                rating: row.get("rating").and_then(|v| v.as_i64()).map(|r| r as i32),
                notes: row
                    .get("notes")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                crop: ImageCrop {
                    x: row.get("crop_x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    y: row.get("crop_y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    width: row
                        .get("crop_width")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(100.0) as f32,
                    height: row
                        .get("crop_height")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(100.0) as f32,
                    rotation: row
                        .get("crop_rotation")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0) as i32,
                },
            })
            .collect();

        Ok::<_, worker::Error>(logs)
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

    Ok(logs)
}

/// Create a new food log entry
#[server]
pub async fn create_food_log(log: FoodLog) -> Result<FoodLog, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let result = SendWrapper::new(async {
        // Build SQL with proper NULL handling using raw SQL for simplicity
        let recipe_id_sql = log
            .recipe_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "NULL".to_string());
        let rating_sql = log
            .rating
            .map(|r| r.to_string())
            .unwrap_or_else(|| "NULL".to_string());
        let image_key_sql = log
            .image_key
            .as_ref()
            .map(|k| format!("'{}'", k.replace('\'', "''")))
            .unwrap_or_else(|| "NULL".to_string());

        let sql = format!(
            "INSERT INTO food_logs (recipe_id, image_key, logged_at, rating, notes, crop_x, crop_y, crop_width, crop_height, crop_rotation) 
             VALUES ({}, {}, '{}', {}, '{}', {}, {}, {}, {}, {}) RETURNING id",
            recipe_id_sql,
            image_key_sql,
            log.logged_at.replace('\'', "''"),
            rating_sql,
            log.notes.replace('\'', "''"),
            log.crop.x,
            log.crop.y,
            log.crop.width,
            log.crop.height,
            log.crop.rotation
        );

        let stmt = db.inner().prepare(&sql);
        stmt.first::<serde_json::Value>(None).await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;

    let id = result
        .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
        .ok_or_else(|| ServerFnError::new("Failed to get inserted ID"))?;

    log::info!("Created food log entry: id={}", id);

    Ok(FoodLog {
        id: Some(id),
        ..log
    })
}

/// Update an existing food log entry
#[server]
pub async fn update_food_log(log: FoodLog) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let id = log
        .id
        .ok_or_else(|| ServerFnError::new("Food log ID is required for update"))?;

    SendWrapper::new(async {
        let recipe_id_sql = log
            .recipe_id
            .map(|rid| rid.to_string())
            .unwrap_or_else(|| "NULL".to_string());
        let rating_sql = log
            .rating
            .map(|r| r.to_string())
            .unwrap_or_else(|| "NULL".to_string());
        let image_key_sql = log
            .image_key
            .as_ref()
            .map(|k| format!("'{}'", k.replace('\'', "''")))
            .unwrap_or_else(|| "NULL".to_string());

        let sql = format!(
            "UPDATE food_logs SET recipe_id = {}, image_key = {}, logged_at = '{}', rating = {}, notes = '{}', 
             crop_x = {}, crop_y = {}, crop_width = {}, crop_height = {}, crop_rotation = {}, updated_at = datetime('now') 
             WHERE id = {}",
            recipe_id_sql,
            image_key_sql,
            log.logged_at.replace('\'', "''"),
            rating_sql,
            log.notes.replace('\'', "''"),
            log.crop.x,
            log.crop.y,
            log.crop.width,
            log.crop.height,
            log.crop.rotation,
            id
        );

        let stmt = db.inner().prepare(&sql);
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 update error: {:?}", e)))?;

    log::info!("Updated food log entry: id={}", id);
    Ok(())
}

/// Delete a food log entry
#[server]
pub async fn delete_food_log(id: i64) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();
    let bucket = expect_context::<SendR2Bucket>();

    // First get the image key to delete from R2
    let image_key: Option<String> = SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare(format!("SELECT image_key FROM food_logs WHERE id = {}", id));
        let result = stmt.first::<serde_json::Value>(None).await?;
        Ok::<_, worker::Error>(result.and_then(|v| {
            v.get("image_key")
                .and_then(|k| k.as_str().map(|s| s.to_string()))
        }))
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

    // Delete from R2 if there's an image
    if let Some(key) = image_key {
        let _ = SendWrapper::new(bucket.inner().delete(&key)).await;
    }

    // Delete from D1
    SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare(format!("DELETE FROM food_logs WHERE id = {}", id));
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete error: {:?}", e)))?;

    log::info!("Deleted food log entry: id={}", id);
    Ok(())
}

/// Upload an image to R2 using base64 encoded data and return the key
#[server]
pub async fn upload_food_image(
    image_base64: String,
    content_type: String,
) -> Result<String, ServerFnError> {
    use base64::Engine;
    use send_wrapper::SendWrapper;

    let bucket = expect_context::<SendR2Bucket>();

    // Decode base64 data
    let data = base64::engine::general_purpose::STANDARD
        .decode(&image_base64)
        .map_err(|e| ServerFnError::new(format!("Failed to decode image: {}", e)))?;

    // Generate a unique key
    let mut key_bytes = [0u8; 16];
    getrandom::fill(&mut key_bytes).expect("Failed to generate random bytes");
    let key = hex::encode(key_bytes);

    // Upload to R2
    SendWrapper::new(async {
        bucket
            .inner()
            .put(&key, data)
            .http_metadata(worker::HttpMetadata {
                content_type: Some(content_type),
                ..Default::default()
            })
            .execute()
            .await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("R2 upload error: {:?}", e)))?;

    log::info!("Uploaded image to R2: key={}", key);
    Ok(key)
}

/// Delete an image from R2
#[server]
pub async fn delete_food_image(key: String) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let bucket = expect_context::<SendR2Bucket>();

    SendWrapper::new(bucket.inner().delete(&key))
        .await
        .map_err(|e| ServerFnError::new(format!("R2 delete error: {:?}", e)))?;

    log::info!("Deleted image from R2: key={}", key);
    Ok(())
}

/// Get an image from R2 (returns base64 encoded data with content type)
#[server]
pub async fn get_food_image(key: String) -> Result<(String, String), ServerFnError> {
    use base64::Engine;
    use send_wrapper::SendWrapper;

    let bucket = expect_context::<SendR2Bucket>();

    // Wrap the entire operation in SendWrapper to handle non-Send types
    let (body_bytes, content_type) = SendWrapper::new(async {
        let object = bucket
            .inner()
            .get(&key)
            .execute()
            .await?
            .ok_or_else(|| worker::Error::JsError("Image not found".to_string()))?;

        let content_type = object
            .http_metadata()
            .content_type
            .unwrap_or_else(|| "image/jpeg".to_string());

        let body_bytes = object
            .body()
            .ok_or_else(|| worker::Error::JsError("No body".to_string()))?
            .bytes()
            .await?;

        Ok::<_, worker::Error>((body_bytes, content_type))
    })
    .await
    .map_err(|e| ServerFnError::new(format!("R2 get error: {:?}", e)))?;

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&body_bytes);

    Ok((base64_data, content_type))
}

// ============================================================================
// Components
// ============================================================================

/// Star rating component
#[component]
fn StarRating(rating: RwSignal<Option<i32>>, #[prop(optional)] readonly: bool) -> impl IntoView {
    let stars = [1, 2, 3, 4, 5];

    view! {
      <div class="flex items-center gap-1">
        {stars
          .into_iter()
          .map(|star| {
            let is_filled = move || rating.get().map(|r| r >= star).unwrap_or(false);
            view! {
              <button
                type="button"
                class=move || {
                  let base = "w-6 h-6 transition-colors";
                  if readonly {
                    format!("{} cursor-default", base)
                  } else {
                    format!("{} cursor-pointer hover:scale-110", base)
                  }
                }
                disabled=readonly
                on:click=move |_| {
                  if !readonly {
                    let current = rating.get();
                    if current == Some(star) {
                      rating.set(None);
                    } else {
                      rating.set(Some(star));
                    }
                  }
                }
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  fill=move || if is_filled() { "currentColor" } else { "none" }
                  stroke="currentColor"
                  stroke-width="1.5"
                  class=move || if is_filled() { "text-yellow-400" } else { "text-slate-300 dark:text-slate-600" }
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
                  />
                </svg>
              </button>
            }
          })
          .collect_view()}
      </div>
    }
}

/// Image cropper component with fixed aspect ratio crop box, rotation, and zoom
#[component]
fn ImageCropper(
    image_data: ReadSignal<Option<String>>,
    crop: RwSignal<ImageCrop>,
) -> impl IntoView {
    let container_ref = NodeRef::<leptos::html::Div>::new();
    let is_dragging = RwSignal::new(false);
    let drag_start = RwSignal::new((0.0_f32, 0.0_f32));
    let crop_start = RwSignal::new(ImageCrop::new());

    let on_mouse_down = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        is_dragging.set(true);
        drag_start.set((ev.client_x() as f32, ev.client_y() as f32));
        crop_start.set(crop.get());
    };

    let on_mouse_move = move |ev: web_sys::MouseEvent| {
        if !is_dragging.get() {
            return;
        }
        let Some(container) = container_ref.get() else {
            return;
        };
        let rect = container.get_bounding_client_rect();
        let container_width = rect.width() as f32;
        let container_height = rect.height() as f32;

        let (start_x, start_y) = drag_start.get();
        let dx = ((ev.client_x() as f32 - start_x) / container_width) * 100.0;
        let dy = ((ev.client_y() as f32 - start_y) / container_height) * 100.0;

        let start_crop = crop_start.get();
        let new_x = (start_crop.x + dx).clamp(0.0, 100.0 - start_crop.width);
        let new_y = (start_crop.y + dy).clamp(0.0, 100.0 - start_crop.height);

        crop.update(|c| {
            c.x = new_x;
            c.y = new_y;
        });
    };

    let on_mouse_up = move |_: web_sys::MouseEvent| {
        is_dragging.set(false);
    };

    let on_rotate_ccw = move |_| {
        crop.update(|c| c.rotate_ccw());
    };

    let on_rotate_cw = move |_| {
        crop.update(|c| c.rotate_cw());
    };

    let on_zoom_in = move |_| {
        crop.update(|c| c.zoom_in());
    };

    let on_zoom_out = move |_| {
        crop.update(|c| c.zoom_out());
    };

    view! {
      <Show when=move || image_data.get().is_some()>
        <div class="space-y-3">
          // Control buttons
          <div class="flex items-center justify-between">
            <p class="text-xs text-slate-500 dark:text-slate-400">"Drag box to pan, use buttons to rotate/zoom"</p>
            <div class="flex items-center gap-1">
              // Rotate counter-clockwise
              <button
                type="button"
                class="p-2 rounded bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300"
                title="Rotate left"
                on:click=on_rotate_ccw
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M3 10h4V6M3 10l3.5-3.5a8 8 0 1111 11"
                  />
                </svg>
              </button>
              // Rotate clockwise
              <button
                type="button"
                class="p-2 rounded bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300"
                title="Rotate right"
                on:click=on_rotate_cw
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 10h-4V6M21 10l-3.5-3.5a8 8 0 10-11 11"
                  />
                </svg>
              </button>
              <div class="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-1" />
              // Zoom out
              <button
                type="button"
                class="p-2 rounded bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300"
                title="Zoom out (larger selection)"
                on:click=on_zoom_out
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM13 10H7"
                  />
                </svg>
              </button>
              // Zoom in
              <button
                type="button"
                class="p-2 rounded bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-700 dark:text-slate-300"
                title="Zoom in (smaller selection)"
                on:click=on_zoom_in
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0zM10 7v6m3-3H7"
                  />
                </svg>
              </button>
            </div>
          </div>
          // Image container
          <div
            node_ref=container_ref
            class="relative w-full bg-slate-900 rounded overflow-hidden select-none"
            style="aspect-ratio: 4/3;"
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mouseleave=on_mouse_up
          >
            // The image fills the container with rotation applied
            <img
              src=move || image_data.get().unwrap_or_default()
              class="absolute inset-0 w-full h-full object-contain transition-transform duration-200"
              style=move || format!("transform: rotate({}deg);", crop.get().rotation)
              draggable="false"
            />
            // Top dark band
            <div
              class="absolute left-0 right-0 top-0 bg-black/60 pointer-events-none"
              style=move || format!("height: {}%;", crop.get().y)
            />
            // Bottom dark band
            <div
              class="absolute left-0 right-0 bottom-0 bg-black/60 pointer-events-none"
              style=move || {
                let c = crop.get();
                format!("height: {}%;", 100.0 - c.y - c.height)
              }
            />
            // Left dark band (between top and bottom bands)
            <div
              class="absolute left-0 bg-black/60 pointer-events-none"
              style=move || {
                let c = crop.get();
                format!("top: {}%; height: {}%; width: {}%;", c.y, c.height, c.x)
              }
            />
            // Right dark band (between top and bottom bands)
            <div
              class="absolute right-0 bg-black/60 pointer-events-none"
              style=move || {
                let c = crop.get();
                format!("top: {}%; height: {}%; width: {}%;", c.y, c.height, 100.0 - c.x - c.width)
              }
            />
            // Crop box border (draggable)
            <div
              class="absolute border-2 border-white shadow-lg cursor-move"
              style=move || {
                let c = crop.get();
                format!("left: {}%; top: {}%; width: {}%; height: {}%;", c.x, c.y, c.width, c.height)
              }
              on:mousedown=on_mouse_down
            >
              // Corner indicators
              <div class="absolute -top-1 -left-1 w-3 h-3 bg-white rounded-sm" />
              <div class="absolute -top-1 -right-1 w-3 h-3 bg-white rounded-sm" />
              <div class="absolute -bottom-1 -left-1 w-3 h-3 bg-white rounded-sm" />
              <div class="absolute -bottom-1 -right-1 w-3 h-3 bg-white rounded-sm" />
            </div>
          </div>
        </div>
      </Show>
    }
}

/// Food log entry modal
#[component]
fn FoodLogModal(
    show: RwSignal<bool>,
    editing: RwSignal<Option<FoodLog>>,
    available_recipes: ReadSignal<Vec<Recipe>>,
    on_save: impl Fn() + Clone + Send + Sync + 'static,
    on_delete: impl Fn(i64) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let recipe_id = RwSignal::new(Option::<i64>::None);
    let logged_at = RwSignal::new(String::new());
    let rating = RwSignal::new(Option::<i32>::None);
    let notes = RwSignal::new(String::new());
    let image_key = RwSignal::new(Option::<String>::None);
    let image_data = RwSignal::new(Option::<String>::None);
    let crop = RwSignal::new(ImageCrop::new());
    let error = RwSignal::new(Option::<String>::None);
    let saving = RwSignal::new(false);
    let show_delete_confirm = RwSignal::new(false);
    let trigger_delete = RwSignal::new(false);

    let file_input_ref = NodeRef::<leptos::html::Input>::new();

    {
        let on_delete = on_delete.clone();
        Effect::new(move || {
            if trigger_delete.get() {
                trigger_delete.set(false);
                if let Some(log) = editing.get() {
                    if let Some(id) = log.id {
                        saving.set(true);
                        let on_delete = on_delete.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match delete_food_log(id).await {
                                Ok(()) => {
                                    show.set(false);
                                    editing.set(None);
                                    on_delete(id);
                                }
                                Err(e) => {
                                    error.set(Some(format!("Failed to delete: {}", e)));
                                }
                            }
                            saving.set(false);
                        });
                    }
                }
            }
        });
    }

    Effect::new(move || {
        if let Some(log) = editing.get() {
            recipe_id.set(log.recipe_id);
            logged_at.set(log.logged_at.clone());
            rating.set(log.rating);
            notes.set(log.notes.clone());
            image_key.set(log.image_key.clone());
            crop.set(log.crop.clone());
            if let Some(url) = log.image_url() {
                image_data.set(Some(url));
            } else {
                image_data.set(None);
            }
        } else {
            recipe_id.set(None);
            let today = js_sys::Date::new_0();
            let year = today.get_full_year();
            let month = today.get_month() + 1;
            let day = today.get_date();
            logged_at.set(format!("{:04}-{:02}-{:02}", year, month, day));
            rating.set(None);
            notes.set(String::new());
            image_key.set(None);
            image_data.set(None);
            crop.set(ImageCrop::new());
        }
        error.set(None);
        show_delete_confirm.set(false);
    });

    let close = move || {
        show.set(false);
        editing.set(None);
    };

    let handle_file_select = move |_| {
        let Some(input) = file_input_ref.get() else {
            return;
        };
        let Some(files) = input.files() else { return };
        if files.length() == 0 {
            return;
        }

        let file = files.get(0).unwrap();
        let reader = web_sys::FileReader::new().unwrap();

        let reader_clone = reader.clone();
        let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Ok(result) = reader_clone.result() {
                if let Some(data_url) = result.as_string() {
                    image_data.set(Some(data_url));
                    crop.set(ImageCrop::new());
                    image_key.set(None);
                }
            }
        }) as Box<dyn FnMut(_)>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        let _ = reader.read_as_data_url(&file);
    };

    let handle_save = {
        let on_save = on_save.clone();
        move || {
            let logged_at_val = logged_at.get();
            if logged_at_val.trim().is_empty() {
                error.set(Some("Date is required".to_string()));
                return;
            }

            saving.set(true);
            let on_save = on_save.clone();
            let current_image_data = image_data.get();
            let current_image_key = image_key.get();
            let needs_upload = current_image_data.is_some()
                && current_image_key.is_none()
                && current_image_data
                    .as_ref()
                    .map(|d| d.starts_with("data:"))
                    .unwrap_or(false);

            wasm_bindgen_futures::spawn_local(async move {
                let final_image_key = if needs_upload {
                    let data_url = image_data.get().unwrap();
                    let parts: Vec<&str> = data_url.splitn(2, ',').collect();
                    if parts.len() != 2 {
                        error.set(Some("Invalid image data".to_string()));
                        saving.set(false);
                        return;
                    }
                    let base64_data = parts[1].to_string();
                    let content_type = if parts[0].contains("png") {
                        "image/png"
                    } else if parts[0].contains("gif") {
                        "image/gif"
                    } else if parts[0].contains("webp") {
                        "image/webp"
                    } else {
                        "image/jpeg"
                    };

                    match upload_food_image(base64_data, content_type.to_string()).await {
                        Ok(key) => Some(key),
                        Err(e) => {
                            error.set(Some(format!("Failed to upload image: {}", e)));
                            saving.set(false);
                            return;
                        }
                    }
                } else {
                    current_image_key
                };

                let log = FoodLog {
                    id: editing.get().and_then(|e| e.id),
                    recipe_id: recipe_id.get(),
                    recipe_name: None,
                    image_key: final_image_key,
                    logged_at: logged_at.get(),
                    rating: rating.get(),
                    notes: notes.get(),
                    crop: crop.get(),
                };

                let result = if log.id.is_some() {
                    update_food_log(log).await.map(|_| ())
                } else {
                    create_food_log(log).await.map(|_| ())
                };

                saving.set(false);
                match result {
                    Ok(()) => {
                        show.set(false);
                        editing.set(None);
                        on_save();
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to save: {}", e)));
                    }
                }
            });
        }
    };

    let input_class = "w-full rounded border border-slate-300 dark:border-slate-600 px-3 py-2 text-sm bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500";
    let label_class = "block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1";

    view! {
      <Show when=move || show.get()>
        <div
          id="food-log-modal-backdrop"
          class="fixed inset-0 z-50 flex items-start justify-center bg-black/50 overflow-y-auto py-4"
          on:click=move |ev: web_sys::MouseEvent| {
            if let Some(target) = ev.target() {
              if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.id() == "food-log-modal-backdrop" {
                  close();
                }
              }
            }
          }
        >
          <div class="w-full max-w-2xl rounded-lg bg-white dark:bg-slate-800 p-6 shadow-xl mx-4 my-auto">
            <div class="mb-4 flex items-center justify-between">
              <h2 class="text-xl font-bold text-slate-900 dark:text-slate-100">
                {move || if editing.get().is_some() { "Edit Food Log Entry" } else { "New Food Log Entry" }}
              </h2>
              <button
                class="text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200"
                on:click=move |_| close()
              >
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            <Show when=move || error.get().is_some()>
              <div class="mb-4 rounded bg-red-100 dark:bg-red-900/30 px-4 py-2 text-sm text-red-700 dark:text-red-400">
                {move || error.get().unwrap_or_default()}
              </div>
            </Show>

            <div class="space-y-4 max-h-[70vh] overflow-y-auto pr-2">
              <div>
                <label class=label_class>"Photo"</label>
                <div class="space-y-2">
                  <input
                    node_ref=file_input_ref
                    type="file"
                    accept="image/*"
                    class="hidden"
                    on:change=handle_file_select
                  />
                  <button
                    type="button"
                    class="w-full rounded border-2 border-dashed border-slate-300 dark:border-slate-600 px-4 py-8 text-center text-slate-500 dark:text-slate-400 hover:border-blue-500 hover:text-blue-500 transition-colors"
                    on:click=move |_| {
                      if let Some(input) = file_input_ref.get() {
                        input.click();
                      }
                    }
                  >
                    <Show
                      when=move || image_data.get().is_some()
                      fallback=move || {
                        view! {
                          <svg class="mx-auto h-12 w-12 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            />
                          </svg>
                          <span>"Click to upload a photo"</span>
                        }
                      }
                    >
                      <span>"Click to change photo"</span>
                    </Show>
                  </button>
                  <ImageCropper image_data=image_data.read_only() crop=crop />
                </div>
              </div>

              <div>
                <label class=label_class>"Recipe (optional)"</label>
                <select
                  class=input_class
                  on:change=move |ev| {
                    let value = event_target_value(&ev);
                    if value.is_empty() {
                      recipe_id.set(None);
                    } else if let Ok(id) = value.parse::<i64>() {
                      recipe_id.set(Some(id));
                    }
                  }
                >
                  <option value="" selected=move || recipe_id.get().is_none()>
                    "-- No recipe --"
                  </option>
                  <For
                    each=move || available_recipes.get()
                    key=|r| r.id.unwrap_or(0)
                    children=move |r: Recipe| {
                      let rid = r.id.unwrap_or(0);
                      view! {
                        <option value=rid.to_string() selected=move || recipe_id.get() == Some(rid)>
                          {r.name}
                        </option>
                      }
                    }
                  />
                </select>
              </div>

              <div>
                <label class=label_class>"Date"</label>
                <input
                  type="date"
                  class=input_class
                  prop:value=move || logged_at.get()
                  on:input=move |ev| logged_at.set(event_target_value(&ev))
                />
              </div>

              <div>
                <label class=label_class>"Rating"</label>
                <StarRating rating=rating />
              </div>

              <div>
                <label class=label_class>"Notes"</label>
                <textarea
                  class=input_class
                  rows="3"
                  prop:value=move || notes.get()
                  on:input=move |ev| notes.set(event_target_value(&ev))
                  placeholder="How was this meal?"
                />
              </div>
            </div>

            <div class="mt-6 flex justify-between gap-3 border-t border-slate-200 dark:border-slate-600 pt-4">
              <div>
                <Show when=move || editing.get().and_then(|l| l.id).is_some()>
                  <Show
                    when=move || show_delete_confirm.get()
                    fallback=move || {
                      view! {
                        <button
                          class="rounded bg-red-100 dark:bg-red-900/30 px-4 py-2 font-medium text-red-700 dark:text-red-400 hover:bg-red-200 dark:hover:bg-red-900/50"
                          on:click=move |_| show_delete_confirm.set(true)
                        >
                          "Delete"
                        </button>
                      }
                    }
                  >
                    <div class="flex items-center gap-2">
                      <span class="text-sm text-red-700 dark:text-red-400">"Are you sure?"</span>
                      <button
                        class="rounded bg-red-600 px-3 py-1 text-sm font-medium text-white hover:bg-red-700"
                        on:click=move |_| trigger_delete.set(true)
                      >
                        "Yes, delete"
                      </button>
                      <button
                        class="rounded bg-slate-200 dark:bg-slate-600 px-3 py-1 text-sm font-medium text-slate-700 dark:text-slate-200 hover:bg-slate-300 dark:hover:bg-slate-500"
                        on:click=move |_| show_delete_confirm.set(false)
                      >
                        "Cancel"
                      </button>
                    </div>
                  </Show>
                </Show>
              </div>
              <div class="flex gap-3">
                <button
                  class="rounded bg-slate-200 dark:bg-slate-600 px-4 py-2 font-medium text-slate-700 dark:text-slate-200 hover:bg-slate-300 dark:hover:bg-slate-500"
                  on:click=move |_| close()
                >
                  "Cancel"
                </button>
                <button
                  class="rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300 dark:disabled:bg-blue-800"
                  disabled=move || saving.get()
                  on:click={
                    let handle_save = handle_save.clone();
                    move |_| handle_save()
                  }
                >
                  {move || if saving.get() { "Saving..." } else { "Save" }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </Show>
    }
}

/// Food log card component
#[component]
fn FoodLogCard(
    log: FoodLog,
    on_edit: impl Fn(FoodLog) + Clone + Send + Sync + 'static,
    is_authenticated: ReadSignal<bool>,
) -> impl IntoView {
    let log_for_edit = log.clone();
    let has_image = log.image_key.is_some();
    let image_url = log.image_url().unwrap_or_default();
    let crop_x = log.crop.x;
    let crop_y = log.crop.y;
    let crop_width = log.crop.width;
    let crop_height = log.crop.height;
    let rotation = log.crop.rotation;
    let recipe_name = log.recipe_name.clone();
    let has_recipe = recipe_name.is_some();
    let logged_at = log.logged_at.clone();
    let rating = log.rating;
    let has_rating = rating.is_some();
    let notes = log.notes.clone();
    let has_notes = !notes.is_empty();

    // Calculate scale factor - image should be scaled so crop area fills the container
    let scale = 100.0 / crop_width.min(crop_height);
    // Calculate position to center the crop area
    let pos_x = if (100.0 - crop_width).abs() < 0.01 {
        0.0
    } else {
        (crop_x / (100.0 - crop_width)) * 100.0
    };
    let pos_y = if (100.0 - crop_height).abs() < 0.01 {
        0.0
    } else {
        (crop_y / (100.0 - crop_height)) * 100.0
    };

    view! {
      <div class="rounded-lg bg-white dark:bg-slate-800 shadow-md overflow-hidden">
        <Show when=move || has_image>
          <div class="h-48 bg-slate-200 dark:bg-slate-700 overflow-hidden">
            <img
              src=image_url.clone()
              class="w-full h-full object-cover"
              style=format!(
                "transform: rotate({}deg) scale({}); transform-origin: {}% {}%; object-position: {}% {}%;",
                rotation,
                scale / 100.0,
                50.0,
                50.0,
                pos_x,
                pos_y,
              )
            />
          </div>
        </Show>

        <div class="p-4">
          <div class="flex items-start justify-between mb-2">
            <div>
              <Show when=move || has_recipe>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
                  {recipe_name.clone().unwrap_or_default()}
                </h3>
              </Show>
              <p class="text-sm text-slate-500 dark:text-slate-400">{logged_at.clone()}</p>
            </div>
            <Show when=move || is_authenticated.get()>
              <button
                class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 p-1"
                title="Edit entry"
                on:click={
                  let log_for_edit = log_for_edit.clone();
                  let on_edit = on_edit.clone();
                  move |_| on_edit(log_for_edit.clone())
                }
              >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                  />
                </svg>
              </button>
            </Show>
          </div>

          <Show when=move || has_rating>
            <div class="flex items-center gap-1 mb-2">
              {(1..=5)
                .map(|star| {
                  let filled = rating.map(|r| r >= star).unwrap_or(false);
                  view! {
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      viewBox="0 0 24 24"
                      fill=if filled { "currentColor" } else { "none" }
                      stroke="currentColor"
                      stroke-width="1.5"
                      class=format!(
                        "w-5 h-5 {}",
                        if filled { "text-yellow-400" } else { "text-slate-300 dark:text-slate-600" },
                      )
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
                      />
                    </svg>
                  }
                })
                .collect_view()}
            </div>
          </Show>

          <Show when=move || has_notes>
            <p class="text-slate-700 dark:text-slate-300 text-sm">{notes.clone()}</p>
          </Show>
        </div>
      </div>
    }
}

/// Main Food Log page component
#[component]
pub fn FoodLogs() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let show_modal = RwSignal::new(false);
    let editing_log = RwSignal::new(Option::<FoodLog>::None);

    let cached_logs = RwSignal::new(Option::<Vec<FoodLog>>::None);
    let cached_recipes = RwSignal::new(Option::<Vec<Recipe>>::None);

    #[cfg(not(feature = "ssr"))]
    {
        if let Some(cached) = get_cache::<Vec<FoodLog>>(FOOD_LOGS_CACHE_KEY) {
            cached_logs.set(Some(cached));
        }
        if let Some(cached) = get_cache::<Vec<Recipe>>(RECIPES_CACHE_KEY) {
            cached_recipes.set(Some(cached));
        }
    }

    let logs_resource = Resource::new(|| (), |_| get_food_logs());
    let recipes_resource = Resource::new(|| (), |_| get_recipes());

    #[cfg(not(feature = "ssr"))]
    Effect::new(move || {
        if let Some(Ok(logs)) = logs_resource.get() {
            set_cache(FOOD_LOGS_CACHE_KEY, &logs);
        }
    });

    #[cfg(not(feature = "ssr"))]
    Effect::new(move || {
        if let Some(Ok(recipes)) = recipes_resource.get() {
            set_cache(RECIPES_CACHE_KEY, &recipes);
        }
    });

    let refetch = move || {
        logs_resource.refetch();
    };

    let handle_delete = move |_id: i64| {
        logs_resource.refetch();
    };

    let handle_new = move |_| {
        editing_log.set(None);
        show_modal.set(true);
    };

    let handle_edit = move |log: FoodLog| {
        editing_log.set(Some(log));
        show_modal.set(true);
    };

    view! {
      <div class="mx-auto max-w-7xl py-6">
        <div class="mb-6 flex items-center justify-between flex-wrap gap-4">
          <h2 class="text-3xl font-bold text-slate-900 dark:text-slate-100">"Food Log"</h2>
          <Show when=move || auth.is_authenticated.get()>
            <button
              class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
              on:click=handle_new
            >
              <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              "Log Meal"
            </button>
          </Show>
        </div>

        <Suspense fallback=move || {
          match (cached_logs.get(), cached_recipes.get()) {
            (Some(logs), Some(recipes)) if !logs.is_empty() => {
              let (recipes_signal, _) = signal(recipes);
              let is_auth = auth.is_authenticated;
              view! {
                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                  <For
                    each=move || logs.clone()
                    key=|log| log.id.unwrap_or(0)
                    children=move |log: FoodLog| {
                      let is_auth_signal = is_auth.read_only();
                      view! { <FoodLogCard log=log on_edit=handle_edit is_authenticated=is_auth_signal /> }
                    }
                  />
                </div>
                <FoodLogModal
                  show=show_modal
                  editing=editing_log
                  available_recipes=recipes_signal
                  on_save=refetch
                  on_delete=handle_delete
                />
              }
                .into_any()
            }
            _ => view! { <p class="text-slate-600 dark:text-slate-400">"Loading food log..."</p> }.into_any(),
          }
        }>
          {move || {
            let logs_result = logs_resource.get();
            let recipes_result = recipes_resource.get();
            match (logs_result, recipes_result) {
              (Some(Ok(logs)), Some(Ok(recipes))) => {
                let (recipes_signal, _) = signal(recipes);
                let is_auth = auth.is_authenticated;
                Some(
                  if logs.is_empty() {
                    view! {
                      <div class="text-center py-12">
                        <p class="text-slate-600 dark:text-slate-400 mb-4">"No food log entries yet."</p>
                        <Show when=move || auth.is_authenticated.get()>
                          <p class="text-slate-500 dark:text-slate-500 text-sm">
                            "Click \"Log Meal\" to add your first entry."
                          </p>
                        </Show>
                      </div>
                      <FoodLogModal
                        show=show_modal
                        editing=editing_log
                        available_recipes=recipes_signal
                        on_save=refetch
                        on_delete=handle_delete
                      />
                    }
                      .into_any()
                  } else {
                    view! {
                      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                        <For
                          each=move || logs.clone()
                          key=|log| log.id.unwrap_or(0)
                          children=move |log: FoodLog| {
                            let is_auth_signal = is_auth.read_only();
                            view! { <FoodLogCard log=log on_edit=handle_edit is_authenticated=is_auth_signal /> }
                          }
                        />
                      </div>
                      <FoodLogModal
                        show=show_modal
                        editing=editing_log
                        available_recipes=recipes_signal
                        on_save=refetch
                        on_delete=handle_delete
                      />
                    }
                      .into_any()
                  },
                )
              }
              (Some(Err(e)), _) | (_, Some(Err(e))) => {
                Some(
                  view! {
                    <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                      <p class="font-medium">"Failed to load data"</p>
                      <p class="text-sm">{e.to_string()}</p>
                    </div>
                  }
                    .into_any(),
                )
              }
              _ => None,
            }
          }}
        </Suspense>
      </div>
    }
}
