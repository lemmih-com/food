//! Recipes module
//!
//! Contains recipe data structures and the Recipes page components.

use leptos::prelude::*;

// ============================================================================
// Data Types
// ============================================================================

#[derive(Clone, PartialEq)]
pub struct RecipeNutrition {
    pub calories: i32,
    pub protein: i32,
    pub carbs: i32,
    pub fat: i32,
    pub sat_fat: f32,
    pub salt: f32,
    pub fiber: i32,
}

#[derive(Clone, PartialEq)]
pub struct Recipe {
    pub name: &'static str,
    pub meal_type: &'static str,
    pub tags: &'static [&'static str],
    pub prep_time: &'static str,
    pub cook_time: &'static str,
    pub servings: i32,
    pub ingredients: &'static [&'static str],
    pub instructions: &'static [&'static str],
    pub nutrition: RecipeNutrition,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn get_meal_type_color(meal_type: &str) -> (&'static str, &'static str) {
    match meal_type {
        "Breakfast" => ("bg-yellow-100", "text-yellow-800"),
        "Lunch" => ("bg-orange-100", "text-orange-800"),
        "Dinner" => ("bg-indigo-100", "text-indigo-800"),
        "Sandwich" => ("bg-amber-100", "text-amber-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

fn get_tag_color(tag: &str) -> (&'static str, &'static str) {
    match tag {
        "High Protein" => ("bg-blue-100", "text-blue-800"),
        "Low Carb" => ("bg-purple-100", "text-purple-800"),
        "Heart Healthy" | "Omega-3" => ("bg-blue-100", "text-blue-800"),
        "Mediterranean" | "Healthy" => ("bg-green-100", "text-green-800"),
        "Plant-Based" | "Vegan" => ("bg-green-100", "text-green-800"),
        "Quick" => ("bg-yellow-100", "text-yellow-800"),
        _ => ("bg-slate-100", "text-slate-800"),
    }
}

// ============================================================================
// Data
// ============================================================================

pub const EXAMPLE_RECIPES: &[Recipe] = &[
    Recipe {
        name: "Grilled Chicken Bowl",
        meal_type: "Lunch",
        tags: &["High Protein", "Low Carb", "Healthy"],
        prep_time: "15 min",
        cook_time: "20 min",
        servings: 2,
        ingredients: &[
            "300g chicken breast",
            "200g brown rice",
            "150g broccoli",
            "1 tbsp olive oil",
            "Salt and pepper to taste",
        ],
        instructions: &[
            "Season chicken with salt and pepper",
            "Grill chicken for 6-8 minutes per side",
            "Cook rice according to package directions",
            "Steam broccoli for 5 minutes",
            "Assemble bowl and drizzle with olive oil",
        ],
        nutrition: RecipeNutrition {
            calories: 520,
            protein: 45,
            carbs: 50,
            fat: 12,
            sat_fat: 2.5,
            salt: 1.2,
            fiber: 6,
        },
    },
    Recipe {
        name: "Salmon with Vegetables",
        meal_type: "Dinner",
        tags: &["Omega-3", "Heart Healthy", "Mediterranean"],
        prep_time: "10 min",
        cook_time: "25 min",
        servings: 2,
        ingredients: &[
            "400g salmon fillet",
            "200g broccoli",
            "1 lemon",
            "2 tbsp olive oil",
            "Garlic, herbs, salt, pepper",
        ],
        instructions: &[
            "Preheat oven to 400F (200C)",
            "Place salmon on baking sheet",
            "Drizzle with olive oil and lemon juice",
            "Add broccoli around salmon",
            "Bake for 15-20 minutes",
        ],
        nutrition: RecipeNutrition {
            calories: 480,
            protein: 40,
            carbs: 10,
            fat: 30,
            sat_fat: 5.0,
            salt: 0.8,
            fiber: 4,
        },
    },
    Recipe {
        name: "Veggie Stir Fry",
        meal_type: "Dinner",
        tags: &["Vegan", "Plant-Based", "Quick"],
        prep_time: "10 min",
        cook_time: "15 min",
        servings: 2,
        ingredients: &[
            "200g broccoli",
            "150g bell peppers",
            "100g snap peas",
            "200g brown rice",
            "2 tbsp soy sauce",
            "1 tbsp sesame oil",
        ],
        instructions: &[
            "Cook rice according to package directions",
            "Heat sesame oil in wok",
            "Add vegetables and stir-fry for 5-7 minutes",
            "Add soy sauce and cook 2 more minutes",
            "Serve over rice",
        ],
        nutrition: RecipeNutrition {
            calories: 380,
            protein: 8,
            carbs: 65,
            fat: 9,
            sat_fat: 1.5,
            salt: 2.0,
            fiber: 8,
        },
    },
];

// ============================================================================
// Components
// ============================================================================

#[component]
fn RecipeCard(
    recipe: Recipe,
    active_filters: ReadSignal<Vec<String>>,
    on_tag_click: Callback<String>,
) -> impl IntoView {
    let (meal_bg, meal_text) = get_meal_type_color(recipe.meal_type);
    let meal_type = recipe.meal_type.to_string();

    view! {
        <div class="rounded-lg bg-white p-6 shadow-md">
            <div class="mb-4 flex items-center justify-between">
                <h3 class="text-2xl font-bold text-slate-900">{recipe.name}</h3>
                <button
                    class=format!("rounded px-3 py-1 text-sm font-medium cursor-pointer hover:opacity-80 {} {}", meal_bg, meal_text)
                    on:click={
                        let meal_type = meal_type.clone();
                        move |_| on_tag_click.run(meal_type.clone())
                    }
                >
                    {recipe.meal_type}
                </button>
            </div>
            <div class="mb-4">
                <p class="mb-2 text-sm text-slate-600">
                    {format!("Prep: {} | Cook: {} | Servings: {}", recipe.prep_time, recipe.cook_time, recipe.servings)}
                </p>
                <div class="flex flex-wrap gap-2">
                    {recipe.tags.iter().map(|tag| {
                        let (bg, text) = get_tag_color(tag);
                        let tag_str = tag.to_string();
                        let is_active = {
                            let tag_str = tag_str.clone();
                            move || active_filters.get().contains(&tag_str)
                        };
                        view! {
                            <button
                                class=move || format!(
                                    "rounded px-2 py-1 text-xs cursor-pointer hover:opacity-80 {} {} {}",
                                    bg,
                                    text,
                                    if is_active() { "ring-2 ring-offset-1 ring-slate-400" } else { "" }
                                )
                                on:click={
                                    let tag_str = tag_str.clone();
                                    move |_| on_tag_click.run(tag_str.clone())
                                }
                            >
                                {*tag}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            <h4 class="mb-2 font-semibold text-slate-900">"Ingredients:"</h4>
            <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
                {recipe.ingredients.iter().map(|ingredient| view! { <li>{*ingredient}</li> }).collect_view()}
            </ul>
            <h4 class="mb-2 font-semibold text-slate-900">"Instructions:"</h4>
            <ol class="list-inside list-decimal space-y-1 text-slate-700">
                {recipe.instructions.iter().map(|instruction| view! { <li>{*instruction}</li> }).collect_view()}
            </ol>
            <div class="mt-4 rounded bg-slate-50 p-3">
                <p class="text-sm font-medium text-slate-900">
                    {format!(
                        "Nutrition per serving: {} kcal | {}g protein | {}g carbs | {}g fat",
                        recipe.nutrition.calories,
                        recipe.nutrition.protein,
                        recipe.nutrition.carbs,
                        recipe.nutrition.fat
                    )}
                </p>
                <p class="text-sm text-slate-600 mt-1">
                    {format!(
                        "Sat. fat: {}g | Salt: {}g | Fiber: {}g",
                        recipe.nutrition.sat_fat,
                        recipe.nutrition.salt,
                        recipe.nutrition.fiber
                    )}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn Recipes() -> impl IntoView {
    let (active_filters, set_active_filters) = signal(Vec::<String>::new());

    let toggle_filter = Callback::new(move |tag: String| {
        set_active_filters.update(|filters| {
            if let Some(pos) = filters.iter().position(|t| t == &tag) {
                filters.remove(pos);
            } else {
                filters.push(tag);
            }
        });
    });

    let filtered_recipes = move || {
        let filters = active_filters.get();
        if filters.is_empty() {
            EXAMPLE_RECIPES.to_vec()
        } else {
            EXAMPLE_RECIPES
                .iter()
                .filter(|recipe| {
                    filters.iter().all(|filter| {
                        recipe.meal_type == filter || recipe.tags.contains(&filter.as_str())
                    })
                })
                .cloned()
                .collect()
        }
    };

    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Recipes"</h2>

            // Active filters display
            <Show when=move || !active_filters.get().is_empty()>
                <div class="mb-4 flex flex-wrap items-center gap-2">
                    <span class="text-sm font-medium text-slate-600">"Active filters:"</span>
                    <For
                        each=move || active_filters.get()
                        key=|tag| tag.clone()
                        children=move |tag: String| {
                            let tag_for_click = tag.clone();
                            let tag_for_display = tag.clone();
                            view! {
                                <button
                                    class="inline-flex items-center gap-1 rounded bg-slate-200 px-2 py-1 text-sm text-slate-700 hover:bg-slate-300 cursor-pointer"
                                    on:click=move |_| toggle_filter.run(tag_for_click.clone())
                                >
                                    {tag_for_display}
                                    <span class="font-bold">"x"</span>
                                </button>
                            }
                        }
                    />
                    <button
                        class="text-sm text-slate-500 hover:text-slate-700 underline cursor-pointer"
                        on:click=move |_| set_active_filters.set(Vec::new())
                    >
                        "Clear all"
                    </button>
                </div>
            </Show>

            <div class="grid gap-6 lg:grid-cols-2">
                <For
                    each=filtered_recipes
                    key=|recipe| recipe.name
                    children=move |recipe: Recipe| {
                        view! {
                            <RecipeCard
                                recipe=recipe
                                active_filters=active_filters
                                on_tag_click=toggle_filter
                            />
                        }
                    }
                />
            </div>

            // Show message when no recipes match
            <Show when=move || filtered_recipes().is_empty()>
                <div class="text-center py-8 text-slate-500">
                    "No recipes match the selected filters."
                </div>
            </Show>
        </div>
    }
}
