//! Recipes module
//!
//! Persists recipes in D1, links recipe ingredients to the ingredient list,
//! and provides CRUD UI with nutrition calculated from ingredient data.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use std::{collections::HashMap, sync::Arc};
use wasm_bindgen::JsCast;

#[cfg(feature = "ssr")]
use crate::ingredients::SendD1Database;
use crate::{
    auth::AdminAuth,
    ingredients::{get_ingredients, Ingredient},
};

// ============================================================================
// Data Types
// ============================================================================

#[derive(Clone, Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct RecipeNutrition {
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub sat_fat: f32,
    pub salt: f32,
    pub fiber: f32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RecipeIngredient {
    pub ingredient_id: i64,
    pub ingredient_name: String,
    pub packages: f32,
    pub grams: Option<f32>,
    pub package_size_g: f32,
    pub amount_g: f32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub meal_type: String,
    pub tags: Vec<String>,
    pub prep_time: String,
    pub cook_time: String,
    pub servings: i32,
    pub ingredients: Vec<RecipeIngredient>,
    pub instructions: Vec<String>,
    pub nutrition: RecipeNutrition,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RecipeIngredientInput {
    pub ingredient_id: i64,
    pub packages: f32,
    pub grams: Option<f32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RecipeInput {
    pub id: Option<i64>,
    pub name: String,
    pub meal_type: String,
    pub tags: Vec<String>,
    pub prep_time: String,
    pub cook_time: String,
    pub servings: i32,
    pub ingredients: Vec<RecipeIngredientInput>,
    pub instructions: Vec<String>,
}

// ============================================================================
// Helpers
// ============================================================================

fn get_meal_type_color(meal_type: &str) -> (&'static str, &'static str) {
    match meal_type {
        "Breakfast" => ("bg-yellow-100", "text-yellow-800"),
        "Lunch" => ("bg-orange-100", "text-orange-800"),
        "Dinner" => ("bg-indigo-100", "text-indigo-800"),
        "Snack" => ("bg-amber-100", "text-amber-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

fn get_tag_color(tag: &str) -> (&'static str, &'static str) {
    match tag {
        "High Protein" => ("bg-blue-100", "text-blue-800"),
        "Low Carb" => ("bg-purple-100", "text-purple-800"),
        "Vegan" | "Plant-Based" => ("bg-green-100", "text-green-800"),
        "Quick" => ("bg-yellow-100", "text-yellow-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

fn display_float(value: f32) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}

fn compute_amount_grams(packages: f32, grams: Option<f32>, package_size_g: f32) -> f32 {
    let pkg_amount = if packages > 0.0 && package_size_g > 0.0 {
        packages * package_size_g
    } else {
        0.0
    };
    let grams_amount = grams.unwrap_or(0.0);
    pkg_amount + grams_amount
}

fn compute_nutrition(
    ingredients: &[RecipeIngredientInput],
    ingredient_lookup: &HashMap<i64, Ingredient>,
    servings: i32,
) -> RecipeNutrition {
    let mut totals = RecipeNutrition::default();

    for item in ingredients {
        if let Some(ing) = ingredient_lookup.get(&item.ingredient_id) {
            let amount_g = compute_amount_grams(item.packages, item.grams, ing.package_size_g);
            if amount_g <= 0.0 {
                continue;
            }
            let factor = amount_g / 100.0;
            totals.calories += ing.calories * factor;
            totals.protein += ing.protein * factor;
            totals.carbs += ing.carbs * factor;
            totals.fat += ing.fat * factor;
            totals.sat_fat += ing.saturated_fat * factor;
            totals.fiber += ing.fiber * factor;
            totals.salt += (ing.salt / 1000.0) * factor; // stored in mg, display in g
        }
    }

    let servings = servings.max(1) as f32;
    RecipeNutrition {
        calories: totals.calories / servings,
        protein: totals.protein / servings,
        carbs: totals.carbs / servings,
        fat: totals.fat / servings,
        sat_fat: totals.sat_fat / servings,
        salt: totals.salt / servings,
        fiber: totals.fiber / servings,
    }
}

// ============================================================================
// Server Functions (SSR only)
// ============================================================================

#[cfg(feature = "ssr")]
fn parse_string_list(raw: Option<&serde_json::Value>) -> Vec<String> {
    if let Some(value) = raw {
        if value.is_array() {
            serde_json::from_value::<Vec<String>>(value.clone()).ok()
        } else if let Some(s) = value.as_str() {
            serde_json::from_str::<Vec<String>>(s).ok()
        } else {
            None
        }
    } else {
        None
    }
    .unwrap_or_default()
}

#[cfg(feature = "ssr")]
fn parse_f32(raw: Option<&serde_json::Value>) -> f32 {
    raw.and_then(|v| v.as_f64()).unwrap_or(0.0) as f32
}

#[cfg(feature = "ssr")]
fn parse_i64(raw: Option<&serde_json::Value>) -> Option<i64> {
    raw.and_then(|v| v.as_i64())
}

#[cfg(feature = "ssr")]
fn build_recipe_from_rows(rows: Vec<serde_json::Value>) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    let mut current: Option<Recipe> = None;

    for row in rows {
        let recipe_id = parse_i64(row.get("recipe_id")).unwrap_or_default();
        let tags = parse_string_list(row.get("tags"));
        let instructions = parse_string_list(row.get("instructions"));

        let base_recipe = Recipe {
            id: recipe_id,
            name: row
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            meal_type: row
                .get("meal_type")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            prep_time: row
                .get("prep_time")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            cook_time: row
                .get("cook_time")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            servings: row.get("servings").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
            tags,
            instructions,
            ingredients: Vec::new(),
            nutrition: RecipeNutrition::default(),
        };

        match current.take() {
            Some(mut recipe) if recipe.id == recipe_id => {
                append_ingredient(&mut recipe, &row);
                current = Some(recipe);
            }
            Some(recipe) => {
                recipes.push(finalize_recipe(recipe));
                let mut next = base_recipe;
                append_ingredient(&mut next, &row);
                current = Some(next);
            }
            None => {
                let mut next = base_recipe;
                append_ingredient(&mut next, &row);
                current = Some(next);
            }
        }
    }

    if let Some(recipe) = current {
        recipes.push(finalize_recipe(recipe));
    }

    recipes
}

#[cfg(feature = "ssr")]
fn append_ingredient(recipe: &mut Recipe, row: &serde_json::Value) {
    let ingredient_id = parse_i64(row.get("ingredient_id"));
    let ingredient_name = row
        .get("ingredient_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if ingredient_id.is_none() || ingredient_name.is_empty() {
        return;
    }

    let packages = parse_f32(row.get("packages"));
    let grams = row.get("grams").and_then(|v| v.as_f64()).map(|v| v as f32);
    let package_size_g = parse_f32(row.get("package_size_g"));
    let amount_g = compute_amount_grams(packages, grams, package_size_g);

    recipe.ingredients.push(RecipeIngredient {
        ingredient_id: ingredient_id.unwrap(),
        ingredient_name,
        packages,
        grams,
        package_size_g,
        amount_g,
    });

    // Accumulate total nutrition (per recipe, divide later)
    let factor = amount_g / 100.0;
    recipe.nutrition.calories += parse_f32(row.get("calories")) * factor;
    recipe.nutrition.protein += parse_f32(row.get("protein")) * factor;
    recipe.nutrition.carbs += parse_f32(row.get("carbs")) * factor;
    recipe.nutrition.fat += parse_f32(row.get("fat")) * factor;
    recipe.nutrition.sat_fat += parse_f32(row.get("saturated_fat")) * factor;
    recipe.nutrition.fiber += parse_f32(row.get("fiber")) * factor;
    recipe.nutrition.salt += (parse_f32(row.get("salt")) / 1000.0) * factor;
}

#[cfg(feature = "ssr")]
fn finalize_recipe(mut recipe: Recipe) -> Recipe {
    let servings = recipe.servings.max(1) as f32;
    recipe.nutrition.calories /= servings;
    recipe.nutrition.protein /= servings;
    recipe.nutrition.carbs /= servings;
    recipe.nutrition.fat /= servings;
    recipe.nutrition.sat_fat /= servings;
    recipe.nutrition.fiber /= servings;
    recipe.nutrition.salt /= servings;
    recipe
}

#[server]
pub async fn get_recipes() -> Result<Vec<Recipe>, ServerFnError> {
    use send_wrapper::SendWrapper;

    #[cfg(not(feature = "ssr"))]
    {
        let _ = SendWrapper;
    }

    #[cfg(feature = "ssr")]
    {
        let db = expect_context::<SendD1Database>();
        let rows = SendWrapper::new(async {
            let stmt = db.inner().prepare(
                "
                SELECT
                  r.id as recipe_id,
                  r.name,
                  r.meal_type,
                  r.prep_time,
                  r.cook_time,
                  r.servings,
                  r.tags,
                  r.instructions,
                  ri.ingredient_id,
                  ri.packages,
                  ri.grams,
                  ing.name as ingredient_name,
                  ing.package_size_g,
                  ing.calories,
                  ing.protein,
                  ing.fat,
                  ing.saturated_fat,
                  ing.carbs,
                  ing.fiber,
                  ing.salt
                FROM recipes r
                LEFT JOIN recipe_ingredients ri ON ri.recipe_id = r.id
                LEFT JOIN ingredients ing ON ri.ingredient_id = ing.id
                ORDER BY r.id, ri.id
                ",
            );
            stmt.all().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

        let rows = rows
            .results::<serde_json::Value>()
            .map_err(|e| ServerFnError::new(format!("D1 results error: {:?}", e)))?;

        return Ok(build_recipe_from_rows(rows));
    }

    Ok(Vec::new())
}

#[cfg(feature = "ssr")]
fn to_json_text<T: Serialize>(value: &T) -> Result<String, ServerFnError> {
    serde_json::to_string(value).map_err(|e| ServerFnError::new(format!("JSON error: {:?}", e)))
}

#[cfg(feature = "ssr")]
async fn persist_recipe(db: &SendD1Database, recipe: &RecipeInput) -> Result<i64, ServerFnError> {
    use send_wrapper::SendWrapper;

    let tags_json = to_json_text(&recipe.tags)?;
    let instructions_json = to_json_text(&recipe.instructions)?;
    let servings = recipe.servings.max(1);

    let result = SendWrapper::new(async {
        if let Some(id) = recipe.id {
            let stmt = db.inner().prepare(
                "UPDATE recipes
                 SET name = ?, meal_type = ?, prep_time = ?, cook_time = ?, servings = ?, tags = ?, instructions = ?, updated_at = datetime('now')
                 WHERE id = ?
                 RETURNING id",
            );
            let stmt = stmt.bind(&[
                recipe.name.clone().into(),
                recipe.meal_type.clone().into(),
                recipe.prep_time.clone().into(),
                recipe.cook_time.clone().into(),
                (servings as f64).into(),
                tags_json.clone().into(),
                instructions_json.clone().into(),
                (id as f64).into(),
            ])?;
            stmt.first::<serde_json::Value>(None).await
        } else {
            let stmt = db.inner().prepare(
                "INSERT INTO recipes (name, meal_type, prep_time, cook_time, servings, tags, instructions)
                 VALUES (?, ?, ?, ?, ?, ?, ?)
                 RETURNING id",
            );
            let stmt = stmt.bind(&[
                recipe.name.clone().into(),
                recipe.meal_type.clone().into(),
                recipe.prep_time.clone().into(),
                recipe.cook_time.clone().into(),
                (servings as f64).into(),
                tags_json.clone().into(),
                instructions_json.clone().into(),
            ])?;
            stmt.first::<serde_json::Value>(None).await
        }
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 upsert error: {:?}", e)))?;

    let id = result
        .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
        .ok_or_else(|| ServerFnError::new("Failed to persist recipe"))?;

    Ok(id)
}

#[cfg(feature = "ssr")]
async fn replace_recipe_ingredients(
    db: &SendD1Database,
    recipe_id: i64,
    ingredients: &[RecipeIngredientInput],
) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    // Clear existing rows
    SendWrapper::new(async {
        let stmt = db
            .inner()
            .prepare("DELETE FROM recipe_ingredients WHERE recipe_id = ?");
        let stmt = stmt.bind(&[(recipe_id as f64).into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete error: {:?}", e)))?;

    // Insert new rows
    for ing in ingredients {
        SendWrapper::new(async {
            let stmt = db.inner().prepare(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, packages, grams)
                 VALUES (?, ?, ?, ?)",
            );
            let gram_value: serde_json::Value = match ing.grams {
                Some(g) => (g as f64).into(),
                None => serde_json::Value::Null,
            };
            let stmt = stmt.bind(&[
                (recipe_id as f64).into(),
                (ing.ingredient_id as f64).into(),
                (ing.packages as f64).into(),
                gram_value,
            ])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 insert ingredient error: {:?}", e)))?;
    }

    Ok(())
}

#[server]
pub async fn upsert_recipe(recipe: RecipeInput) -> Result<Recipe, ServerFnError> {
    if recipe.name.trim().is_empty() {
        return Err(ServerFnError::new("Recipe name is required"));
    }
    if recipe.ingredients.is_empty() {
        return Err(ServerFnError::new("Add at least one ingredient"));
    }

    #[cfg(not(feature = "ssr"))]
    {
        return Ok(Recipe {
            id: 0,
            name: recipe.name,
            meal_type: recipe.meal_type,
            tags: recipe.tags,
            prep_time: recipe.prep_time,
            cook_time: recipe.cook_time,
            servings: recipe.servings,
            ingredients: Vec::new(),
            instructions: recipe.instructions,
            nutrition: RecipeNutrition::default(),
        });
    }

    #[cfg(feature = "ssr")]
    {
        let db = expect_context::<SendD1Database>();
        let recipe_id = persist_recipe(&db, &recipe).await?;
        replace_recipe_ingredients(&db, recipe_id, &recipe.ingredients).await?;

        // Fetch the single recipe back so we return computed nutrition
        let mut recipes = get_recipes().await?;
        let recipe = recipes
            .drain(..)
            .find(|r| r.id == recipe_id)
            .ok_or_else(|| ServerFnError::new("Recipe saved but not found"))?;
        Ok(recipe)
    }
}

#[server]
pub async fn delete_recipe(id: i64) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    #[cfg(not(feature = "ssr"))]
    {
        let _ = id;
        return Ok(());
    }

    #[cfg(feature = "ssr")]
    {
        let db = expect_context::<SendD1Database>();
        SendWrapper::new(async {
            let stmt = db.inner().prepare("DELETE FROM recipes WHERE id = ?");
            let stmt = stmt.bind(&[(id as f64).into()])?;
            stmt.run().await
        })
        .await
        .map_err(|e| ServerFnError::new(format!("D1 delete recipe error: {:?}", e)))?;
        Ok(())
    }
}

// ============================================================================
// Components
// ============================================================================

#[derive(Clone)]
struct IngredientRow {
    ingredient_id: Option<i64>,
    packages: String,
    grams: String,
}

impl IngredientRow {
    fn new() -> Self {
        Self {
            ingredient_id: None,
            packages: String::from("1"),
            grams: String::new(),
        }
    }

    fn from_recipe(ing: &RecipeIngredient) -> Self {
        Self {
            ingredient_id: Some(ing.ingredient_id),
            packages: if ing.packages > 0.0 {
                display_float(ing.packages)
            } else {
                String::new()
            },
            grams: ing
                .grams
                .map(|g| display_float(g))
                .unwrap_or_else(String::new),
        }
    }
}

fn build_input_from_form(
    name: String,
    meal_type: String,
    prep_time: String,
    cook_time: String,
    servings: String,
    tags: String,
    instructions: String,
    rows: &[IngredientRow],
) -> Result<RecipeInput, String> {
    let servings = servings.trim().parse::<i32>().unwrap_or(1).max(1);
    let tags_vec = tags
        .split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect();
    let instructions_vec = instructions
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    let mut ingredients = Vec::new();
    for row in rows {
        if let Some(id) = row.ingredient_id {
            let packages = row.packages.trim().parse::<f32>().unwrap_or(0.0);
            let grams_val = row.grams.trim().parse::<f32>().ok().filter(|v| *v > 0.0);
            ingredients.push(RecipeIngredientInput {
                ingredient_id: id,
                packages,
                grams: grams_val,
            });
        }
    }

    if ingredients.is_empty() {
        return Err("Add at least one ingredient".into());
    }

    Ok(RecipeInput {
        id: None,
        name,
        meal_type,
        tags: tags_vec,
        prep_time,
        cook_time,
        servings,
        ingredients,
        instructions: instructions_vec,
    })
}

fn ingredient_lookup(ingredients: &[Ingredient]) -> HashMap<i64, Ingredient> {
    ingredients
        .iter()
        .filter_map(|ing| ing.id.map(|id| (id, ing.clone())))
        .collect()
}

#[component]
fn RecipeIngredientFormRow(
    idx: usize,
    row: IngredientRow,
    ingredients: Arc<Vec<Ingredient>>,
    on_change: Callback<(usize, IngredientRow)>,
    on_remove: Callback<usize>,
) -> impl IntoView {
    let select_value = row
        .ingredient_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    let packages_value = row.packages.clone();
    let grams_value = row.grams.clone();

    let ingredient_options = ingredients
        .iter()
        .filter_map(|ing| ing.id.map(|id| (id, ing.name.clone(), ing.package_size_g)))
        .collect::<Vec<_>>();
    let ingredient_options_for_select = ingredient_options.clone();

    let on_select = {
        let on_change = on_change.clone();
        let row_clone = row.clone();
        move |value: String| {
            let mut next = row_clone.clone();
            let id = value.parse::<i64>().ok();
            next.ingredient_id = id;
            if let Some(selected) = id.and_then(|id| {
                ingredient_options_for_select
                    .iter()
                    .find(|(opt_id, _, _)| *opt_id == id)
            }) {
                // Default to one package when selecting an ingredient
                next.packages = "1".to_string();
                next.grams = String::new();
                let package_size = selected.2;
                if package_size > 0.0 {
                    next.grams = display_float(package_size);
                }
            }
            on_change.run((idx, next));
        }
    };

    let handle_packages = {
        let on_change = on_change.clone();
        let row_clone = row.clone();
        move |value: String| {
            let mut next = row_clone.clone();
            next.packages = value.clone();
            on_change.run((idx, next));
        }
    };

    let handle_grams = {
        let on_change = on_change.clone();
        let row_clone = row.clone();
        move |value: String| {
            let mut next = row_clone.clone();
            next.grams = value.clone();
            on_change.run((idx, next));
        }
    };

    view! {
      <div class="grid grid-cols-12 gap-3 items-end" data-test=format!("recipe-ingredient-row-{}", idx)>
        <div class="col-span-12 md:col-span-4">
          <label class="text-sm font-medium text-slate-700 mb-1 block">"Ingredient"</label>
          <select
            class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            prop:value=move || select_value.clone()
            on:change=move |ev| on_select(event_target_value(&ev))
            data-test=format!("recipe-ingredient-select-{}", idx)
          >
            <option value="">"Select ingredient"</option>
            {ingredient_options
              .clone()
              .into_iter()
              .map(|(id, name, _)| {
                view! {
                  <option value=id.to_string() selected=move || row.ingredient_id == Some(id)>
                    {name}
                  </option>
                }
              })
              .collect_view()}
          </select>
        </div>
        <div class="col-span-6 md:col-span-3">
          <label class="text-sm font-medium text-slate-700 mb-1 block">"Packages"</label>
          <input
            class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            type="number"
            step="0.1"
            min="0"
            inputmode="decimal"
            prop:value=move || packages_value.clone()
            on:input=move |ev| handle_packages(event_target_value(&ev))
            placeholder="1"
            data-test=format!("recipe-ingredient-packages-{}", idx)
          />
          <p class="text-xs text-slate-500 mt-1">"Whole package friendly—use fractions if needed."</p>
        </div>
        <div class="col-span-6 md:col-span-3">
          <label class="text-sm font-medium text-slate-700 mb-1 block">"Extra grams"</label>
          <input
            class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            type="number"
            step="1"
            min="0"
            inputmode="decimal"
            prop:value=move || grams_value.clone()
            on:input=move |ev| handle_grams(event_target_value(&ev))
            placeholder="e.g. 50"
            data-test=format!("recipe-ingredient-grams-{}", idx)
          />
          <p class="text-xs text-slate-500 mt-1">"Optional grams for oils, spices, etc."</p>
        </div>
        <div class="col-span-12 md:col-span-2 flex justify-end">
          <button
            class="text-sm text-red-600 hover:text-red-800 underline"
            on:click=move |_| on_remove.run(idx)
            data-test=format!("recipe-ingredient-remove-{}", idx)
          >
            "Remove"
          </button>
        </div>
      </div>
    }
}

#[component]
fn RecipeModalContent(
    show: RwSignal<bool>,
    editing_id: RwSignal<Option<i64>>,
    name: RwSignal<String>,
    meal_type: RwSignal<String>,
    prep_time: RwSignal<String>,
    cook_time: RwSignal<String>,
    servings: RwSignal<String>,
    tags: RwSignal<String>,
    instructions: RwSignal<String>,
    error: RwSignal<Option<String>>,
    rows: RwSignal<Vec<IngredientRow>>,
    ingredients: Arc<Vec<Ingredient>>,
    nutrition_preview: Memo<RecipeNutrition>,
    saving: RwSignal<bool>,
    add_row: Callback<()>,
    handle_row_change: Callback<(usize, IngredientRow)>,
    handle_row_remove: Callback<usize>,
    save_recipe: Callback<()>,
) -> impl IntoView {
    view! {
        <div
            id="recipe-modal-backdrop"
            class="fixed inset-0 z-50 bg-black/50 overflow-y-auto py-6"
            on:click=move |ev: web_sys::MouseEvent| {
                if let Some(target) = ev.target() {
                    if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if element.id() == "recipe-modal-backdrop" {
                            show.set(false);
                        }
                    }
                }
            }
        >
            <div class="relative mx-auto max-w-5xl rounded-lg bg-white p-6 shadow-xl w-[95%]" data-test="recipe-modal">
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="text-xl font-bold text-slate-900">
                        {move || if editing_id.get().is_some() { "Edit recipe" } else { "New recipe" }}
                    </h2>
                    <button class="text-slate-500 hover:text-slate-700" on:click=move |_| show.set(false) aria-label="Close recipe modal">
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

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-slate-700 mb-1">"Name"</label>
                                    <input
                                        class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        type="text"
                                        prop:value=name
                                        on:input=move |ev| name.set(event_target_value(&ev))
                                        placeholder="e.g. Chicken bowl"
                                        data-test="recipe-name"
                                    />
                                </div>
                                <div class="grid grid-cols-2 gap-3">
                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 mb-1">"Meal type"</label>
                                        <select
                                            class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                            prop:value=meal_type
                                            on:change=move |ev| meal_type.set(event_target_value(&ev))
                                            data-test="recipe-meal-type"
                                        >
                                            <option value="Breakfast">"Breakfast"</option>
                                            <option value="Lunch">"Lunch"</option>
                                            <option value="Dinner" selected>"Dinner"</option>
                                            <option value="Snack">"Snack"</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label class="block text-sm font-medium text-slate-700 mb-1">"Servings"</label>
                                        <input
                                            class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                            type="number"
                                            min="1"
                                            prop:value=servings
                                            on:input=move |ev| servings.set(event_target_value(&ev))
                                            data-test="recipe-servings"
                                        />
                                    </div>
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-slate-700 mb-1">"Prep time"</label>
                                    <input
                                        class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        type="text"
                                        prop:value=prep_time
                                        on:input=move |ev| prep_time.set(event_target_value(&ev))
                                        placeholder="e.g. 10 min"
                                        data-test="recipe-prep-time"
                                    />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-slate-700 mb-1">"Cook time"</label>
                                    <input
                                        class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        type="text"
                                        prop:value=cook_time
                                        on:input=move |ev| cook_time.set(event_target_value(&ev))
                                        placeholder="e.g. 20 min"
                                        data-test="recipe-cook-time"
                                    />
                                </div>
                                <div class="md:col-span-2">
                                    <label class="block text-sm font-medium text-slate-700 mb-1">"Tags (comma separated)"</label>
                                    <input
                                        class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        type="text"
                                        prop:value=tags
                                        on:input=move |ev| tags.set(event_target_value(&ev))
                                        placeholder="e.g. High Protein, Quick"
                                        data-test="recipe-tags"
                                    />
                                </div>
                                <div class="md:col-span-2">
                                    <label class="block text-sm font-medium text-slate-700 mb-1">"Instructions (one per line)"</label>
                                    <textarea
                                        class="w-full rounded border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        rows=4
                                        prop:value=instructions
                                        on:input=move |ev| instructions.set(event_target_value(&ev))
                                        placeholder="Line by line instructions"
                                        data-test="recipe-instructions"
                                    ></textarea>
                                </div>
                            </div>

                            <div class="mt-6 flex items-center justify-between">
                                <h3 class="text-lg font-semibold text-slate-900">"Ingredients"</h3>
                                <button
                                    class="flex items-center gap-2 rounded bg-green-600 px-3 py-2 text-sm font-medium text-white hover:bg-green-700"
                                    on:click=add_row
                                    data-test="recipe-add-ingredient"
                                >
                                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                                    </svg>
                                    "Add ingredient"
                                </button>
                            </div>

                            <div class="mt-4 space-y-4">
                                <For
                                    each=move || rows.get().into_iter().enumerate().collect::<Vec<_>>()
                                    key=|(idx, _)| *idx
                                    children=move |(idx, row): (usize, IngredientRow)| {
                                        view! {
                                            <RecipeIngredientFormRow
                                                idx=idx
                                                row=row
                                                ingredients=ingredients.clone()
                                                on_change=handle_row_change
                                                on_remove=handle_row_remove
                                            />
                                        }
                                    }
                                />
                            </div>

                            <div class="mt-6 rounded bg-slate-50 p-4">
                                <h4 class="font-semibold text-slate-900 mb-2">"Nutrition preview (per serving)"</h4>
                                <p class="text-sm text-slate-700">
                                    {move || {
                                        let n = nutrition_preview.get();
                                        format!(
                                            "{} kcal • {}g protein • {}g carbs • {}g fat • {}g sat fat • {}g fiber • {}g salt",
                                            display_float(n.calories),
                                            display_float(n.protein),
                                            display_float(n.carbs),
                                            display_float(n.fat),
                                            display_float(n.sat_fat),
                                            display_float(n.fiber),
                                            display_float(n.salt),
                                        )
                                    }}
                                </p>
                            </div>

                            <div class="mt-6 flex justify-end gap-3">
                                <button
                                    class="rounded bg-slate-200 px-4 py-2 font-medium text-slate-700 hover:bg-slate-300"
                                    on:click=move |_| show.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    class="rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300"
                                    disabled=move || saving.get()
                                    on:click=save_recipe
                                    data-test="recipe-save"
                                >
                                    {move || if saving.get() { "Saving..." } else { "Save recipe" }}
                                </button>
                            </div>
                        </div>
                    </div>
        </Show>
    }
}

#[component]
fn RecipeCard(
    recipe: Recipe,
    on_edit: Callback<Recipe>,
    on_delete: Callback<i64>,
    is_admin: ReadSignal<bool>,
) -> impl IntoView {
    let (meal_bg, meal_text) = get_meal_type_color(&recipe.meal_type);

    view! {
      <div class="rounded-lg bg-white p-6 shadow-md" data-test="recipe-card">
        <div class="mb-4 flex items-center justify-between gap-3">
          <div>
            <h3 class="text-2xl font-bold text-slate-900">{recipe.name.clone()}</h3>
            <p class="text-sm text-slate-600">
              {format!("Prep: {} | Cook: {} | Servings: {}", recipe.prep_time, recipe.cook_time, recipe.servings)}
            </p>
          </div>
          <span class=format!(
            "rounded px-3 py-1 text-sm font-medium {} {}",
            meal_bg,
            meal_text,
          )>{recipe.meal_type.clone()}</span>
        </div>

        <div class="mb-3 flex flex-wrap gap-2">
          {recipe
            .tags
            .iter()
            .map(|tag| {
              let (bg, text) = get_tag_color(tag);
              view! { <span class=format!("rounded px-2 py-1 text-xs {} {}", bg, text)>{tag.clone()}</span> }
            })
            .collect_view()}
        </div>

        <h4 class="mb-2 font-semibold text-slate-900">"Ingredients"</h4>
        <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
          {recipe
            .ingredients
            .iter()
            .map(|ing| {
              let mut parts = Vec::new();
              if ing.packages > 0.0 {
                parts.push(format!("{} package(s)", display_float(ing.packages)));
              }
              if let Some(g) = ing.grams {
                parts.push(format!("{}g", display_float(g)));
              } else if ing.packages > 0.0 && ing.package_size_g > 0.0 {
                parts.push(format!("~{}g", display_float(ing.amount_g)));
              }
              let amount = if parts.is_empty() { String::from("as needed") } else { parts.join(" + ") };
              view! { <li>{format!("{} — {}", ing.ingredient_name, amount)}</li> }
            })
            .collect_view()}
        </ul>

        <h4 class="mb-2 font-semibold text-slate-900">"Instructions"</h4>
        <ol class="list-inside list-decimal space-y-1 text-slate-700">
          {recipe.instructions.iter().map(|step| view! { <li>{step.clone()}</li> }).collect_view()}
        </ol>

        <div class="mt-4 rounded bg-slate-50 p-3">
          <p class="text-sm font-medium text-slate-900">
            {format!(
              "Nutrition per serving: {} kcal | {}g protein | {}g carbs | {}g fat",
              display_float(recipe.nutrition.calories),
              display_float(recipe.nutrition.protein),
              display_float(recipe.nutrition.carbs),
              display_float(recipe.nutrition.fat),
            )}
          </p>
          <p class="text-sm text-slate-600 mt-1">
            {format!(
              "Sat. fat: {}g | Salt: {}g | Fiber: {}g",
              display_float(recipe.nutrition.sat_fat),
              display_float(recipe.nutrition.salt),
              display_float(recipe.nutrition.fiber),
            )}
          </p>
        </div>

        <Show when=move || is_admin.get()>
          <div class="mt-4 flex gap-3">
            <button
              class="rounded bg-blue-600 px-3 py-2 text-sm font-medium text-white hover:bg-blue-700"
              on:click={
                let recipe = recipe.clone();
                let on_edit = on_edit.clone();
                move |_| on_edit.run(recipe.clone())
              }
              data-test=format!("recipe-edit-{}", recipe.id)
            >
              "Edit"
            </button>
            <button
              class="rounded bg-red-600 px-3 py-2 text-sm font-medium text-white hover:bg-red-700"
              on:click={
                let on_delete = on_delete.clone();
                move |_| {
                  let confirmed = web_sys::window()
                    .and_then(|w| w.confirm_with_message("Delete this recipe?").ok())
                    .unwrap_or(false);
                  if confirmed {
                    on_delete.run(recipe.id);
                  }
                }
              }
              data-test=format!("recipe-delete-{}", recipe.id)
            >
              "Delete"
            </button>
          </div>
        </Show>
      </div>
    }
}

#[component]
fn RecipeModal(
    show: RwSignal<bool>,
    existing: Option<Recipe>,
    ingredients: Arc<Vec<Ingredient>>,
    on_saved: Callback<()>,
) -> impl IntoView {
    let empty_row = IngredientRow::new();
    let rows = RwSignal::new(vec![empty_row.clone()]);

    let name = RwSignal::new(String::new());
    let meal_type = RwSignal::new(String::from("Dinner"));
    let prep_time = RwSignal::new(String::new());
    let cook_time = RwSignal::new(String::new());
    let servings = RwSignal::new(String::from("2"));
    let tags = RwSignal::new(String::new());
    let instructions = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let saving = RwSignal::new(false);
    let editing_id = RwSignal::new(Option::<i64>::None);

    // Prefill when editing changes
    Effect::new(move || {
        if let Some(recipe) = existing.clone() {
            editing_id.set(Some(recipe.id));
            name.set(recipe.name);
            meal_type.set(recipe.meal_type);
            prep_time.set(recipe.prep_time);
            cook_time.set(recipe.cook_time);
            servings.set(recipe.servings.to_string());
            tags.set(recipe.tags.join(", "));
            instructions.set(recipe.instructions.join("\n"));
            rows.set(
                recipe
                    .ingredients
                    .iter()
                    .map(IngredientRow::from_recipe)
                    .collect(),
            );
        } else {
            editing_id.set(None);
            name.set(String::new());
            meal_type.set(String::from("Dinner"));
            prep_time.set(String::new());
            cook_time.set(String::new());
            servings.set(String::from("2"));
            tags.set(String::new());
            instructions.set(String::new());
            rows.set(vec![empty_row.clone()]);
        }
        error.set(None);
    });

    let ingredient_map = ingredient_lookup(ingredients.as_ref());

    let nutrition_preview = Memo::new({
        let rows = rows.clone();
        let servings = servings.clone();
        move |_| {
            let parsed_rows = rows.get();
            let input_rows = parsed_rows
                .iter()
                .filter_map(|row| {
                    let ingredient_id = row.ingredient_id?;
                    Some(RecipeIngredientInput {
                        ingredient_id,
                        packages: row.packages.trim().parse::<f32>().unwrap_or(0.0),
                        grams: row.grams.trim().parse::<f32>().ok().filter(|v| *v > 0.0),
                    })
                })
                .collect::<Vec<_>>();
            let servings_val = servings.get().parse::<i32>().unwrap_or(1);
            compute_nutrition(&input_rows, &ingredient_map, servings_val)
        }
    });

    let save_recipe = {
        let rows = rows.clone();
        let name = name.clone();
        let meal_type = meal_type.clone();
        let prep_time = prep_time.clone();
        let cook_time = cook_time.clone();
        let servings = servings.clone();
        let tags = tags.clone();
        let instructions = instructions.clone();
        let editing_id = editing_id.clone();
        let saving = saving.clone();
        let show = show.clone();
        let on_saved = on_saved.clone();
        let error = error.clone();
        Callback::new(move |_| {
            error.set(None);

            let mut input = build_input_from_form(
                name.get(),
                meal_type.get(),
                prep_time.get(),
                cook_time.get(),
                servings.get(),
                tags.get(),
                instructions.get(),
                &rows.get(),
            )
            .map_err(|e| error.set(Some(e)))
            .ok();

            if let Some(id) = editing_id.get() {
                if let Some(ref mut inp) = input {
                    inp.id = Some(id);
                }
            }

            if let Some(input) = input {
                saving.set(true);
                let on_saved = on_saved.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match upsert_recipe(input).await {
                        Ok(_) => {
                            saving.set(false);
                            show.set(false);
                            on_saved.run(());
                        }
                        Err(e) => {
                            saving.set(false);
                            error.set(Some(format!("Failed to save: {}", e)));
                        }
                    }
                });
            }
        })
    };

    let add_row = {
        let rows = rows.clone();
        Callback::new(move |_| {
            rows.update(|list| list.push(IngredientRow::new()));
        })
    };

    let handle_row_change = {
        let rows = rows.clone();
        Callback::new(move |(idx, updated): (usize, IngredientRow)| {
            rows.update(|list| {
                if let Some(row) = list.get_mut(idx) {
                    *row = updated;
                }
            });
        })
    };

    let handle_row_remove = {
        let rows = rows.clone();
        Callback::new(move |idx: usize| {
            rows.update(|list| {
                if list.len() > 1 && idx < list.len() {
                    list.remove(idx);
                }
            });
        })
    };

    let ingredients_clone = ingredients.clone();
    view! {
      <Show when=move || show.get() fallback=|| ()>
        <RecipeModalContent
          show=show
          editing_id=editing_id
          name=name
          meal_type=meal_type
          prep_time=prep_time
          cook_time=cook_time
          servings=servings
          tags=tags
          instructions=instructions
          error=error
          rows=rows
          ingredients=ingredients_clone
          nutrition_preview=nutrition_preview
          saving=saving
          add_row=add_row
          handle_row_change=handle_row_change
          handle_row_remove=handle_row_remove
          save_recipe=save_recipe
        />
      </Show>
    }
}

#[component]
pub fn Recipes() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let recipes_resource = Resource::new(|| (), |_| get_recipes());
    let ingredients_resource = Resource::new(|| (), |_| get_ingredients());

    let show_modal = RwSignal::new(false);
    let editing_recipe = RwSignal::new(Option::<Recipe>::None);

    let handle_new = move |_| {
        editing_recipe.set(None);
        show_modal.set(true);
    };

    let handle_edit = {
        let editing_recipe = editing_recipe.clone();
        let show_modal = show_modal.clone();
        Callback::new(move |recipe: Recipe| {
            editing_recipe.set(Some(recipe));
            show_modal.set(true);
        })
    };

    let refetch_all = {
        let recipes_resource = recipes_resource.clone();
        move || recipes_resource.refetch()
    };

    let handle_delete = {
        let recipes_resource = recipes_resource.clone();
        Callback::new(move |id: i64| {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = delete_recipe(id).await;
                recipes_resource.refetch();
            });
        })
    };

    view! {
      <div class="mx-auto max-w-7xl py-6">
        <div class="mb-6 flex items-center justify-between flex-wrap gap-4">
          <h2 class="text-3xl font-bold text-slate-900">"Recipes"</h2>
          <Show when=move || auth.is_authenticated.get()>
            <button
              class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
              on:click=handle_new
              data-test="recipe-new-button"
            >
              <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              "New recipe"
            </button>
          </Show>
        </div>

        <Suspense fallback=move || {
          view! { <p class="text-slate-600">"Loading recipes..."</p> }
        }>
          {move || {
            let recipes_result = recipes_resource.get();
            let ingredients_result = ingredients_resource.get();
            match (recipes_result, ingredients_result) {
              (Some(Ok(recipes)), Some(Ok(_ingredients))) => {
                if recipes.is_empty() {

                  view! {
                    <div
                      class="rounded-lg border border-dashed border-slate-300 bg-white p-8 text-center text-slate-600"
                      data-test="recipes-empty-state"
                    >
                      "No recipes yet. Unlock and add your first recipe."
                    </div>
                  }
                    .into_any()
                } else {
                  view! {
                    <div class="grid gap-6 lg:grid-cols-2">
                      <For
                        each=move || recipes.clone()
                        key=|recipe| recipe.id
                        children=move |recipe: Recipe| {
                          view! {
                            <RecipeCard
                              recipe=recipe
                              on_edit=handle_edit.clone()
                              on_delete=handle_delete.clone()
                              is_admin=auth.is_authenticated.read_only()
                            />
                          }
                        }
                      />
                    </div>
                  }
                    .into_any()
                }
              }
              (Some(Err(e)), _) => {
                view! {
                  <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                    <p class="font-medium">"Failed to load recipes"</p>
                    <p class="text-sm">{e.to_string()}</p>
                  </div>
                }
                  .into_any()
              }
              (_, Some(Err(e))) => {
                view! {
                  <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                    <p class="font-medium">"Failed to load ingredients"</p>
                    <p class="text-sm">{e.to_string()}</p>
                  </div>
                }
                  .into_any()
              }
              _ => view! { <p class="text-slate-600">"Loading..."</p> }.into_any(),
            }
          }}
        </Suspense>
        <RecipeModal
          show=show_modal
          existing=editing_recipe.get()
          ingredients=Arc::new(ingredients_resource.get().and_then(Result::ok).unwrap_or_default())
          on_saved=Callback::new(move |_| refetch_all())
        />
      </div>
    }
}
