//! Ingredients module
//!
//! Contains ingredient data structures, D1 database operations, and the Ingredients page components.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use wasm_bindgen::JsCast;

use crate::auth::AdminAuth;

// ============================================================================
// Data Types
// ============================================================================

/// All nutrient values are per 100g
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ingredient {
    pub id: Option<i64>,
    pub name: String,
    pub labels: Vec<String>,
    // Nutrients per 100g
    pub calories: f32,      // kcal
    pub protein: f32,       // g
    pub fat: f32,           // g
    pub saturated_fat: f32, // g
    pub carbs: f32,         // g
    pub sugar: f32,         // g
    pub fiber: f32,         // g
    pub salt: f32,          // mg
    // Package info
    pub package_size_g: f32, // grams
    pub package_price: f32,  // price in local currency
}

impl Ingredient {
    /// Get nutrient value per 100 kcal
    pub fn per_calorie(&self, value_per_100g: f32) -> f32 {
        if self.calories > 0.0 {
            (value_per_100g / self.calories) * 100.0
        } else {
            0.0
        }
    }

    /// Create a new empty ingredient
    pub fn new_empty() -> Self {
        Self {
            id: None,
            name: String::new(),
            labels: Vec::new(),
            calories: 0.0,
            protein: 0.0,
            fat: 0.0,
            saturated_fat: 0.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 0.0,
            package_size_g: 0.0,
            package_price: 0.0,
        }
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Self::new_empty()
    }
}

/// Which column to sort by
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SortColumn {
    #[default]
    Name,
    Calories,
    Protein,
    Fat,
    SaturatedFat,
    Carbs,
    Sugar,
    Fiber,
    Salt,
    PackageSize,
    Price,
}

/// Sort direction
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    pub fn next(self) -> Self {
        match self {
            SortDirection::None => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::None,
        }
    }

    pub fn indicator(&self) -> &'static str {
        match self {
            SortDirection::None => "",
            SortDirection::Ascending => " \u{25B2}", // up arrow
            SortDirection::Descending => " \u{25BC}", // down arrow
        }
    }
}

/// Whether to show nutrients per 100g or per 100kcal
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum NutrientView {
    #[default]
    Per100g,
    Per100kcal,
}

// ============================================================================
// D1 Database Wrapper (SSR only)
// ============================================================================

/// Wrapper around CloudFlare D1 database that implements Send + Sync
/// Uses send_wrapper internally since CloudFlare Workers are single-threaded WASM
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct SendD1Database(std::sync::Arc<send_wrapper::SendWrapper<worker::d1::D1Database>>);

#[cfg(feature = "ssr")]
impl SendD1Database {
    pub fn new(db: worker::d1::D1Database) -> Self {
        Self(std::sync::Arc::new(send_wrapper::SendWrapper::new(db)))
    }

    pub fn inner(&self) -> &worker::d1::D1Database {
        &self.0
    }
}

// ============================================================================
// Server Functions
// ============================================================================

/// Fetch all ingredients from D1 database
#[server]
pub async fn get_ingredients() -> Result<Vec<Ingredient>, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    // Fetch all ingredients with labels aggregated via GROUP_CONCAT
    // This uses a single query instead of N+1 queries
    let ingredients = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "SELECT i.id, i.name, i.calories, i.protein, i.fat, i.saturated_fat, i.carbs, i.sugar, i.fiber, i.salt, i.package_size_g, i.package_price, GROUP_CONCAT(il.label, ',') as labels
             FROM ingredients i
             LEFT JOIN ingredient_labels il ON i.id = il.ingredient_id
             GROUP BY i.id
             ORDER BY i.name"
        );
        let results = stmt.all().await?;
        let rows: Vec<serde_json::Value> = results.results::<serde_json::Value>()?;

        let ingredients: Vec<Ingredient> = rows
            .into_iter()
            .map(|row| {
                // Parse comma-separated labels, handling NULL/empty case
                let labels: Vec<String> = row
                    .get("labels")
                    .and_then(|v| v.as_str())
                    .map(|s| s.split(',').map(|l| l.to_string()).collect())
                    .unwrap_or_default();

                Ingredient {
                    id: row.get("id").and_then(|v| v.as_i64()),
                    name: row
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    labels,
                    calories: row.get("calories").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    protein: row.get("protein").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    fat: row.get("fat").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    saturated_fat: row
                        .get("saturated_fat")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as f32,
                    carbs: row.get("carbs").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    sugar: row.get("sugar").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    fiber: row.get("fiber").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    salt: row.get("salt").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32,
                    package_size_g: row
                        .get("package_size_g")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as f32,
                    package_price: row
                        .get("package_price")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as f32,
                }
            })
            .collect();

        Ok::<_, worker::Error>(ingredients)
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

    Ok(ingredients)
}

/// Create a new ingredient
#[server]
pub async fn create_ingredient(ingredient: Ingredient) -> Result<Ingredient, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let result = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "INSERT INTO ingredients (name, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"
        );

        let stmt = stmt.bind(&[
            ingredient.name.clone().into(),
            ingredient.calories.into(),
            ingredient.protein.into(),
            ingredient.fat.into(),
            ingredient.saturated_fat.into(),
            ingredient.carbs.into(),
            ingredient.sugar.into(),
            ingredient.fiber.into(),
            ingredient.salt.into(),
            ingredient.package_size_g.into(),
            ingredient.package_price.into(),
        ])?;

        stmt.first::<serde_json::Value>(None).await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;

    let id = result
        .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
        .ok_or_else(|| ServerFnError::new("Failed to get inserted ID"))?;

    // Insert labels
    for label in &ingredient.labels {
        SendWrapper::new(async {
            let stmt = db
                .inner()
                .prepare("INSERT INTO ingredient_labels (ingredient_id, label) VALUES (?, ?)");
            let stmt = stmt.bind(&[(id as f64).into(), label.clone().into()])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 insert label error: {:?}", e)))?;
    }

    log::info!("Created ingredient: {} (id: {})", ingredient.name, id);

    Ok(Ingredient {
        id: Some(id),
        ..ingredient
    })
}

/// Update an existing ingredient
#[server]
pub async fn update_ingredient(ingredient: Ingredient) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let id = ingredient
        .id
        .ok_or_else(|| ServerFnError::new("Ingredient ID is required for update"))?;

    SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "UPDATE ingredients SET name = ?, calories = ?, protein = ?, fat = ?, saturated_fat = ?, carbs = ?, sugar = ?, fiber = ?, salt = ?, package_size_g = ?, package_price = ?, updated_at = datetime('now') WHERE id = ?"
        );

        let stmt = stmt.bind(&[
            ingredient.name.into(),
            ingredient.calories.into(),
            ingredient.protein.into(),
            ingredient.fat.into(),
            ingredient.saturated_fat.into(),
            ingredient.carbs.into(),
            ingredient.sugar.into(),
            ingredient.fiber.into(),
            ingredient.salt.into(),
            ingredient.package_size_g.into(),
            ingredient.package_price.into(),
            (id as f64).into(),
        ])?;

        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 update error: {:?}", e)))?;

    // Delete existing labels and re-insert
    SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare("DELETE FROM ingredient_labels WHERE ingredient_id = ?");
        let stmt = stmt.bind(&[(id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete labels error: {:?}", e)))?;

    // Insert updated labels
    for label in &ingredient.labels {
        SendWrapper::new(async {
            let stmt = db
                .inner()
                .prepare("INSERT INTO ingredient_labels (ingredient_id, label) VALUES (?, ?)");
            let stmt = stmt.bind(&[(id as f64).into(), label.clone().into()])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 insert label error: {:?}", e)))?;
    }

    log::info!("Updated ingredient id: {}", id);
    Ok(())
}

/// Delete an ingredient
#[server]
pub async fn delete_ingredient(id: i64) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    // Labels will be deleted by CASCADE
    SendWrapper::new(async {
        let stmt = db.inner().prepare("DELETE FROM ingredients WHERE id = ?");
        let stmt = stmt.bind(&[(id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete error: {:?}", e)))?;

    log::info!("Deleted ingredient id: {}", id);
    Ok(())
}

/// Bulk upsert ingredients - inserts new ingredients or updates existing ones by name
#[server]
pub async fn bulk_upsert_ingredients(ingredients: Vec<Ingredient>) -> Result<usize, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();
    let mut count = 0;

    for ingredient in ingredients {
        // Check if exists by name
        let existing = SendWrapper::new(async {
            let stmt = db
                .inner()
                .prepare("SELECT id FROM ingredients WHERE name = ?");
            let stmt = stmt.bind(&[ingredient.name.clone().into()])?;
            stmt.first::<serde_json::Value>(None).await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

        let ingredient_id: i64;

        if let Some(row) = existing {
            // Update existing
            ingredient_id = row
                .get("id")
                .and_then(|v| v.as_i64())
                .ok_or_else(|| ServerFnError::new("Failed to get ID"))?;

            SendWrapper::new(async {
                let stmt = db.inner().prepare(
                    "UPDATE ingredients SET calories = ?, protein = ?, fat = ?, saturated_fat = ?, carbs = ?, sugar = ?, fiber = ?, salt = ?, package_size_g = ?, package_price = ?, updated_at = datetime('now') WHERE id = ?"
                );
                let stmt = stmt.bind(&[
                    ingredient.calories.into(),
                    ingredient.protein.into(),
                    ingredient.fat.into(),
                    ingredient.saturated_fat.into(),
                    ingredient.carbs.into(),
                    ingredient.sugar.into(),
                    ingredient.fiber.into(),
                    ingredient.salt.into(),
                    ingredient.package_size_g.into(),
                    ingredient.package_price.into(),
                    (ingredient_id as f64).into(),
                ])?;
                stmt.run().await
            })
            .await
            .map_err(|e| ServerFnError::new(format!("D1 update error: {:?}", e)))?;

            log::info!(
                "Updated ingredient: {} (id: {})",
                ingredient.name,
                ingredient_id
            );
        } else {
            // Insert new
            let result = SendWrapper::new(async {
                let stmt = db.inner().prepare(
                    "INSERT INTO ingredients (name, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"
                );
                let stmt = stmt.bind(&[
                    ingredient.name.clone().into(),
                    ingredient.calories.into(),
                    ingredient.protein.into(),
                    ingredient.fat.into(),
                    ingredient.saturated_fat.into(),
                    ingredient.carbs.into(),
                    ingredient.sugar.into(),
                    ingredient.fiber.into(),
                    ingredient.salt.into(),
                    ingredient.package_size_g.into(),
                    ingredient.package_price.into(),
                ])?;
                stmt.first::<serde_json::Value>(None).await
            })
            .await
            .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;

            ingredient_id = result
                .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
                .ok_or_else(|| ServerFnError::new("Failed to get inserted ID"))?;

            log::info!("Inserted ingredient: {}", ingredient.name);
        }

        // Delete existing labels and re-insert
        SendWrapper::new(async {
            let stmt = db
                .inner()
                .prepare("DELETE FROM ingredient_labels WHERE ingredient_id = ?");
            let stmt = stmt.bind(&[(ingredient_id as f64).into()])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 delete labels error: {:?}", e)))?;

        // Insert labels
        for label in &ingredient.labels {
            SendWrapper::new(async {
                let stmt = db
                    .inner()
                    .prepare("INSERT INTO ingredient_labels (ingredient_id, label) VALUES (?, ?)");
                let stmt = stmt.bind(&[(ingredient_id as f64).into(), label.clone().into()])?;
                stmt.run().await
            })
            .await
            .map_err(|e| ServerFnError::new(format!("D1 insert label error: {:?}", e)))?;
        }

        count += 1;
    }

    Ok(count)
}

// ============================================================================
// Components
// ============================================================================

#[component]
fn SortableHeader(
    col: SortColumn,
    label: &'static str,
    width_class: &'static str,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    on_click: impl Fn(SortColumn) + 'static,
) -> impl IntoView {
    view! {
      <th
        class=format!(
          "px-3 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-700 select-none {}",
          width_class,
        )
        on:click=move |_| on_click(col)
      >
        <span class="inline-flex items-center gap-1">
          {label}
          <span class="w-3 inline-block text-center">
            {move || { if sort_column.get() == col { sort_direction.get().indicator() } else { "" } }}
          </span>
        </span>
      </th>
    }
}

/// Label badge component
#[component]
fn LabelBadge(label: String, on_remove: Option<Box<dyn Fn() + Send + Sync>>) -> impl IntoView {
    view! {
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
        {label}
        {on_remove
          .map(|remove| {
            view! {
              <button
                type="button"
                class="ml-0.5 text-blue-600 hover:text-blue-800 dark:text-blue-300 dark:hover:text-blue-100"
                on:click=move |_| remove()
              >
                <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            }
          })}
      </span>
    }
}

/// Label filter component (similar to recipe filtering)
#[component]
fn LabelFilter(
    all_labels: ReadSignal<Vec<String>>,
    selected_labels: RwSignal<Vec<String>>,
) -> impl IntoView {
    let toggle_label = move |label: String| {
        selected_labels.update(|labels| {
            if let Some(pos) = labels.iter().position(|l| l == &label) {
                labels.remove(pos);
            } else {
                labels.push(label);
            }
        });
    };

    view! {
      <div class="flex flex-wrap gap-2 items-center">
        <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Filter by label:"</span>
        <For
          each=move || all_labels.get()
          key=|label| label.clone()
          children=move |label: String| {
            let label_clone = label.clone();
            let label_for_check = label.clone();
            let is_selected = move || selected_labels.get().contains(&label_for_check);
            view! {
              <button
                type="button"
                class=move || {
                  let base = "px-2 py-1 text-xs font-medium rounded-full transition-colors";
                  if is_selected() {
                    format!("{} bg-blue-600 text-white", base)
                  } else {
                    format!(
                      "{} bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600",
                      base,
                    )
                  }
                }
                on:click={
                  let label_clone = label_clone.clone();
                  move |_| toggle_label(label_clone.clone())
                }
              >
                {label}
              </button>
            }
          }
        />
        <Show when=move || !selected_labels.get().is_empty()>
          <button
            type="button"
            class="text-xs text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200 underline"
            on:click=move |_| selected_labels.set(Vec::new())
          >
            "Clear all"
          </button>
        </Show>
      </div>
    }
}

/// Modal for creating/editing ingredients
#[component]
fn IngredientModal(
    show: RwSignal<bool>,
    editing: RwSignal<Option<Ingredient>>,
    on_save: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
    // Form fields
    let name = RwSignal::new(String::new());
    let labels = RwSignal::new(Vec::<String>::new());
    let new_label = RwSignal::new(String::new());
    let calories = RwSignal::new(String::new());
    let protein = RwSignal::new(String::new());
    let fat = RwSignal::new(String::new());
    let saturated_fat = RwSignal::new(String::new());
    let carbs = RwSignal::new(String::new());
    let sugar = RwSignal::new(String::new());
    let fiber = RwSignal::new(String::new());
    let salt = RwSignal::new(String::new());
    let package_size = RwSignal::new(String::new());
    let package_price = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let saving = RwSignal::new(false);

    // Populate form when editing changes
    Effect::new(move || {
        if let Some(ing) = editing.get() {
            name.set(ing.name.clone());
            labels.set(ing.labels.clone());
            calories.set(ing.calories.to_string());
            protein.set(ing.protein.to_string());
            fat.set(ing.fat.to_string());
            saturated_fat.set(ing.saturated_fat.to_string());
            carbs.set(ing.carbs.to_string());
            sugar.set(ing.sugar.to_string());
            fiber.set(ing.fiber.to_string());
            salt.set(ing.salt.to_string());
            package_size.set(ing.package_size_g.to_string());
            package_price.set(ing.package_price.to_string());
        } else {
            // Reset form for new ingredient
            name.set(String::new());
            labels.set(Vec::new());
            calories.set(String::new());
            protein.set(String::new());
            fat.set(String::new());
            saturated_fat.set(String::new());
            carbs.set(String::new());
            sugar.set(String::new());
            fiber.set(String::new());
            salt.set(String::new());
            package_size.set(String::new());
            package_price.set(String::new());
        }
        new_label.set(String::new());
        error.set(None);
    });

    let close = move || {
        show.set(false);
        editing.set(None);
    };

    let add_label = move || {
        let label_val = new_label.get().trim().to_lowercase();
        if !label_val.is_empty() {
            labels.update(|list| {
                if !list.contains(&label_val) {
                    list.push(label_val);
                }
            });
            new_label.set(String::new());
        }
    };

    let remove_label = move |idx: usize| {
        labels.update(|list| {
            if idx < list.len() {
                list.remove(idx);
            }
        });
    };

    let handle_save = {
        let on_save = on_save.clone();
        move || {
            let name_val = name.get();
            if name_val.trim().is_empty() {
                error.set(Some("Name is required".to_string()));
                return;
            }

            let ingredient = Ingredient {
                id: editing.get().and_then(|e| e.id),
                name: name_val,
                labels: labels.get(),
                calories: calories.get().parse().unwrap_or(0.0),
                protein: protein.get().parse().unwrap_or(0.0),
                fat: fat.get().parse().unwrap_or(0.0),
                saturated_fat: saturated_fat.get().parse().unwrap_or(0.0),
                carbs: carbs.get().parse().unwrap_or(0.0),
                sugar: sugar.get().parse().unwrap_or(0.0),
                fiber: fiber.get().parse().unwrap_or(0.0),
                salt: salt.get().parse().unwrap_or(0.0),
                package_size_g: package_size.get().parse().unwrap_or(0.0),
                package_price: package_price.get().parse().unwrap_or(0.0),
            };

            saving.set(true);
            let on_save = on_save.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = if ingredient.id.is_some() {
                    update_ingredient(ingredient).await.map(|_| ())
                } else {
                    create_ingredient(ingredient).await.map(|_| ())
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

    let input_class = "w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500";
    let label_class = "block text-sm font-medium text-slate-700 mb-1";

    view! {
      <Show when=move || show.get()>
        <div
          id="ingredient-modal-backdrop"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 overflow-y-auto py-4"
          on:click=move |ev: web_sys::MouseEvent| {
            if let Some(target) = ev.target() {
              if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.id() == "ingredient-modal-backdrop" {
                  close();
                }
              }
            }
          }
        >
          <div class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl mx-4">
            <div class="mb-4 flex items-center justify-between">
              <h2 class="text-xl font-bold text-slate-900">
                {move || if editing.get().is_some() { "Edit Ingredient" } else { "New Ingredient" }}
              </h2>
              <button class="text-slate-500 hover:text-slate-700" on:click=move |_| close()>
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            <Show when=move || error.get().is_some()>
              <div class="mb-4 rounded bg-red-100 px-4 py-2 text-sm text-red-700">
                {move || error.get().unwrap_or_default()}
              </div>
            </Show>

            <div class="grid grid-cols-2 gap-4">
              // Name
              <div class="col-span-2">
                <label class=label_class>"Name"</label>
                <input
                  type="text"
                  class=input_class
                  prop:value=move || name.get()
                  on:input=move |ev| name.set(event_target_value(&ev))
                  placeholder="e.g., Chicken Breast"
                />
              </div>

              // Labels
              <div class="col-span-2">
                <label class=label_class>"Labels"</label>
                <div class="flex gap-2 mb-2">
                  <input
                    type="text"
                    class=input_class
                    prop:value=move || new_label.get()
                    on:input=move |ev| new_label.set(event_target_value(&ev))
                    on:keypress=move |ev: web_sys::KeyboardEvent| {
                      if ev.key() == "Enter" {
                        ev.prevent_default();
                        add_label();
                      }
                    }
                    placeholder="Add a label (e.g., protein, costco)"
                  />
                  <button
                    type="button"
                    class="px-3 py-2 text-sm font-medium rounded bg-blue-600 text-white hover:bg-blue-700"
                    on:click=move |_| add_label()
                  >
                    "Add"
                  </button>
                </div>
                <div class="flex flex-wrap gap-2">
                  {move || {
                    labels
                      .get()
                      .into_iter()
                      .enumerate()
                      .map(|(idx, label)| {
                        view! { <LabelBadge label=label.clone() on_remove=Some(Box::new(move || remove_label(idx))) /> }
                      })
                      .collect_view()
                  }}
                </div>
              </div>

              // Nutrients section
              <div class="col-span-2">
                <h3 class="text-sm font-semibold text-slate-600 mb-2 mt-2">"Nutrients (per 100g)"</h3>
              </div>

              <div>
                <label class=label_class>"Calories (kcal)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || calories.get()
                  on:input=move |ev| calories.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Protein (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || protein.get()
                  on:input=move |ev| protein.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Fat (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || fat.get()
                  on:input=move |ev| fat.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Saturated Fat (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || saturated_fat.get()
                  on:input=move |ev| saturated_fat.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Carbs (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || carbs.get()
                  on:input=move |ev| carbs.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Sugar (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || sugar.get()
                  on:input=move |ev| sugar.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Fiber (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || fiber.get()
                  on:input=move |ev| fiber.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Salt (mg)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || salt.get()
                  on:input=move |ev| salt.set(event_target_value(&ev))
                />
              </div>

              // Package info section
              <div class="col-span-2">
                <h3 class="text-sm font-semibold text-slate-600 mb-2 mt-2">"Package Information"</h3>
              </div>

              <div>
                <label class=label_class>"Package Size (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  class=input_class
                  prop:value=move || package_size.get()
                  on:input=move |ev| package_size.set(event_target_value(&ev))
                />
              </div>
              <div>
                <label class=label_class>"Price ($)"</label>
                <input
                  type="number"
                  step="0.01"
                  class=input_class
                  prop:value=move || package_price.get()
                  on:input=move |ev| package_price.set(event_target_value(&ev))
                />
              </div>
            </div>

            <div class="mt-6 flex justify-end gap-3">
              <button
                class="rounded bg-slate-200 px-4 py-2 font-medium text-slate-700 hover:bg-slate-300"
                on:click=move |_| close()
              >
                "Cancel"
              </button>
              <button
                class="rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300"
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
      </Show>
    }
}

/// Result of parsing a single line of TSV input
#[derive(Clone, Debug)]
pub enum ParsedLine {
    /// A label header line (e.g., "Labels: protein, meat")
    Labels(Vec<String>),
    /// A valid ingredient
    Ingredient(Ingredient),
    /// A line that couldn't be parsed (with error message)
    Error(String, String),
    /// Empty or header line to skip
    Skip,
}

/// Parse TSV input into ingredients
/// Format: Name\tPrice\tUnit size\tCalories\tTotal Fat\tSaturated Fat\tCarbs\tSugar\tFiber\tProtein\tSalt\t[Labels]
fn parse_tsv_ingredients(input: &str) -> Vec<ParsedLine> {
    let mut results = Vec::new();
    let mut current_labels: Vec<String> = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();

        // Check if this is a header line
        if parts.len() == 1 || (parts.len() > 1 && parts[1..].iter().all(|p| p.trim().is_empty())) {
            let name = parts[0].trim();
            // Check for labels header (e.g., "Labels: protein, meat")
            if name.to_lowercase().starts_with("labels:") {
                let labels_str = name[7..].trim();
                current_labels = labels_str
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect();
                results.push(ParsedLine::Labels(current_labels.clone()));
                continue;
            }
            // Check for the header row
            if name == "Name" || name.contains("Calories") {
                results.push(ParsedLine::Skip);
                continue;
            }
        }

        // Try to parse as ingredient
        // Expected columns: Name, Price, Unit size, Calories, Total Fat, Saturated Fat, Carbs, Sugar, Fiber, Protein, Salt, [Labels]
        if parts.len() < 11 {
            results.push(ParsedLine::Error(
                line.to_string(),
                format!("Expected at least 11 columns, got {}", parts.len()),
            ));
            continue;
        }

        let name = parts[0].trim();
        if name.is_empty() {
            results.push(ParsedLine::Skip);
            continue;
        }

        let parse_f32 = |s: &str| -> f32 { s.trim().parse().unwrap_or(0.0) };

        // Parse labels from 12th column if present, otherwise use current_labels
        let labels = if parts.len() > 11 && !parts[11].trim().is_empty() {
            parts[11]
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            current_labels.clone()
        };

        let ingredient = Ingredient {
            id: None,
            name: name.to_string(),
            labels,
            package_price: parse_f32(parts[1]),
            package_size_g: parse_f32(parts[2]),
            calories: parse_f32(parts[3]),
            fat: parse_f32(parts[4]),
            saturated_fat: parse_f32(parts[5]),
            carbs: parse_f32(parts[6]),
            sugar: parse_f32(parts[7]),
            fiber: parse_f32(parts[8]),
            protein: parse_f32(parts[9]),
            salt: parse_f32(parts[10]),
        };

        results.push(ParsedLine::Ingredient(ingredient));
    }

    results
}

/// Modal for bulk importing ingredients from TSV
#[component]
fn BulkImportModal(
    show: RwSignal<bool>,
    on_save: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let tsv_input = RwSignal::new(String::new());
    let parsed_results = RwSignal::new(Vec::<ParsedLine>::new());
    let error = RwSignal::new(Option::<String>::None);
    let importing = RwSignal::new(false);

    // Parse input whenever it changes
    Effect::new(move || {
        let input = tsv_input.get();
        let results = parse_tsv_ingredients(&input);
        parsed_results.set(results);
    });

    let close = move || {
        show.set(false);
        tsv_input.set(String::new());
        parsed_results.set(Vec::new());
        error.set(None);
    };

    let get_ingredients_to_import = move || -> Vec<Ingredient> {
        parsed_results
            .get()
            .into_iter()
            .filter_map(|p| match p {
                ParsedLine::Ingredient(ing) => Some(ing),
                _ => None,
            })
            .collect()
    };

    let handle_import = {
        let on_save = on_save.clone();
        move || {
            let ingredients = get_ingredients_to_import();
            if ingredients.is_empty() {
                error.set(Some("No valid ingredients to import".to_string()));
                return;
            }

            importing.set(true);
            let on_save = on_save.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = bulk_upsert_ingredients(ingredients).await;

                importing.set(false);
                match result {
                    Ok(count) => {
                        log::info!("Imported {} ingredients", count);
                        show.set(false);
                        tsv_input.set(String::new());
                        parsed_results.set(Vec::new());
                        on_save();
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to import: {}", e)));
                    }
                }
            });
        }
    };

    let input_class = "w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 font-mono";
    let cell_class = "px-2 py-1 text-xs border-b border-slate-200";

    view! {
      <Show when=move || show.get()>
        <div
          id="bulk-import-modal-backdrop"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 overflow-y-auto py-4"
          on:click=move |ev: web_sys::MouseEvent| {
            if let Some(target) = ev.target() {
              if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.id() == "bulk-import-modal-backdrop" {
                  close();
                }
              }
            }
          }
        >
          <div class="w-full max-w-6xl rounded-lg bg-white p-6 shadow-xl mx-4 max-h-[90vh] overflow-y-auto">
            <div class="mb-4 flex items-center justify-between">
              <h2 class="text-xl font-bold text-slate-900">"Bulk Import Ingredients"</h2>
              <button class="text-slate-500 hover:text-slate-700" on:click=move |_| close()>
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            <Show when=move || error.get().is_some()>
              <div class="mb-4 rounded bg-red-100 px-4 py-2 text-sm text-red-700">
                {move || error.get().unwrap_or_default()}
              </div>
            </Show>

            <div class="mb-4">
              <label class="block text-sm font-medium text-slate-700 mb-2">
                "Paste TSV data (tab-separated values from spreadsheet)"
              </label>
              <p class="text-xs text-slate-500 mb-2">
                "Format: Name, Price, Unit size, Calories, Total Fat, Saturated Fat, Carbs, Sugar, Fiber, Protein, Salt, Labels (optional)"
              </p>
              <p class="text-xs text-slate-500 mb-2">
                "Label headers (e.g., \"Labels: protein, meat\") on their own line will set labels for following ingredients."
              </p>
              <textarea
                class=input_class
                rows=8
                prop:value=move || tsv_input.get()
                on:input=move |ev| tsv_input.set(event_target_value(&ev))
                placeholder="Paste your ingredient data here..."
              />
            </div>

            // Preview section
            <div class="mb-4">
              <h3 class="text-sm font-semibold text-slate-700 mb-2">"Preview"</h3>
              {move || {
                let results = parsed_results.get();
                let ingredient_count = results.iter().filter(|p| matches!(p, ParsedLine::Ingredient(_))).count();
                let error_count = results.iter().filter(|p| matches!(p, ParsedLine::Error(_, _))).count();

                view! {
                  <div class="text-sm text-slate-600 mb-2">
                    <span class="font-medium">{ingredient_count}</span>
                    " ingredients to import"
                    {if error_count > 0 {
                      view! { <span class="text-red-600 ml-2">"("{error_count}" errors)"</span> }.into_any()
                    } else {
                      view! { <span></span> }.into_any()
                    }}
                  </div>

                  <Show when=move || !results.is_empty()>
                    <div class="rounded border border-slate-200 overflow-x-auto max-h-64 overflow-y-auto">
                      <table class="w-full text-xs">
                        <thead class="bg-slate-50 sticky top-0">
                          <tr>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Status"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Name"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Labels"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Price"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Size (g)"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Cal"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Fat"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Sat. Fat"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Carbs"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Sugar"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Fiber"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Protein"</th>
                            <th class="px-2 py-1 text-left font-medium text-slate-600">"Salt"</th>
                          </tr>
                        </thead>
                        <tbody>
                          <For each=move || parsed_results.get().into_iter().enumerate() key=|(i, _)| *i let:item>
                            {
                              let (_, parsed) = item;
                              match parsed {
                                ParsedLine::Labels(labels) => {
                                  view! {
                                    <tr class="bg-blue-50">
                                      <td class=cell_class colspan="13">
                                        <span class="font-semibold text-blue-700">"Labels: "{labels.join(", ")}</span>
                                      </td>
                                    </tr>
                                  }
                                    .into_any()
                                }
                                ParsedLine::Ingredient(ing) => {
                                  view! {
                                    <tr class="hover:bg-slate-50">
                                      <td class=cell_class>
                                        <span class="text-green-600 font-medium">"OK"</span>
                                      </td>
                                      <td class=format!("{} font-medium", cell_class)>{ing.name.clone()}</td>
                                      <td class=cell_class>
                                        <div class="flex flex-wrap gap-1">
                                          {ing
                                            .labels
                                            .iter()
                                            .map(|l| {
                                              view! {
                                                <span class="px-1 bg-blue-100 text-blue-800 rounded text-xs">
                                                  {l.clone()}
                                                </span>
                                              }
                                            })
                                            .collect_view()}
                                        </div>
                                      </td>
                                      <td class=cell_class>
                                        {if ing.package_price > 0.0 {
                                          format!("${:.2}", ing.package_price)
                                        } else {
                                          "-".to_string()
                                        }}
                                      </td>
                                      <td class=cell_class>{format!("{:.0}", ing.package_size_g)}</td>
                                      <td class=cell_class>{format!("{:.0}", ing.calories)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.fat)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.saturated_fat)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.carbs)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.sugar)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.fiber)}</td>
                                      <td class=cell_class>{format!("{:.1}", ing.protein)}</td>
                                      <td class=cell_class>{format!("{:.2}", ing.salt)}</td>
                                    </tr>
                                  }
                                    .into_any()
                                }
                                ParsedLine::Error(line, err) => {
                                  view! {
                                    <tr class="bg-red-50">
                                      <td class=cell_class>
                                        <span class="text-red-600 font-medium">"Error"</span>
                                      </td>
                                      <td class=cell_class colspan="12">
                                        <span class="text-red-600">{err}</span>
                                        <span class="text-slate-500 ml-2 truncate block max-w-md">{line}</span>
                                      </td>
                                    </tr>
                                  }
                                    .into_any()
                                }
                                ParsedLine::Skip => view! { <tr></tr> }.into_any(),
                              }
                            }
                          </For>
                        </tbody>
                      </table>
                    </div>
                  </Show>
                }
              }}
            </div>

            <div class="mt-6 flex justify-end gap-3">
              <button
                class="rounded bg-slate-200 px-4 py-2 font-medium text-slate-700 hover:bg-slate-300"
                on:click=move |_| close()
              >
                "Cancel"
              </button>
              <button
                class="rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300"
                disabled=move || importing.get() || get_ingredients_to_import().is_empty()
                on:click={
                  let handle_import = handle_import.clone();
                  move |_| handle_import()
                }
              >
                {move || {
                  if importing.get() {
                    "Importing...".to_string()
                  } else {
                    let count = get_ingredients_to_import().len();
                    format!("Import {} ingredient{}", count, if count == 1 { "" } else { "s" })
                  }
                }}
              </button>
            </div>
          </div>
        </div>
      </Show>
    }
}

#[component]
fn IngredientTable(
    ingredients: ReadSignal<Vec<Ingredient>>,
    view_mode: ReadSignal<NutrientView>,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    selected_labels: ReadSignal<Vec<String>>,
    on_header_click: impl Fn(SortColumn) + Clone + Send + Sync + 'static,
    on_edit: impl Fn(Ingredient) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let get_sorted_ingredients = move || {
        let selected = selected_labels.get();
        let mut filtered: Vec<Ingredient> = ingredients
            .get()
            .into_iter()
            .filter(|i| {
                // If no labels selected, show all; otherwise filter by labels
                selected.is_empty() || selected.iter().all(|l| i.labels.contains(l))
            })
            .collect();

        let dir = sort_direction.get();
        let col = sort_column.get();
        let view = view_mode.get();

        if dir != SortDirection::None {
            filtered.sort_by(|a, b| {
                let get_value = |ing: &Ingredient| -> f32 {
                    let raw = match col {
                        SortColumn::Name => return 0.0, // Handle separately
                        SortColumn::Calories => ing.calories,
                        SortColumn::Protein => ing.protein,
                        SortColumn::Fat => ing.fat,
                        SortColumn::SaturatedFat => ing.saturated_fat,
                        SortColumn::Carbs => ing.carbs,
                        SortColumn::Sugar => ing.sugar,
                        SortColumn::Fiber => ing.fiber,
                        SortColumn::Salt => ing.salt,
                        SortColumn::PackageSize => ing.package_size_g,
                        SortColumn::Price => ing.package_price,
                    };
                    if view == NutrientView::Per100kcal
                        && !matches!(
                            col,
                            SortColumn::PackageSize | SortColumn::Price | SortColumn::Calories
                        )
                    {
                        ing.per_calorie(raw)
                    } else {
                        raw
                    }
                };

                if col == SortColumn::Name {
                    let cmp = a.name.cmp(&b.name);
                    return if dir == SortDirection::Ascending {
                        cmp
                    } else {
                        cmp.reverse()
                    };
                }

                let val_a = get_value(a);
                let val_b = get_value(b);
                let cmp = val_a
                    .partial_cmp(&val_b)
                    .unwrap_or(std::cmp::Ordering::Equal);
                if dir == SortDirection::Ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            });
        } else {
            // Default: sort by name ascending
            filtered.sort_by(|a, b| a.name.cmp(&b.name));
        }

        filtered
    };

    // Column width classes
    let w_name = "w-64"; // Ingredient name (wider to compensate for removed columns)
    let w_cal = "w-20"; // Calories
    let w_nutr = "w-16"; // Nutrient columns (protein, fat, etc.)
    let w_salt = "w-20"; // Salt (needs more space for mg)
    let w_actions = "w-16"; // Actions column

    let cell_class = "px-3 py-3 whitespace-nowrap text-slate-700 dark:text-slate-300";

    view! {
      <div class="rounded-lg bg-white shadow-md overflow-hidden overflow-x-auto dark:bg-slate-800">
        <table class="w-full table-fixed divide-y divide-slate-200 dark:divide-slate-700 text-sm">
          <thead class="bg-slate-50 dark:bg-slate-700">
            <tr>
              <SortableHeader
                col=SortColumn::Name
                label="Ingredient"
                width_class=w_name
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Calories
                label="Calories"
                width_class=w_cal
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Protein
                label="Protein"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Fat
                label="Fat"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::SaturatedFat
                label="Sat. Fat"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Carbs
                label="Carbs"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Sugar
                label="Sugar"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Fiber
                label="Fiber"
                width_class=w_nutr
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <SortableHeader
                col=SortColumn::Salt
                label="Salt"
                width_class=w_salt
                sort_column=sort_column
                sort_direction=sort_direction
                on_click=on_header_click.clone()
              />
              <Show when=move || auth.is_authenticated.get()>
                <th class=format!(
                  "px-3 py-3 text-left text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wider {}",
                  w_actions,
                )></th>
              </Show>
            </tr>
          </thead>
          <tbody class="bg-white dark:bg-slate-800 divide-y divide-slate-200 dark:divide-slate-700">
            <For each=get_sorted_ingredients key=|ing| ing.id.unwrap_or(0) let:ing>
              {
                let ing_cal = ing.clone();
                let ing_protein = ing.clone();
                let ing_fat = ing.clone();
                let ing_sat_fat = ing.clone();
                let ing_carbs = ing.clone();
                let ing_sugar = ing.clone();
                let ing_fiber = ing.clone();
                let ing_salt = ing.clone();
                let ing_for_edit = ing.clone();
                let on_edit = on_edit.clone();
                let labels = ing.labels.clone();
                let tooltip = {
                  let mut parts = Vec::new();
                  if !labels.is_empty() {
                    parts.push(format!("Labels: {}", labels.join(", ")));
                  }
                  parts.push(format!("Package: {}g", ing.package_size_g));
                  parts.push(format!("Price: ${:.2}", ing.package_price));
                  parts.join("\n")
                };
                // Build tooltip with labels, package size, and price
                view! {
                  <tr class="hover:bg-slate-50 dark:hover:bg-slate-700">
                    <td
                      class=format!(
                        "{} font-medium text-slate-900 dark:text-slate-100 truncate cursor-help",
                        cell_class,
                      )
                      title=tooltip
                    >
                      {ing.name.clone()}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let cal = if view_mode.get() == NutrientView::Per100kcal { 100.0 } else { ing_cal.calories };
                        format!("{:.0} kcal", cal)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_protein.per_calorie(ing_protein.protein)
                        } else {
                          ing_protein.protein
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_fat.per_calorie(ing_fat.fat)
                        } else {
                          ing_fat.fat
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_sat_fat.per_calorie(ing_sat_fat.saturated_fat)
                        } else {
                          ing_sat_fat.saturated_fat
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_carbs.per_calorie(ing_carbs.carbs)
                        } else {
                          ing_carbs.carbs
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_sugar.per_calorie(ing_sugar.sugar)
                        } else {
                          ing_sugar.sugar
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_fiber.per_calorie(ing_fiber.fiber)
                        } else {
                          ing_fiber.fiber
                        };
                        format!("{:.1}g", val)
                      }}
                    </td>
                    <td class=cell_class>
                      {move || {
                        let val = if view_mode.get() == NutrientView::Per100kcal {
                          ing_salt.per_calorie(ing_salt.salt)
                        } else {
                          ing_salt.salt
                        };
                        format!("{:.0}mg", val)
                      }}
                    </td>
                    <Show when=move || auth.is_authenticated.get()>
                      <td class=cell_class>
                        <button
                          class="text-blue-600 hover:text-blue-800"
                          title="Edit"
                          on:click={
                            let ing_for_edit = ing_for_edit.clone();
                            let on_edit = on_edit.clone();
                            move |_| on_edit(ing_for_edit.clone())
                          }
                        >
                          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                            />
                          </svg>
                        </button>
                      </td>
                    </Show>
                  </tr>
                }
              }
            </For>
          </tbody>
        </table>
      </div>
      <p class="mt-2 text-xs text-slate-500 dark:text-slate-400">
        {move || {
          let suffix = if view_mode.get() == NutrientView::Per100kcal { "/100kcal" } else { "/100g" };
          format!("* Nutrient values shown {}", suffix)
        }}
      </p>
    }
}

#[component]
pub fn Ingredients() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let (view_mode, set_view_mode) = signal(NutrientView::Per100g);
    let (sort_column, set_sort_column) = signal(SortColumn::Name);
    let (sort_direction, set_sort_direction) = signal(SortDirection::None);
    let selected_labels = RwSignal::new(Vec::<String>::new());

    // Modal state
    let show_modal = RwSignal::new(false);
    let editing_ingredient = RwSignal::new(Option::<Ingredient>::None);
    let show_bulk_import = RwSignal::new(false);

    // Fetch ingredients from server
    let ingredients_resource = Resource::new(|| (), |_| get_ingredients());

    // Refetch after save
    let refetch = move || {
        ingredients_resource.refetch();
    };

    let handle_header_click = move |col: SortColumn| {
        if sort_column.get() == col {
            set_sort_direction.set(sort_direction.get().next());
        } else {
            set_sort_column.set(col);
            set_sort_direction.set(SortDirection::Descending);
        }
    };

    let handle_new = move |_| {
        editing_ingredient.set(None);
        show_modal.set(true);
    };

    let handle_edit = move |ing: Ingredient| {
        editing_ingredient.set(Some(ing));
        show_modal.set(true);
    };

    view! {
      <div class="mx-auto max-w-7xl py-6">
        <div class="mb-6 flex items-center justify-between flex-wrap gap-4">
          <h2 class="text-3xl font-bold text-slate-900 dark:text-slate-100">"Ingredient List"</h2>
          <div class="flex items-center gap-3 flex-wrap">
            <Show when=move || auth.is_authenticated.get()>
              <button
                class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
                on:click=handle_new
              >
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                "New Ingredient"
              </button>
              <button
                class="flex items-center gap-2 rounded bg-purple-600 px-4 py-2 text-sm font-medium text-white hover:bg-purple-700"
                on:click=move |_| show_bulk_import.set(true)
              >
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                  />
                </svg>
                "Bulk Import"
              </button>
            </Show>
            <div class="flex items-center gap-3 bg-white dark:bg-slate-800 rounded-lg px-4 py-2 shadow-sm">
              <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"View nutrients:"</span>
              <button
                class=move || {
                  let base = "px-3 py-1 text-sm font-medium rounded transition-colors";
                  if view_mode.get() == NutrientView::Per100g {
                    format!("{} bg-blue-600 text-white", base)
                  } else {
                    format!(
                      "{} bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600",
                      base,
                    )
                  }
                }
                on:click=move |_| set_view_mode.set(NutrientView::Per100g)
              >
                "per 100g"
              </button>
              <button
                class=move || {
                  let base = "px-3 py-1 text-sm font-medium rounded transition-colors";
                  if view_mode.get() == NutrientView::Per100kcal {
                    format!("{} bg-blue-600 text-white", base)
                  } else {
                    format!(
                      "{} bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-600",
                      base,
                    )
                  }
                }
                on:click=move |_| set_view_mode.set(NutrientView::Per100kcal)
              >
                "per 100kcal"
              </button>
            </div>
          </div>
        </div>

        <Suspense fallback=move || {
          view! { <p class="text-slate-600 dark:text-slate-400">"Loading ingredients..."</p> }
        }>
          {move || {
            ingredients_resource
              .get()
              .map(|result| {
                match result {
                  Ok(ings) => {
                    let all_labels: Vec<String> = {
                      let mut labels: Vec<String> = ings.iter().flat_map(|i| i.labels.clone()).collect();
                      labels.sort();
                      labels.dedup();
                      labels
                    };
                    let (ingredients, _) = signal(ings);
                    let (all_labels_signal, _) = signal(all_labels.clone());
                    if ingredients.get().is_empty() {
                      // Collect all unique labels

                      view! {
                        <div class="text-center py-12">
                          <p class="text-slate-600 dark:text-slate-400 mb-4">"No ingredients yet."</p>
                          <Show when=move || auth.is_authenticated.get()>
                            <p class="text-slate-500 dark:text-slate-500 text-sm">
                              "Click \"New Ingredient\" to create your first ingredient."
                            </p>
                          </Show>
                        </div>
                      }
                        .into_any()
                    } else {
                      view! {
                        <div class="mb-4">
                          <Show when=move || !all_labels.is_empty()>
                            <LabelFilter all_labels=all_labels_signal selected_labels=selected_labels />
                          </Show>
                        </div>

                        <IngredientTable
                          ingredients=ingredients
                          view_mode=view_mode
                          sort_column=sort_column
                          sort_direction=sort_direction
                          selected_labels=selected_labels.read_only()
                          on_header_click=handle_header_click
                          on_edit=handle_edit
                        />
                      }
                        .into_any()
                    }
                  }
                  Err(e) => {
                    view! {
                      <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                        <p class="font-medium">"Failed to load ingredients"</p>
                        <p class="text-sm">{e.to_string()}</p>
                      </div>
                    }
                      .into_any()
                  }
                }
              })
          }}
        </Suspense>

        <IngredientModal show=show_modal editing=editing_ingredient on_save=refetch />
        <BulkImportModal show=show_bulk_import on_save=refetch />
      </div>
    }
}
