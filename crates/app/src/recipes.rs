//! Recipes module
//!
//! Contains recipe data structures, D1 database operations, and the Recipes page components.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use wasm_bindgen::JsCast;

use crate::auth::AdminAuth;
use crate::ingredients::{get_ingredients, Ingredient};

// ============================================================================
// Data Types
// ============================================================================

/// An ingredient used in a recipe with its amount
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecipeIngredient {
    pub id: Option<i64>,
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub amount_grams: f32,
    pub use_whole_package: bool,
    pub package_size_g: f32,
    // Cached nutrient values from the ingredient (per 100g)
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub saturated_fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub sugar_per_100g: f32,
    pub fiber_per_100g: f32,
    pub salt_per_100g: f32,
}

impl RecipeIngredient {
    /// Get the effective amount in grams
    pub fn effective_grams(&self) -> f32 {
        if self.use_whole_package {
            self.package_size_g
        } else {
            self.amount_grams
        }
    }

    /// Calculate calories for this ingredient
    pub fn calories(&self) -> f32 {
        self.calories_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate protein for this ingredient
    pub fn protein(&self) -> f32 {
        self.protein_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate fat for this ingredient
    pub fn fat(&self) -> f32 {
        self.fat_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate saturated fat for this ingredient
    pub fn saturated_fat(&self) -> f32 {
        self.saturated_fat_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate carbs for this ingredient
    pub fn carbs(&self) -> f32 {
        self.carbs_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate sugar for this ingredient
    pub fn sugar(&self) -> f32 {
        self.sugar_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate fiber for this ingredient
    pub fn fiber(&self) -> f32 {
        self.fiber_per_100g * self.effective_grams() / 100.0
    }

    /// Calculate salt for this ingredient
    pub fn salt(&self) -> f32 {
        self.salt_per_100g * self.effective_grams() / 100.0
    }
}

/// Computed nutrition values for a recipe
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct RecipeNutrition {
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub saturated_fat: f32,
    pub carbs: f32,
    pub sugar: f32,
    pub fiber: f32,
    pub salt: f32,
}

impl RecipeNutrition {
    /// Compute total nutrition from ingredients
    pub fn from_ingredients(ingredients: &[RecipeIngredient]) -> Self {
        let mut nutrition = Self::default();
        for ing in ingredients {
            nutrition.calories += ing.calories();
            nutrition.protein += ing.protein();
            nutrition.fat += ing.fat();
            nutrition.saturated_fat += ing.saturated_fat();
            nutrition.carbs += ing.carbs();
            nutrition.sugar += ing.sugar();
            nutrition.fiber += ing.fiber();
            nutrition.salt += ing.salt();
        }
        nutrition
    }

    /// Get nutrition per serving
    pub fn per_serving(&self, servings: i32) -> Self {
        let s = servings.max(1) as f32;
        Self {
            calories: self.calories / s,
            protein: self.protein / s,
            fat: self.fat / s,
            saturated_fat: self.saturated_fat / s,
            carbs: self.carbs / s,
            sugar: self.sugar / s,
            fiber: self.fiber / s,
            salt: self.salt / s,
        }
    }
}

/// A recipe with its ingredients
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub instructions: Vec<String>,
    pub ingredients: Vec<RecipeIngredient>,
}

impl Recipe {
    /// Create a new empty recipe
    pub fn new_empty() -> Self {
        Self {
            id: None,
            name: String::new(),
            description: String::new(),
            servings: 2,
            prep_time_minutes: 0,
            cook_time_minutes: 0,
            instructions: Vec::new(),
            ingredients: Vec::new(),
        }
    }

    /// Compute total nutrition for this recipe
    pub fn nutrition(&self) -> RecipeNutrition {
        RecipeNutrition::from_ingredients(&self.ingredients)
    }

    /// Compute nutrition per serving
    pub fn nutrition_per_serving(&self) -> RecipeNutrition {
        self.nutrition().per_serving(self.servings)
    }

    /// Format total time
    pub fn total_time(&self) -> String {
        let total = self.prep_time_minutes + self.cook_time_minutes;
        if total >= 60 {
            format!("{}h {}min", total / 60, total % 60)
        } else {
            format!("{}min", total)
        }
    }
}

impl Default for Recipe {
    fn default() -> Self {
        Self::new_empty()
    }
}

// ============================================================================
// Server Functions
// ============================================================================

#[cfg(feature = "ssr")]
use crate::ingredients::SendD1Database;

/// Fetch all recipes from D1 database
#[server]
pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    // Fetch all recipes and their ingredients in a single SendWrapper block
    let recipes = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "SELECT id, name, description, servings, prep_time_minutes, cook_time_minutes, instructions FROM recipes ORDER BY name"
        );
        let recipe_results = stmt.all().await?;

        let recipe_rows: Vec<serde_json::Value> = recipe_results.results::<serde_json::Value>()?;

        let mut recipes = Vec::new();

        for row in recipe_rows {
            let recipe_id = row
                .get("id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            let instructions_json = row
                .get("instructions")
                .and_then(|v| v.as_str())
                .unwrap_or("[]");
            let instructions: Vec<String> =
                serde_json::from_str(instructions_json).unwrap_or_default();

            // Fetch ingredients for this recipe
            let ing_stmt = db.inner().prepare(
                "SELECT ri.id, ri.ingredient_id, ri.amount_grams, ri.use_whole_package,
                        i.name, i.calories, i.protein, i.fat, i.saturated_fat, 
                        i.carbs, i.sugar, i.fiber, i.salt, i.package_size_g
                 FROM recipe_ingredients ri
                 JOIN ingredients i ON ri.ingredient_id = i.id
                 WHERE ri.recipe_id = ?
                 ORDER BY ri.id",
            );
            let ing_stmt = ing_stmt.bind(&[(recipe_id as f64).into()])?;
            let ing_results = ing_stmt.all().await?;

            let ing_rows: Vec<serde_json::Value> = ing_results.results::<serde_json::Value>()?;

            let ingredients: Vec<RecipeIngredient> = ing_rows
                .into_iter()
                .filter_map(|r| {
                    Some(RecipeIngredient {
                        id: r.get("id")?.as_i64(),
                        ingredient_id: r.get("ingredient_id")?.as_i64()?,
                        ingredient_name: r.get("name")?.as_str()?.to_string(),
                        amount_grams: r.get("amount_grams")?.as_f64()? as f32,
                        use_whole_package: r.get("use_whole_package")?.as_i64()? == 1,
                        package_size_g: r.get("package_size_g")?.as_f64()? as f32,
                        calories_per_100g: r.get("calories")?.as_f64()? as f32,
                        protein_per_100g: r.get("protein")?.as_f64()? as f32,
                        fat_per_100g: r.get("fat")?.as_f64()? as f32,
                        saturated_fat_per_100g: r.get("saturated_fat")?.as_f64()? as f32,
                        carbs_per_100g: r.get("carbs")?.as_f64()? as f32,
                        sugar_per_100g: r.get("sugar")?.as_f64()? as f32,
                        fiber_per_100g: r.get("fiber")?.as_f64()? as f32,
                        salt_per_100g: r.get("salt")?.as_f64()? as f32,
                    })
                })
                .collect();

            recipes.push(Recipe {
                id: Some(recipe_id),
                name: row
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: row
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                servings: row.get("servings").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
                prep_time_minutes: row
                    .get("prep_time_minutes")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32,
                cook_time_minutes: row
                    .get("cook_time_minutes")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32,
                instructions,
                ingredients,
            });
        }

        Ok::<_, worker::Error>(recipes)
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

    Ok(recipes)
}

/// Create a new recipe
#[server]
pub async fn create_recipe(recipe: Recipe) -> Result<Recipe, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let instructions_json = serde_json::to_string(&recipe.instructions)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Insert the recipe
    let result = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "INSERT INTO recipes (name, description, servings, prep_time_minutes, cook_time_minutes, instructions) VALUES (?, ?, ?, ?, ?, ?) RETURNING id"
        );
        let stmt = stmt.bind(&[
            recipe.name.clone().into(),
            recipe.description.clone().into(),
            (recipe.servings as f64).into(),
            (recipe.prep_time_minutes as f64).into(),
            (recipe.cook_time_minutes as f64).into(),
            instructions_json.into(),
        ])?;
        stmt.first::<serde_json::Value>(None).await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;

    let recipe_id = result
        .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
        .ok_or_else(|| ServerFnError::new("Failed to get inserted recipe ID"))?;

    // Insert recipe ingredients
    for ing in &recipe.ingredients {
        SendWrapper::new(async {
            let stmt = db.inner().prepare(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount_grams, use_whole_package) VALUES (?, ?, ?, ?)"
            );
            let stmt = stmt.bind(&[
                (recipe_id as f64).into(),
                (ing.ingredient_id as f64).into(),
                (ing.amount_grams as f64).into(),
                (if ing.use_whole_package { 1.0 } else { 0.0 }).into(),
            ])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 insert ingredient error: {:?}", e)))?;
    }

    log::info!("Created recipe: {} (id: {})", recipe.name, recipe_id);

    Ok(Recipe {
        id: Some(recipe_id),
        ..recipe
    })
}

/// Update an existing recipe
#[server]
pub async fn update_recipe(recipe: Recipe) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let recipe_id = recipe
        .id
        .ok_or_else(|| ServerFnError::new("Recipe ID is required for update"))?;

    let instructions_json = serde_json::to_string(&recipe.instructions)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Update the recipe
    SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "UPDATE recipes SET name = ?, description = ?, servings = ?, prep_time_minutes = ?, cook_time_minutes = ?, instructions = ?, updated_at = datetime('now') WHERE id = ?"
        );
        let stmt = stmt.bind(&[
            recipe.name.into(),
            recipe.description.into(),
            (recipe.servings as f64).into(),
            (recipe.prep_time_minutes as f64).into(),
            (recipe.cook_time_minutes as f64).into(),
            instructions_json.into(),
            (recipe_id as f64).into(),
        ])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 update error: {:?}", e)))?;

    // Delete existing ingredients and re-insert
    SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare("DELETE FROM recipe_ingredients WHERE recipe_id = ?");
        let stmt = stmt.bind(&[(recipe_id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete ingredients error: {:?}", e)))?;

    // Insert updated ingredients
    for ing in &recipe.ingredients {
        SendWrapper::new(async {
            let stmt = db.inner().prepare(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, amount_grams, use_whole_package) VALUES (?, ?, ?, ?)"
            );
            let stmt = stmt.bind(&[
                (recipe_id as f64).into(),
                (ing.ingredient_id as f64).into(),
                (ing.amount_grams as f64).into(),
                (if ing.use_whole_package { 1.0 } else { 0.0 }).into(),
            ])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 insert ingredient error: {:?}", e)))?;
    }

    log::info!("Updated recipe id: {}", recipe_id);
    Ok(())
}

/// Delete a recipe
#[server]
pub async fn delete_recipe(id: i64) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    // Delete recipe ingredients first (cascade should handle this, but be explicit)
    SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare("DELETE FROM recipe_ingredients WHERE recipe_id = ?");
        let stmt = stmt.bind(&[(id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete ingredients error: {:?}", e)))?;

    // Delete the recipe
    SendWrapper::new(async {
        let stmt = db.inner().prepare("DELETE FROM recipes WHERE id = ?");
        let stmt = stmt.bind(&[(id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete error: {:?}", e)))?;

    log::info!("Deleted recipe id: {}", id);
    Ok(())
}

// ============================================================================
// Components
// ============================================================================

/// Ingredient selector for recipe editing
#[component]
fn IngredientSelector(
    available_ingredients: ReadSignal<Vec<Ingredient>>,
    selected_ingredients: RwSignal<Vec<RecipeIngredient>>,
) -> impl IntoView {
    let add_ingredient = move |ing: Ingredient| {
        selected_ingredients.update(|list| {
            // Check if already added
            if list
                .iter()
                .any(|ri| ri.ingredient_id == ing.id.unwrap_or(-1))
            {
                return;
            }
            list.push(RecipeIngredient {
                id: None,
                ingredient_id: ing.id.unwrap_or(0),
                ingredient_name: ing.name.clone(),
                amount_grams: ing.package_size_g,
                use_whole_package: true,
                package_size_g: ing.package_size_g,
                calories_per_100g: ing.calories,
                protein_per_100g: ing.protein,
                fat_per_100g: ing.fat,
                saturated_fat_per_100g: ing.saturated_fat,
                carbs_per_100g: ing.carbs,
                sugar_per_100g: ing.sugar,
                fiber_per_100g: ing.fiber,
                salt_per_100g: ing.salt,
            });
        });
    };

    let remove_ingredient = move |idx: usize| {
        selected_ingredients.update(|list| {
            if idx < list.len() {
                list.remove(idx);
            }
        });
    };

    let update_amount = move |idx: usize, amount: f32| {
        selected_ingredients.update(|list| {
            if let Some(ing) = list.get_mut(idx) {
                ing.amount_grams = amount;
                ing.use_whole_package = false;
            }
        });
    };

    let toggle_whole_package = move |idx: usize| {
        selected_ingredients.update(|list| {
            if let Some(ing) = list.get_mut(idx) {
                ing.use_whole_package = !ing.use_whole_package;
                if ing.use_whole_package {
                    ing.amount_grams = ing.package_size_g;
                }
            }
        });
    };

    let input_class = "w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500";

    view! {
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-slate-700 mb-1">"Add Ingredient"</label>
          <select
            class=input_class
            on:change=move |ev| {
              let value = event_target_value(&ev);
              if let Ok(id) = value.parse::<i64>() {
                if let Some(ing) = available_ingredients.get().into_iter().find(|i| i.id == Some(id)) {
                  add_ingredient(ing);
                }
              }
              if let Some(target) = ev.target() {
                if let Some(select) = target.dyn_ref::<web_sys::HtmlSelectElement>() {
                  select.set_value("");
                }
              }
            }
          >
            <option value="">"-- Select an ingredient --"</option>
            <For
              each=move || available_ingredients.get()
              key=|ing| ing.id.unwrap_or(0)
              children=move |ing: Ingredient| {
                view! {
                  <option value=ing
                    .id
                    .unwrap_or(0)
                    .to_string()>{format!("{} ({}g pkg)", ing.name, ing.package_size_g)}</option>
                }
              }
            />
          </select>
        </div>

        <Show when=move || !selected_ingredients.get().is_empty()>
          <div class="rounded border border-slate-200 divide-y divide-slate-200">
            {move || {
              selected_ingredients
                .get()
                .into_iter()
                .enumerate()
                .map(|(idx, ing)| {
                  let ing_name = ing.ingredient_name.clone();
                  let pkg_size = ing.package_size_g;
                  let use_whole = ing.use_whole_package;
                  let amount = ing.amount_grams;
                  view! {
                    <div class="p-3 flex flex-wrap items-center gap-3 bg-white">
                      <span class="font-medium text-slate-900 min-w-[120px]">{ing_name}</span>
                      <div class="flex items-center gap-2">
                        <label class="flex items-center gap-1 text-sm text-slate-600">
                          <input
                            type="checkbox"
                            class="rounded border-slate-300"
                            prop:checked=use_whole
                            on:change=move |_| toggle_whole_package(idx)
                          />
                          {format!("Whole pkg ({}g)", pkg_size)}
                        </label>
                      </div>
                      <Show when=move || {
                        !selected_ingredients.get().get(idx).map(|i| i.use_whole_package).unwrap_or(true)
                      }>
                        <div class="flex items-center gap-1">
                          <input
                            type="number"
                            step="1"
                            min="1"
                            class="w-20 rounded border border-slate-300 px-2 py-1 text-sm"
                            prop:value=move || {
                              selected_ingredients.get().get(idx).map(|i| i.amount_grams).unwrap_or(amount)
                            }
                            on:input=move |ev| {
                              if let Ok(val) = event_target_value(&ev).parse::<f32>() {
                                update_amount(idx, val);
                              }
                            }
                          />
                          <span class="text-sm text-slate-600">"g"</span>
                        </div>
                      </Show>
                      <button
                        type="button"
                        class="ml-auto text-red-600 hover:text-red-800"
                        on:click=move |_| remove_ingredient(idx)
                      >
                        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                          />
                        </svg>
                      </button>
                    </div>
                  }
                })
                .collect_view()
            }}
          </div>
        </Show>
      </div>
    }
}

/// Instructions editor component
#[component]
fn InstructionsEditor(instructions: RwSignal<Vec<String>>) -> impl IntoView {
    let add_instruction = move |_| {
        instructions.update(|list| {
            list.push(String::new());
        });
    };

    let remove_instruction = move |idx: usize| {
        instructions.update(|list| {
            if idx < list.len() {
                list.remove(idx);
            }
        });
    };

    let update_instruction = move |idx: usize, value: String| {
        instructions.update(|list| {
            if let Some(inst) = list.get_mut(idx) {
                *inst = value;
            }
        });
    };

    view! {
      <div class="space-y-2">
        <label class="block text-sm font-medium text-slate-700">"Instructions"</label>
        {move || {
          instructions
            .get()
            .into_iter()
            .enumerate()
            .map(|(idx, inst)| {
              let inst_value = inst.clone();
              view! {
                <div class="flex items-start gap-2">
                  <span class="mt-2 text-sm font-medium text-slate-500 w-6">{idx + 1}"."</span>
                  <textarea
                    class="flex-1 rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    rows="2"
                    prop:value=inst_value
                    on:input=move |ev| update_instruction(idx, event_target_value(&ev))
                  />
                  <button
                    type="button"
                    class="mt-1 text-red-600 hover:text-red-800"
                    on:click=move |_| remove_instruction(idx)
                  >
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
              }
            })
            .collect_view()
        }}
        <button
          type="button"
          class="flex items-center gap-1 text-sm text-blue-600 hover:text-blue-800"
          on:click=add_instruction
        >
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          "Add step"
        </button>
      </div>
    }
}

/// Recipe edit modal component
#[component]
fn RecipeModal(
    show: RwSignal<bool>,
    editing: RwSignal<Option<Recipe>>,
    available_ingredients: ReadSignal<Vec<Ingredient>>,
    on_save: impl Fn() + Clone + Send + Sync + 'static,
    on_delete: impl Fn(i64) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    // Form fields
    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let servings = RwSignal::new(String::from("2"));
    let prep_time = RwSignal::new(String::new());
    let cook_time = RwSignal::new(String::new());
    let instructions = RwSignal::new(Vec::<String>::new());
    let selected_ingredients = RwSignal::new(Vec::<RecipeIngredient>::new());
    let error = RwSignal::new(Option::<String>::None);
    let saving = RwSignal::new(false);
    let show_delete_confirm = RwSignal::new(false);
    let trigger_delete = RwSignal::new(false);

    // Handle delete via Effect
    {
        let on_delete = on_delete.clone();
        Effect::new(move || {
            if trigger_delete.get() {
                trigger_delete.set(false);
                if let Some(recipe) = editing.get() {
                    if let Some(id) = recipe.id {
                        saving.set(true);
                        let on_delete = on_delete.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            match delete_recipe(id).await {
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

    // Populate form when editing changes
    Effect::new(move || {
        if let Some(recipe) = editing.get() {
            name.set(recipe.name.clone());
            description.set(recipe.description.clone());
            servings.set(recipe.servings.to_string());
            prep_time.set(if recipe.prep_time_minutes > 0 {
                recipe.prep_time_minutes.to_string()
            } else {
                String::new()
            });
            cook_time.set(if recipe.cook_time_minutes > 0 {
                recipe.cook_time_minutes.to_string()
            } else {
                String::new()
            });
            instructions.set(recipe.instructions.clone());
            selected_ingredients.set(recipe.ingredients.clone());
        } else {
            // Reset form for new recipe
            name.set(String::new());
            description.set(String::new());
            servings.set(String::from("2"));
            prep_time.set(String::new());
            cook_time.set(String::new());
            instructions.set(Vec::new());
            selected_ingredients.set(Vec::new());
        }
        error.set(None);
        show_delete_confirm.set(false);
    });

    let close = move || {
        show.set(false);
        editing.set(None);
    };

    let handle_save = {
        let on_save = on_save.clone();
        move || {
            let name_val = name.get();
            if name_val.trim().is_empty() {
                error.set(Some("Recipe name is required".to_string()));
                return;
            }

            let recipe = Recipe {
                id: editing.get().and_then(|e| e.id),
                name: name_val,
                description: description.get(),
                servings: servings.get().parse().unwrap_or(2),
                prep_time_minutes: prep_time.get().parse().unwrap_or(0),
                cook_time_minutes: cook_time.get().parse().unwrap_or(0),
                instructions: instructions.get(),
                ingredients: selected_ingredients.get(),
            };

            saving.set(true);
            let on_save = on_save.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = if recipe.id.is_some() {
                    update_recipe(recipe).await.map(|_| ())
                } else {
                    create_recipe(recipe).await.map(|_| ())
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
          id="recipe-modal-backdrop"
          class="fixed inset-0 z-50 flex items-start justify-center bg-black/50 overflow-y-auto py-4"
          on:click=move |ev: web_sys::MouseEvent| {
            if let Some(target) = ev.target() {
              if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.id() == "recipe-modal-backdrop" {
                  close();
                }
              }
            }
          }
        >
          <div class="w-full max-w-3xl rounded-lg bg-white p-6 shadow-xl mx-4 my-auto">
            <div class="mb-4 flex items-center justify-between">
              <h2 class="text-xl font-bold text-slate-900">
                {move || if editing.get().is_some() { "Edit Recipe" } else { "New Recipe" }}
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

            <div class="space-y-4 max-h-[70vh] overflow-y-auto pr-2">
              // Basic info
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div class="sm:col-span-2">
                  <label class=label_class>"Recipe Name"</label>
                  <input
                    type="text"
                    class=input_class
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                    placeholder="e.g., Grilled Chicken Bowl"
                  />
                </div>
                <div class="sm:col-span-2">
                  <label class=label_class>"Description"</label>
                  <textarea
                    class=input_class
                    rows="2"
                    prop:value=move || description.get()
                    on:input=move |ev| description.set(event_target_value(&ev))
                    placeholder="Brief description of the recipe..."
                  />
                </div>
                <div>
                  <label class=label_class>"Servings"</label>
                  <input
                    type="number"
                    min="1"
                    class=input_class
                    prop:value=move || servings.get()
                    on:input=move |ev| servings.set(event_target_value(&ev))
                  />
                </div>
                <div class="grid grid-cols-2 gap-2">
                  <div>
                    <label class=label_class>"Prep Time (min)"</label>
                    <input
                      type="number"
                      min="0"
                      class=input_class
                      prop:value=move || prep_time.get()
                      on:input=move |ev| prep_time.set(event_target_value(&ev))
                    />
                  </div>
                  <div>
                    <label class=label_class>"Cook Time (min)"</label>
                    <input
                      type="number"
                      min="0"
                      class=input_class
                      prop:value=move || cook_time.get()
                      on:input=move |ev| cook_time.set(event_target_value(&ev))
                    />
                  </div>
                </div>
              </div>

              // Ingredients section
              <div class="border-t border-slate-200 pt-4">
                <h3 class="text-lg font-semibold text-slate-800 mb-3">"Ingredients"</h3>
                <IngredientSelector
                  available_ingredients=available_ingredients
                  selected_ingredients=selected_ingredients
                />
              </div>

              // Instructions section
              <div class="border-t border-slate-200 pt-4">
                <InstructionsEditor instructions=instructions />
              </div>

              // Nutrition preview
              <Show when=move || !selected_ingredients.get().is_empty()>
                <div class="border-t border-slate-200 pt-4">
                  <h3 class="text-lg font-semibold text-slate-800 mb-2">"Nutrition Preview (per serving)"</h3>
                  {move || {
                    let ings = selected_ingredients.get();
                    let servings_count = servings.get().parse::<i32>().unwrap_or(1).max(1);
                    let nutrition = RecipeNutrition::from_ingredients(&ings).per_serving(servings_count);
                    view! {
                      <div class="grid grid-cols-4 gap-2 text-sm">
                        <div class="bg-slate-50 rounded p-2 text-center">
                          <div class="font-semibold text-slate-900">{format!("{:.0}", nutrition.calories)}</div>
                          <div class="text-slate-600">"kcal"</div>
                        </div>
                        <div class="bg-slate-50 rounded p-2 text-center">
                          <div class="font-semibold text-slate-900">{format!("{:.1}g", nutrition.protein)}</div>
                          <div class="text-slate-600">"protein"</div>
                        </div>
                        <div class="bg-slate-50 rounded p-2 text-center">
                          <div class="font-semibold text-slate-900">{format!("{:.1}g", nutrition.carbs)}</div>
                          <div class="text-slate-600">"carbs"</div>
                        </div>
                        <div class="bg-slate-50 rounded p-2 text-center">
                          <div class="font-semibold text-slate-900">{format!("{:.1}g", nutrition.fat)}</div>
                          <div class="text-slate-600">"fat"</div>
                        </div>
                      </div>
                    }
                  }}
                </div>
              </Show>
            </div>

            <div class="mt-6 flex justify-between gap-3 border-t border-slate-200 pt-4">
              <div>
                <Show when=move || editing.get().and_then(|r| r.id).is_some()>
                  <Show
                    when=move || show_delete_confirm.get()
                    fallback=move || {
                      view! {
                        <button
                          class="rounded bg-red-100 px-4 py-2 font-medium text-red-700 hover:bg-red-200"
                          on:click=move |_| show_delete_confirm.set(true)
                        >
                          "Delete"
                        </button>
                      }
                    }
                  >
                    <div class="flex items-center gap-2">
                      <span class="text-sm text-red-700">"Are you sure?"</span>
                      <button
                        class="rounded bg-red-600 px-3 py-1 text-sm font-medium text-white hover:bg-red-700"
                        on:click=move |_| trigger_delete.set(true)
                      >
                        "Yes, delete"
                      </button>
                      <button
                        class="rounded bg-slate-200 px-3 py-1 text-sm font-medium text-slate-700 hover:bg-slate-300"
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
        </div>
      </Show>
    }
}

/// Recipe card component
#[component]
fn RecipeCard(
    recipe: Recipe,
    on_edit: impl Fn(Recipe) + Clone + Send + Sync + 'static,
    is_authenticated: ReadSignal<bool>,
) -> impl IntoView {
    let nutrition = recipe.nutrition_per_serving();
    let recipe_for_edit = recipe.clone();
    let recipe_name = recipe.name.clone();
    let recipe_desc = recipe.description.clone();
    let has_description = !recipe.description.is_empty();
    let servings = recipe.servings;
    let prep = recipe.prep_time_minutes;
    let cook = recipe.cook_time_minutes;
    let total = recipe.total_time();
    let has_ingredients = !recipe.ingredients.is_empty();
    let has_instructions = !recipe.instructions.is_empty();
    let ingredients = recipe.ingredients.clone();
    let instructions = recipe.instructions.clone();

    view! {
      <div class="rounded-lg bg-white dark:bg-slate-800 p-6 shadow-md">
        <div class="mb-4 flex items-start justify-between">
          <div>
            <h3 class="text-xl font-bold text-slate-900 dark:text-slate-100">{recipe_name}</h3>
            <Show when=move || has_description>
              <p class="mt-1 text-sm text-slate-600 dark:text-slate-400">{recipe_desc.clone()}</p>
            </Show>
          </div>
          <Show when=move || is_authenticated.get()>
            <button
              class="text-blue-600 hover:text-blue-800 p-1"
              title="Edit recipe"
              on:click={
                let recipe_for_edit = recipe_for_edit.clone();
                let on_edit = on_edit.clone();
                move |_| on_edit(recipe_for_edit.clone())
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

        <div class="mb-4 flex flex-wrap gap-4 text-sm text-slate-600 dark:text-slate-400">
          <span>{format!("Servings: {}", servings)}</span>
          <Show when=move || { prep > 0 }>
            <span>{format!("Prep: {}min", prep)}</span>
          </Show>
          <Show when=move || { cook > 0 }>
            <span>{format!("Cook: {}min", cook)}</span>
          </Show>
          <Show when=move || { prep > 0 || cook > 0 }>
            <span class="font-medium">{format!("Total: {}", total.clone())}</span>
          </Show>
        </div>

        <Show when=move || has_ingredients>
          <div class="mb-4">
            <h4 class="font-semibold text-slate-900 dark:text-slate-100 mb-2">"Ingredients:"</h4>
            <ul class="list-inside list-disc space-y-1 text-slate-700 dark:text-slate-300">
              {ingredients
                .iter()
                .map(|ing| {
                  let amount_str = if ing.use_whole_package {
                    format!("{}g (whole pkg)", ing.effective_grams())
                  } else {
                    format!("{}g", ing.effective_grams())
                  };
                  view! { <li>{format!("{} - {}", ing.ingredient_name, amount_str)}</li> }
                })
                .collect_view()}
            </ul>
          </div>
        </Show>

        <Show when=move || has_instructions>
          <div class="mb-4">
            <h4 class="font-semibold text-slate-900 dark:text-slate-100 mb-2">"Instructions:"</h4>
            <ol class="list-inside list-decimal space-y-1 text-slate-700 dark:text-slate-300">
              {instructions.iter().map(|inst| view! { <li>{inst.clone()}</li> }).collect_view()}
            </ol>
          </div>
        </Show>

        <div class="rounded bg-slate-50 dark:bg-slate-700 p-3">
          <p class="text-sm font-medium text-slate-900 dark:text-slate-100">
            {format!(
              "Nutrition per serving: {:.0} kcal | {:.1}g protein | {:.1}g carbs | {:.1}g fat",
              nutrition.calories,
              nutrition.protein,
              nutrition.carbs,
              nutrition.fat,
            )}
          </p>
          <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
            {format!(
              "Sat. fat: {:.1}g | Sugar: {:.1}g | Fiber: {:.1}g | Salt: {:.0}mg",
              nutrition.saturated_fat,
              nutrition.sugar,
              nutrition.fiber,
              nutrition.salt,
            )}
          </p>
        </div>
      </div>
    }
}

/// Main Recipes page component
#[component]
pub fn Recipes() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    // Modal state
    let show_modal = RwSignal::new(false);
    let editing_recipe = RwSignal::new(Option::<Recipe>::None);

    // Fetch recipes and ingredients from server
    let recipes_resource = Resource::new(|| (), |_| get_recipes());
    let ingredients_resource = Resource::new(|| (), |_| get_ingredients());

    // Refetch after save/delete
    let refetch = move || {
        recipes_resource.refetch();
    };

    let handle_delete = move |_id: i64| {
        recipes_resource.refetch();
    };

    let handle_new = move |_| {
        editing_recipe.set(None);
        show_modal.set(true);
    };

    let handle_edit = move |recipe: Recipe| {
        editing_recipe.set(Some(recipe));
        show_modal.set(true);
    };

    view! {
      <div class="mx-auto max-w-7xl py-6">
        <div class="mb-6 flex items-center justify-between flex-wrap gap-4">
          <h2 class="text-3xl font-bold text-slate-900 dark:text-slate-100">"Recipes"</h2>
          <Show when=move || auth.is_authenticated.get()>
            <button
              class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
              on:click=handle_new
            >
              <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              "New Recipe"
            </button>
          </Show>
        </div>

        <Suspense fallback=move || {
          view! { <p class="text-slate-600 dark:text-slate-400">"Loading recipes..."</p> }
        }>
          {move || {
            let recipes_result = recipes_resource.get();
            let ingredients_result = ingredients_resource.get();
            match (recipes_result, ingredients_result) {
              (Some(Ok(recipes)), Some(Ok(ingredients))) => {
                let (ingredients_signal, _) = signal(ingredients);
                let is_auth = auth.is_authenticated;
                if recipes.is_empty() {

                  view! {
                    <div class="text-center py-12">
                      <p class="text-slate-600 dark:text-slate-400 mb-4">"No recipes yet."</p>
                      <Show when=move || auth.is_authenticated.get()>
                        <p class="text-slate-500 dark:text-slate-500 text-sm">
                          "Click \"New Recipe\" to create your first recipe."
                        </p>
                      </Show>
                    </div>
                    <RecipeModal
                      show=show_modal
                      editing=editing_recipe
                      available_ingredients=ingredients_signal
                      on_save=refetch
                      on_delete=handle_delete
                    />
                  }
                    .into_any()
                } else {
                  view! {
                    <div class="grid gap-6 lg:grid-cols-2">
                      <For
                        each=move || recipes.clone()
                        key=|recipe| recipe.id.unwrap_or(0)
                        children=move |recipe: Recipe| {
                          let is_auth_signal = is_auth.read_only();
                          view! { <RecipeCard recipe=recipe on_edit=handle_edit is_authenticated=is_auth_signal /> }
                        }
                      />
                    </div>
                    <RecipeModal
                      show=show_modal
                      editing=editing_recipe
                      available_ingredients=ingredients_signal
                      on_save=refetch
                      on_delete=handle_delete
                    />
                  }
                    .into_any()
                }
              }
              (Some(Err(e)), _) | (_, Some(Err(e))) => {
                view! {
                  <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                    <p class="font-medium">"Failed to load data"</p>
                    <p class="text-sm">{e.to_string()}</p>
                  </div>
                }
                  .into_any()
              }
              _ => view! { <p class="text-slate-600">"Loading..."</p> }.into_any(),
            }
          }}
        </Suspense>
      </div>
    }
}
