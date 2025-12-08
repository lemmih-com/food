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

/// Ingredient category for organizing into separate tables
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IngredientCategory {
    Protein,
    Carbs,
    Veggies,
    Other,
}

impl IngredientCategory {
    pub fn title(&self) -> &'static str {
        match self {
            IngredientCategory::Protein => "Proteins",
            IngredientCategory::Carbs => "Carbs",
            IngredientCategory::Veggies => "Vegetables",
            IngredientCategory::Other => "Other",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            IngredientCategory::Protein => "protein",
            IngredientCategory::Carbs => "carbs",
            IngredientCategory::Veggies => "veggies",
            IngredientCategory::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "protein" => Some(IngredientCategory::Protein),
            "carbs" => Some(IngredientCategory::Carbs),
            "veggies" => Some(IngredientCategory::Veggies),
            "other" => Some(IngredientCategory::Other),
            _ => None,
        }
    }

    pub fn all() -> [IngredientCategory; 4] {
        [
            IngredientCategory::Protein,
            IngredientCategory::Carbs,
            IngredientCategory::Veggies,
            IngredientCategory::Other,
        ]
    }
}

/// All nutrient values are per 100g
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ingredient {
    pub id: Option<i64>,
    pub name: String,
    pub category: IngredientCategory,
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
    pub store: String,
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

    /// Create a new empty ingredient with a given category
    pub fn new_empty(category: IngredientCategory) -> Self {
        Self {
            id: None,
            name: String::new(),
            category,
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
            store: String::new(),
        }
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Self::new_empty(IngredientCategory::Other)
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
            SortDirection::Ascending => " \u{25B2}",  // up arrow
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
// Default Data (for seeding empty database)
// ============================================================================

fn get_default_ingredients() -> Vec<Ingredient> {
    vec![
        // Proteins
        Ingredient {
            id: None,
            name: "Chicken Breast".to_string(),
            category: IngredientCategory::Protein,
            calories: 165.0,
            protein: 31.0,
            fat: 3.6,
            saturated_fat: 1.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 74.0,
            package_size_g: 500.0,
            package_price: 8.99,
            store: "Whole Foods".to_string(),
        },
        Ingredient {
            id: None,
            name: "Salmon".to_string(),
            category: IngredientCategory::Protein,
            calories: 208.0,
            protein: 20.0,
            fat: 13.0,
            saturated_fat: 3.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 59.0,
            package_size_g: 400.0,
            package_price: 12.99,
            store: "Costco".to_string(),
        },
        Ingredient {
            id: None,
            name: "Eggs (dozen)".to_string(),
            category: IngredientCategory::Protein,
            calories: 155.0,
            protein: 13.0,
            fat: 11.0,
            saturated_fat: 3.3,
            carbs: 1.1,
            sugar: 1.1,
            fiber: 0.0,
            salt: 124.0,
            package_size_g: 720.0,
            package_price: 4.99,
            store: "All stores".to_string(),
        },
        Ingredient {
            id: None,
            name: "Ground Beef (lean)".to_string(),
            category: IngredientCategory::Protein,
            calories: 250.0,
            protein: 26.0,
            fat: 15.0,
            saturated_fat: 6.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 75.0,
            package_size_g: 500.0,
            package_price: 7.99,
            store: "Safeway".to_string(),
        },
        Ingredient {
            id: None,
            name: "Tofu (firm)".to_string(),
            category: IngredientCategory::Protein,
            calories: 144.0,
            protein: 17.0,
            fat: 9.0,
            saturated_fat: 1.3,
            carbs: 3.0,
            sugar: 1.0,
            fiber: 2.0,
            salt: 14.0,
            package_size_g: 400.0,
            package_price: 2.99,
            store: "Trader Joe's".to_string(),
        },
        // Carbs
        Ingredient {
            id: None,
            name: "Brown Rice".to_string(),
            category: IngredientCategory::Carbs,
            calories: 112.0,
            protein: 2.6,
            fat: 0.9,
            saturated_fat: 0.2,
            carbs: 24.0,
            sugar: 0.4,
            fiber: 1.8,
            salt: 1.0,
            package_size_g: 907.0,
            package_price: 3.99,
            store: "Trader Joe's".to_string(),
        },
        Ingredient {
            id: None,
            name: "Pasta (whole wheat)".to_string(),
            category: IngredientCategory::Carbs,
            calories: 124.0,
            protein: 5.3,
            fat: 0.5,
            saturated_fat: 0.1,
            carbs: 25.0,
            sugar: 0.6,
            fiber: 4.5,
            salt: 4.0,
            package_size_g: 454.0,
            package_price: 2.49,
            store: "Safeway".to_string(),
        },
        Ingredient {
            id: None,
            name: "Oats (rolled)".to_string(),
            category: IngredientCategory::Carbs,
            calories: 389.0,
            protein: 16.9,
            fat: 6.9,
            saturated_fat: 1.2,
            carbs: 66.0,
            sugar: 0.0,
            fiber: 10.6,
            salt: 2.0,
            package_size_g: 510.0,
            package_price: 4.49,
            store: "Trader Joe's".to_string(),
        },
        Ingredient {
            id: None,
            name: "Quinoa".to_string(),
            category: IngredientCategory::Carbs,
            calories: 120.0,
            protein: 4.4,
            fat: 1.9,
            saturated_fat: 0.2,
            carbs: 21.0,
            sugar: 0.9,
            fiber: 2.8,
            salt: 7.0,
            package_size_g: 340.0,
            package_price: 5.99,
            store: "Whole Foods".to_string(),
        },
        Ingredient {
            id: None,
            name: "Bread (whole grain)".to_string(),
            category: IngredientCategory::Carbs,
            calories: 247.0,
            protein: 13.0,
            fat: 4.2,
            saturated_fat: 0.8,
            carbs: 41.0,
            sugar: 6.0,
            fiber: 7.0,
            salt: 450.0,
            package_size_g: 680.0,
            package_price: 4.99,
            store: "Safeway".to_string(),
        },
        // Vegetables
        Ingredient {
            id: None,
            name: "Broccoli".to_string(),
            category: IngredientCategory::Veggies,
            calories: 34.0,
            protein: 2.8,
            fat: 0.4,
            saturated_fat: 0.0,
            carbs: 7.0,
            sugar: 1.7,
            fiber: 2.6,
            salt: 33.0,
            package_size_g: 350.0,
            package_price: 2.49,
            store: "Safeway".to_string(),
        },
        Ingredient {
            id: None,
            name: "Spinach (fresh)".to_string(),
            category: IngredientCategory::Veggies,
            calories: 23.0,
            protein: 2.9,
            fat: 0.4,
            saturated_fat: 0.0,
            carbs: 3.6,
            sugar: 0.4,
            fiber: 2.2,
            salt: 79.0,
            package_size_g: 142.0,
            package_price: 3.99,
            store: "Trader Joe's".to_string(),
        },
        Ingredient {
            id: None,
            name: "Bell Peppers".to_string(),
            category: IngredientCategory::Veggies,
            calories: 31.0,
            protein: 1.0,
            fat: 0.3,
            saturated_fat: 0.0,
            carbs: 6.0,
            sugar: 4.2,
            fiber: 2.1,
            salt: 4.0,
            package_size_g: 300.0,
            package_price: 3.99,
            store: "Whole Foods".to_string(),
        },
        Ingredient {
            id: None,
            name: "Carrots".to_string(),
            category: IngredientCategory::Veggies,
            calories: 41.0,
            protein: 0.9,
            fat: 0.2,
            saturated_fat: 0.0,
            carbs: 10.0,
            sugar: 4.7,
            fiber: 2.8,
            salt: 69.0,
            package_size_g: 454.0,
            package_price: 1.99,
            store: "All stores".to_string(),
        },
        Ingredient {
            id: None,
            name: "Tomatoes (canned)".to_string(),
            category: IngredientCategory::Veggies,
            calories: 18.0,
            protein: 0.9,
            fat: 0.1,
            saturated_fat: 0.0,
            carbs: 4.0,
            sugar: 2.6,
            fiber: 1.0,
            salt: 9.0,
            package_size_g: 400.0,
            package_price: 1.49,
            store: "All stores".to_string(),
        },
        // Other
        Ingredient {
            id: None,
            name: "Olive Oil".to_string(),
            category: IngredientCategory::Other,
            calories: 884.0,
            protein: 0.0,
            fat: 100.0,
            saturated_fat: 14.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 2.0,
            package_size_g: 500.0,
            package_price: 9.99,
            store: "Trader Joe's".to_string(),
        },
        Ingredient {
            id: None,
            name: "Butter".to_string(),
            category: IngredientCategory::Other,
            calories: 717.0,
            protein: 0.9,
            fat: 81.0,
            saturated_fat: 51.0,
            carbs: 0.1,
            sugar: 0.1,
            fiber: 0.0,
            salt: 714.0,
            package_size_g: 227.0,
            package_price: 4.99,
            store: "All stores".to_string(),
        },
        Ingredient {
            id: None,
            name: "Greek Yogurt".to_string(),
            category: IngredientCategory::Other,
            calories: 59.0,
            protein: 10.0,
            fat: 0.7,
            saturated_fat: 0.1,
            carbs: 3.6,
            sugar: 3.2,
            fiber: 0.0,
            salt: 36.0,
            package_size_g: 450.0,
            package_price: 5.49,
            store: "Trader Joe's".to_string(),
        },
        Ingredient {
            id: None,
            name: "Cheese (cheddar)".to_string(),
            category: IngredientCategory::Other,
            calories: 403.0,
            protein: 25.0,
            fat: 33.0,
            saturated_fat: 21.0,
            carbs: 1.3,
            sugar: 0.5,
            fiber: 0.0,
            salt: 621.0,
            package_size_g: 227.0,
            package_price: 5.99,
            store: "Costco".to_string(),
        },
        Ingredient {
            id: None,
            name: "Honey".to_string(),
            category: IngredientCategory::Other,
            calories: 304.0,
            protein: 0.3,
            fat: 0.0,
            saturated_fat: 0.0,
            carbs: 82.0,
            sugar: 82.0,
            fiber: 0.0,
            salt: 4.0,
            package_size_g: 340.0,
            package_price: 7.99,
            store: "Whole Foods".to_string(),
        },
    ]
}

// ============================================================================
// Server Functions
// ============================================================================

/// Fetch all ingredients from D1 database
/// If database is empty, populates with default data
#[server]
pub async fn get_ingredients() -> Result<Vec<Ingredient>, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    // Check if table is empty
    let count_result = SendWrapper::new(async {
        let count_stmt = db
            .inner()
            .prepare("SELECT COUNT(*) as count FROM ingredients");
        count_stmt.first::<serde_json::Value>(None).await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 count error: {:?}", e)))?;

    let is_empty = count_result
        .and_then(|v| v.get("count").and_then(|c| c.as_i64()))
        .map(|c| c == 0)
        .unwrap_or(true);

    // If empty, seed with default data
    if is_empty {
        log::info!("Ingredients table is empty, seeding with default data");
        let defaults = get_default_ingredients();
        for ing in defaults {
            SendWrapper::new(async {
                let stmt = db.inner().prepare(
                    "INSERT INTO ingredients (name, category, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price, store) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                );
                let stmt = stmt
                    .bind(&[
                        ing.name.into(),
                        ing.category.as_str().into(),
                        ing.calories.into(),
                        ing.protein.into(),
                        ing.fat.into(),
                        ing.saturated_fat.into(),
                        ing.carbs.into(),
                        ing.sugar.into(),
                        ing.fiber.into(),
                        ing.salt.into(),
                        ing.package_size_g.into(),
                        ing.package_price.into(),
                        ing.store.into(),
                    ])?;
                stmt.run().await
            })
            .await
            .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;
        }
    }

    // Fetch all ingredients
    let results = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "SELECT id, name, category, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price, store FROM ingredients ORDER BY name"
        );
        stmt.all().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 query error: {:?}", e)))?;

    let ingredients: Vec<Ingredient> = results
        .results::<serde_json::Value>()
        .map_err(|e| ServerFnError::new(format!("D1 results error: {:?}", e)))?
        .into_iter()
        .filter_map(|row| {
            let category_str = row.get("category")?.as_str()?;
            let category = IngredientCategory::from_str(category_str)?;
            Some(Ingredient {
                id: row.get("id")?.as_i64(),
                name: row.get("name")?.as_str()?.to_string(),
                category,
                calories: row.get("calories")?.as_f64()? as f32,
                protein: row.get("protein")?.as_f64()? as f32,
                fat: row.get("fat")?.as_f64()? as f32,
                saturated_fat: row.get("saturated_fat")?.as_f64()? as f32,
                carbs: row.get("carbs")?.as_f64()? as f32,
                sugar: row.get("sugar")?.as_f64()? as f32,
                fiber: row.get("fiber")?.as_f64()? as f32,
                salt: row.get("salt")?.as_f64()? as f32,
                package_size_g: row.get("package_size_g")?.as_f64()? as f32,
                package_price: row.get("package_price")?.as_f64()? as f32,
                store: row.get("store")?.as_str()?.to_string(),
            })
        })
        .collect();

    Ok(ingredients)
}

/// Create a new ingredient
#[server]
pub async fn create_ingredient(ingredient: Ingredient) -> Result<Ingredient, ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    let result = SendWrapper::new(async {
        let stmt = db.inner().prepare(
            "INSERT INTO ingredients (name, category, calories, protein, fat, saturated_fat, carbs, sugar, fiber, salt, package_size_g, package_price, store) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id"
        );

        let stmt = stmt
            .bind(&[
                ingredient.name.clone().into(),
                ingredient.category.as_str().into(),
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
                ingredient.store.clone().into(),
            ])?;

        stmt.first::<serde_json::Value>(None).await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 insert error: {:?}", e)))?;

    let id = result
        .and_then(|v| v.get("id").and_then(|id| id.as_i64()))
        .ok_or_else(|| ServerFnError::new("Failed to get inserted ID"))?;

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
            "UPDATE ingredients SET name = ?, category = ?, calories = ?, protein = ?, fat = ?, saturated_fat = ?, carbs = ?, sugar = ?, fiber = ?, salt = ?, package_size_g = ?, package_price = ?, store = ?, updated_at = datetime('now') WHERE id = ?"
        );

        let stmt = stmt
            .bind(&[
                ingredient.name.into(),
                ingredient.category.as_str().into(),
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
                ingredient.store.into(),
                id.into(),
            ])?;

        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 update error: {:?}", e)))?;

    log::info!("Updated ingredient id: {}", id);
    Ok(())
}

/// Delete an ingredient
#[server]
pub async fn delete_ingredient(id: i64) -> Result<(), ServerFnError> {
    use send_wrapper::SendWrapper;

    let db = expect_context::<SendD1Database>();

    SendWrapper::new(async {
        let stmt = db.inner().prepare("DELETE FROM ingredients WHERE id = ?");
        let stmt = stmt.bind(&[id.into()])?;
        stmt.run().await
    })
    .await
    .map_err(|e| ServerFnError::new(format!("D1 delete error: {:?}", e)))?;

    log::info!("Deleted ingredient id: {}", id);
    Ok(())
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
            class=format!("px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider cursor-pointer hover:bg-slate-100 select-none {}", width_class)
            on:click=move |_| on_click(col)
        >
            <span class="inline-flex items-center gap-1">
                {label}
                <span class="w-3 inline-block text-center">
                    {move || {
                        if sort_column.get() == col {
                            sort_direction.get().indicator()
                        } else {
                            ""
                        }
                    }}
                </span>
            </span>
        </th>
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
    let category = RwSignal::new(IngredientCategory::Other);
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
    let store = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let saving = RwSignal::new(false);

    // Populate form when editing changes
    Effect::new(move || {
        if let Some(ing) = editing.get() {
            name.set(ing.name.clone());
            category.set(ing.category);
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
            store.set(ing.store.clone());
        } else {
            // Reset form for new ingredient
            name.set(String::new());
            category.set(IngredientCategory::Other);
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
            store.set(String::new());
        }
        error.set(None);
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
                error.set(Some("Name is required".to_string()));
                return;
            }

            let ingredient = Ingredient {
                id: editing.get().and_then(|e| e.id),
                name: name_val,
                category: category.get(),
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
                store: store.get(),
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
                        <button
                            class="text-slate-500 hover:text-slate-700"
                            on:click=move |_| close()
                        >
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                        </button>
                    </div>

                    <Show when=move || error.get().is_some()>
                        <div class="mb-4 rounded bg-red-100 px-4 py-2 text-sm text-red-700">
                            {move || error.get().unwrap_or_default()}
                        </div>
                    </Show>

                    <div class="grid grid-cols-2 gap-4">
                        // Name and Category
                        <div class="col-span-2 sm:col-span-1">
                            <label class=label_class>"Name"</label>
                            <input
                                type="text"
                                class=input_class
                                prop:value=move || name.get()
                                on:input=move |ev| name.set(event_target_value(&ev))
                                placeholder="e.g., Chicken Breast"
                            />
                        </div>
                        <div class="col-span-2 sm:col-span-1">
                            <label class=label_class>"Category"</label>
                            <select
                                class=input_class
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    if let Some(cat) = IngredientCategory::from_str(&value) {
                                        category.set(cat);
                                    }
                                }
                            >
                                {IngredientCategory::all().into_iter().map(|cat| {
                                    view! {
                                        <option
                                            value=cat.as_str()
                                            selected=move || category.get() == cat
                                        >
                                            {cat.title()}
                                        </option>
                                    }
                                }).collect_view()}
                            </select>
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
                        <div class="col-span-2">
                            <label class=label_class>"Store"</label>
                            <input
                                type="text"
                                class=input_class
                                prop:value=move || store.get()
                                on:input=move |ev| store.set(event_target_value(&ev))
                                placeholder="e.g., Whole Foods"
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

#[component]
fn IngredientTable(
    title: &'static str,
    category: IngredientCategory,
    ingredients: ReadSignal<Vec<Ingredient>>,
    view_mode: ReadSignal<NutrientView>,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    on_header_click: impl Fn(SortColumn) + Clone + Send + Sync + 'static,
    on_edit: impl Fn(Ingredient) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let get_sorted_ingredients = move || {
        let mut filtered: Vec<Ingredient> = ingredients
            .get()
            .into_iter()
            .filter(|i| i.category == category)
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
    let w_name = "w-36"; // Ingredient name
    let w_pkg = "w-20"; // Package size
    let w_price = "w-16"; // Price
    let w_cal = "w-20"; // Calories
    let w_nutr = "w-16"; // Nutrient columns (protein, fat, etc.)
    let w_salt = "w-20"; // Salt (needs more space for mg)
    let w_store = "w-28"; // Store
    let w_actions = "w-16"; // Actions column

    let cell_class = "px-3 py-3 whitespace-nowrap text-slate-700";

    view! {
        <div class="mb-8">
            <h3 class="mb-3 text-xl font-semibold text-slate-800">{title}</h3>
            <div class="rounded-lg bg-white shadow-md overflow-hidden overflow-x-auto">
                <table class="w-full table-fixed divide-y divide-slate-200 text-sm">
                    <thead class="bg-slate-50">
                        <tr>
                            <SortableHeader col=SortColumn::Name label="Ingredient" width_class=w_name sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::PackageSize label="Package" width_class=w_pkg sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Price label="Price" width_class=w_price sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Calories label="Calories" width_class=w_cal sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Protein label="Protein" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Fat label="Fat" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::SaturatedFat label="Sat. Fat" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Carbs label="Carbs" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Sugar label="Sugar" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Fiber label="Fiber" width_class=w_nutr sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <SortableHeader col=SortColumn::Salt label="Salt" width_class=w_salt sort_column=sort_column sort_direction=sort_direction on_click=on_header_click.clone() />
                            <th class=format!("px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider {}", w_store)>"Store"</th>
                            <Show when=move || auth.is_authenticated.get()>
                                <th class=format!("px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider {}", w_actions)></th>
                            </Show>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <For
                            each=get_sorted_ingredients
                            key=|ing| ing.id.unwrap_or(0)
                            let:ing
                        >
                            {
                                // Create separate clones for each closure that needs the ingredient
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
                                view! {
                                    <tr class="hover:bg-slate-50">
                                        <td class=format!("{} font-medium text-slate-900 truncate", cell_class)>{ing.name.clone()}</td>
                                        <td class=cell_class>{format!("{}g", ing.package_size_g)}</td>
                                        <td class=cell_class>{format!("${:.2}", ing.package_price)}</td>
                                        <td class=cell_class>
                                            {move || {
                                                let cal = if view_mode.get() == NutrientView::Per100kcal { 100.0 } else { ing_cal.calories };
                                                format!("{:.0} kcal", cal)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_protein.per_calorie(ing_protein.protein) } else { ing_protein.protein };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_fat.per_calorie(ing_fat.fat) } else { ing_fat.fat };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_sat_fat.per_calorie(ing_sat_fat.saturated_fat) } else { ing_sat_fat.saturated_fat };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_carbs.per_calorie(ing_carbs.carbs) } else { ing_carbs.carbs };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_sugar.per_calorie(ing_sugar.sugar) } else { ing_sugar.sugar };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_fiber.per_calorie(ing_fiber.fiber) } else { ing_fiber.fiber };
                                                format!("{:.1}g", val)
                                            }}
                                        </td>
                                        <td class=cell_class>
                                            {move || {
                                                let val = if view_mode.get() == NutrientView::Per100kcal { ing_salt.per_calorie(ing_salt.salt) } else { ing_salt.salt };
                                                format!("{:.0}mg", val)
                                            }}
                                        </td>
                                        <td class=format!("{} truncate", cell_class)>{ing.store.clone()}</td>
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
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
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
            <p class="mt-2 text-xs text-slate-500">
                {move || {
                    let suffix = if view_mode.get() == NutrientView::Per100kcal { "/100kcal" } else { "/100g" };
                    format!("* Nutrient values shown {}", suffix)
                }}
            </p>
        </div>
    }
}

#[component]
pub fn Ingredients() -> impl IntoView {
    let auth = expect_context::<AdminAuth>();

    let (view_mode, set_view_mode) = signal(NutrientView::Per100g);
    let (sort_column, set_sort_column) = signal(SortColumn::Name);
    let (sort_direction, set_sort_direction) = signal(SortDirection::None);

    // Modal state
    let show_modal = RwSignal::new(false);
    let editing_ingredient = RwSignal::new(Option::<Ingredient>::None);

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
                <h2 class="text-3xl font-bold text-slate-900">"Ingredient List"</h2>
                <div class="flex items-center gap-3 flex-wrap">
                    <Show when=move || auth.is_authenticated.get()>
                        <button
                            class="flex items-center gap-2 rounded bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
                            on:click=handle_new
                        >
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                            </svg>
                            "New Ingredient"
                        </button>
                    </Show>
                    <div class="flex items-center gap-3 bg-white rounded-lg px-4 py-2 shadow-sm">
                        <span class="text-sm font-medium text-slate-700">"View nutrients:"</span>
                        <button
                            class=move || {
                                let base = "px-3 py-1 text-sm font-medium rounded transition-colors";
                                if view_mode.get() == NutrientView::Per100g {
                                    format!("{} bg-blue-600 text-white", base)
                                } else {
                                    format!("{} bg-slate-100 text-slate-700 hover:bg-slate-200", base)
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
                                    format!("{} bg-slate-100 text-slate-700 hover:bg-slate-200", base)
                                }
                            }
                            on:click=move |_| set_view_mode.set(NutrientView::Per100kcal)
                        >
                            "per 100kcal"
                        </button>
                    </div>
                </div>
            </div>

            <Suspense fallback=move || view! { <p class="text-slate-600">"Loading ingredients..."</p> }>
                {move || {
                    ingredients_resource.get().map(|result| {
                        match result {
                            Ok(ings) => {
                                let (ingredients, _) = signal(ings);
                                view! {
                                    <IngredientTable
                                        title=IngredientCategory::Protein.title()
                                        category=IngredientCategory::Protein
                                        ingredients=ingredients
                                        view_mode=view_mode
                                        sort_column=sort_column
                                        sort_direction=sort_direction
                                        on_header_click=handle_header_click.clone()
                                        on_edit=handle_edit.clone()
                                    />

                                    <IngredientTable
                                        title=IngredientCategory::Carbs.title()
                                        category=IngredientCategory::Carbs
                                        ingredients=ingredients
                                        view_mode=view_mode
                                        sort_column=sort_column
                                        sort_direction=sort_direction
                                        on_header_click=handle_header_click.clone()
                                        on_edit=handle_edit.clone()
                                    />

                                    <IngredientTable
                                        title=IngredientCategory::Veggies.title()
                                        category=IngredientCategory::Veggies
                                        ingredients=ingredients
                                        view_mode=view_mode
                                        sort_column=sort_column
                                        sort_direction=sort_direction
                                        on_header_click=handle_header_click.clone()
                                        on_edit=handle_edit.clone()
                                    />

                                    <IngredientTable
                                        title=IngredientCategory::Other.title()
                                        category=IngredientCategory::Other
                                        ingredients=ingredients
                                        view_mode=view_mode
                                        sort_column=sort_column
                                        sort_direction=sort_direction
                                        on_header_click=handle_header_click
                                        on_edit=handle_edit.clone()
                                    />
                                }.into_any()
                            }
                            Err(e) => {
                                view! {
                                    <div class="rounded bg-red-100 px-4 py-3 text-red-700">
                                        <p class="font-medium">"Failed to load ingredients"</p>
                                        <p class="text-sm">{e.to_string()}</p>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>

            <IngredientModal
                show=show_modal
                editing=editing_ingredient
                on_save=refetch
            />
        </div>
    }
}
