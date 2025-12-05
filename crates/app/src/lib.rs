use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::*,
};
use leptos_config::LeptosOptions;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <nav class="bg-slate-800 text-white shadow-md">
            <div class="mx-auto max-w-7xl px-4">
                <div class="flex h-16 items-center justify-between">
                    <div class="flex items-center space-x-8">
                        <h1 class="text-xl font-bold">"food.lemmih.com"</h1>
                        <div class="flex space-x-4">
                            <A href="/" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Food Log"</A>
                            <A href="/ingredients" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Ingredients"</A>
                            <A href="/recipes" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Recipes"</A>
                            <A href="/settings" attr:class="rounded px-3 py-2 text-sm font-medium hover:bg-slate-700">"Settings"</A>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Food Log"</h2>
            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-orange-400 to-pink-400 flex items-center justify-center">
                        <span class="text-6xl">"🍕"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Margherita Pizza"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-01"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Dinner"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"750 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Delicious homemade pizza with fresh mozzarella and basil. Perfect crispy crust!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
                    </div>
                </div>
                
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-green-400 to-emerald-500 flex items-center justify-center">
                        <span class="text-6xl">"🥗"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Caesar Salad"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-02"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Lunch"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"350 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Fresh romaine with grilled chicken, parmesan, and homemade dressing. Very satisfying!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐"</span>
                    </div>
                </div>
                
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 h-48 rounded bg-gradient-to-br from-red-400 to-rose-500 flex items-center justify-center">
                        <span class="text-6xl">"🍝"</span>
                    </div>
                    <h3 class="mb-2 text-xl font-semibold">"Spaghetti Carbonara"</h3>
                    <p class="mb-2 text-sm text-slate-600">"Date: 2025-12-03"</p>
                    <div class="mb-3 flex gap-2">
                        <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Dinner"</span>
                        <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"650 kcal"</span>
                    </div>
                    <p class="text-slate-700">"Classic Italian pasta with eggs, pecorino, and guanciale. Rich and creamy!"</p>
                    <div class="mt-4 flex items-center justify-between text-sm">
                        <span class="text-slate-600">"Rating:"</span>
                        <span class="text-yellow-500">"⭐⭐⭐⭐⭐"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Ingredient category for organizing into separate tables
#[derive(Clone, Copy, PartialEq, Eq)]
enum IngredientCategory {
    Protein,
    Carbs,
    Veggies,
    Other,
}

impl IngredientCategory {
    fn title(&self) -> &'static str {
        match self {
            IngredientCategory::Protein => "Proteins",
            IngredientCategory::Carbs => "Carbs",
            IngredientCategory::Veggies => "Vegetables",
            IngredientCategory::Other => "Other",
        }
    }
}

/// All nutrient values are per 100g
#[derive(Clone)]
struct Ingredient {
    name: &'static str,
    category: IngredientCategory,
    // Nutrients per 100g
    calories: f32,      // kcal
    protein: f32,       // g
    fat: f32,           // g
    saturated_fat: f32, // g
    carbs: f32,         // g
    sugar: f32,         // g
    fiber: f32,         // g
    salt: f32,          // mg
    // Package info
    package_size_g: f32, // grams
    package_price: f32,  // price in local currency
    store: &'static str,
}

impl Ingredient {
    /// Get nutrient value per 100 kcal
    fn per_calorie(&self, value_per_100g: f32) -> f32 {
        if self.calories > 0.0 {
            (value_per_100g / self.calories) * 100.0
        } else {
            0.0
        }
    }
}

/// Which column to sort by
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SortColumn {
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
enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    fn next(self) -> Self {
        match self {
            SortDirection::None => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::None,
        }
    }

    fn indicator(&self) -> &'static str {
        match self {
            SortDirection::None => "",
            SortDirection::Ascending => " \u{2191}",
            SortDirection::Descending => " \u{2193}",
        }
    }
}

/// Whether to show nutrients per 100g or per 100kcal
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum NutrientView {
    #[default]
    Per100g,
    Per100kcal,
}

fn get_ingredients() -> Vec<Ingredient> {
    vec![
        // Proteins
        Ingredient {
            name: "Chicken Breast",
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
            store: "Whole Foods",
        },
        Ingredient {
            name: "Salmon",
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
            store: "Costco",
        },
        Ingredient {
            name: "Eggs (dozen)",
            category: IngredientCategory::Protein,
            calories: 155.0,
            protein: 13.0,
            fat: 11.0,
            saturated_fat: 3.3,
            carbs: 1.1,
            sugar: 1.1,
            fiber: 0.0,
            salt: 124.0,
            package_size_g: 720.0, // ~60g per egg x 12
            package_price: 4.99,
            store: "All stores",
        },
        Ingredient {
            name: "Ground Beef (lean)",
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
            store: "Safeway",
        },
        Ingredient {
            name: "Tofu (firm)",
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
            store: "Trader Joe's",
        },
        // Carbs
        Ingredient {
            name: "Brown Rice",
            category: IngredientCategory::Carbs,
            calories: 112.0,
            protein: 2.6,
            fat: 0.9,
            saturated_fat: 0.2,
            carbs: 24.0,
            sugar: 0.4,
            fiber: 1.8,
            salt: 1.0,
            package_size_g: 907.0, // 2lb
            package_price: 3.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Pasta (whole wheat)",
            category: IngredientCategory::Carbs,
            calories: 124.0,
            protein: 5.3,
            fat: 0.5,
            saturated_fat: 0.1,
            carbs: 25.0,
            sugar: 0.6,
            fiber: 4.5,
            salt: 4.0,
            package_size_g: 454.0, // 1lb
            package_price: 2.49,
            store: "Safeway",
        },
        Ingredient {
            name: "Oats (rolled)",
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
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Quinoa",
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
            store: "Whole Foods",
        },
        Ingredient {
            name: "Bread (whole grain)",
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
            store: "Safeway",
        },
        // Vegetables
        Ingredient {
            name: "Broccoli",
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
            store: "Safeway",
        },
        Ingredient {
            name: "Spinach (fresh)",
            category: IngredientCategory::Veggies,
            calories: 23.0,
            protein: 2.9,
            fat: 0.4,
            saturated_fat: 0.0,
            carbs: 3.6,
            sugar: 0.4,
            fiber: 2.2,
            salt: 79.0,
            package_size_g: 142.0, // 5oz bag
            package_price: 3.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Bell Peppers",
            category: IngredientCategory::Veggies,
            calories: 31.0,
            protein: 1.0,
            fat: 0.3,
            saturated_fat: 0.0,
            carbs: 6.0,
            sugar: 4.2,
            fiber: 2.1,
            salt: 4.0,
            package_size_g: 300.0, // 3-pack
            package_price: 3.99,
            store: "Whole Foods",
        },
        Ingredient {
            name: "Carrots",
            category: IngredientCategory::Veggies,
            calories: 41.0,
            protein: 0.9,
            fat: 0.2,
            saturated_fat: 0.0,
            carbs: 10.0,
            sugar: 4.7,
            fiber: 2.8,
            salt: 69.0,
            package_size_g: 454.0, // 1lb bag
            package_price: 1.99,
            store: "All stores",
        },
        Ingredient {
            name: "Tomatoes (canned)",
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
            store: "All stores",
        },
        // Other
        Ingredient {
            name: "Olive Oil",
            category: IngredientCategory::Other,
            calories: 884.0,
            protein: 0.0,
            fat: 100.0,
            saturated_fat: 14.0,
            carbs: 0.0,
            sugar: 0.0,
            fiber: 0.0,
            salt: 2.0,
            package_size_g: 500.0, // 500ml bottle
            package_price: 9.99,
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Butter",
            category: IngredientCategory::Other,
            calories: 717.0,
            protein: 0.9,
            fat: 81.0,
            saturated_fat: 51.0,
            carbs: 0.1,
            sugar: 0.1,
            fiber: 0.0,
            salt: 714.0,
            package_size_g: 227.0, // 1/2 lb
            package_price: 4.99,
            store: "All stores",
        },
        Ingredient {
            name: "Greek Yogurt",
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
            store: "Trader Joe's",
        },
        Ingredient {
            name: "Cheese (cheddar)",
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
            store: "Costco",
        },
        Ingredient {
            name: "Honey",
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
            store: "Whole Foods",
        },
    ]
}

#[component]
fn IngredientTable(
    title: &'static str,
    ingredients: Vec<Ingredient>,
    view_mode: NutrientView,
    sort_column: SortColumn,
    sort_direction: SortDirection,
    on_header_click: impl Fn(SortColumn) + Clone + 'static,
) -> impl IntoView {
    let header_class = "px-4 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider cursor-pointer hover:bg-slate-100 select-none";

    let make_header = {
        let on_header_click = on_header_click.clone();
        move |col: SortColumn, label: &'static str| {
            let on_click = on_header_click.clone();
            let indicator = if sort_column == col {
                sort_direction.indicator()
            } else {
                ""
            };
            view! {
                <th
                    class=header_class
                    on:click=move |_| on_click(col)
                >
                    {label}{indicator}
                </th>
            }
        }
    };

    let is_per_calorie = view_mode == NutrientView::Per100kcal;
    let unit_suffix = if is_per_calorie { "/100kcal" } else { "/100g" };

    view! {
        <div class="mb-8">
            <h3 class="mb-3 text-xl font-semibold text-slate-800">{title}</h3>
            <div class="rounded-lg bg-white shadow-md overflow-hidden overflow-x-auto">
                <table class="min-w-full divide-y divide-slate-200 text-sm">
                    <thead class="bg-slate-50">
                        <tr>
                            {make_header(SortColumn::Name, "Ingredient")}
                            {make_header(SortColumn::PackageSize, "Package")}
                            {make_header(SortColumn::Price, "Price")}
                            {make_header(SortColumn::Calories, "Calories")}
                            {make_header(SortColumn::Protein, "Protein")}
                            {make_header(SortColumn::Fat, "Fat")}
                            {make_header(SortColumn::SaturatedFat, "Sat. Fat")}
                            {make_header(SortColumn::Carbs, "Carbs")}
                            {make_header(SortColumn::Sugar, "Sugar")}
                            {make_header(SortColumn::Fiber, "Fiber")}
                            {make_header(SortColumn::Salt, "Salt")}
                            <th class="px-4 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Store"</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        {ingredients.into_iter().map(|ing| {
                            let (cal, prot, fat, sat_fat, carbs, sugar, fiber, salt) = if is_per_calorie {
                                (
                                    100.0, // Always 100 kcal in per-calorie view
                                    ing.per_calorie(ing.protein),
                                    ing.per_calorie(ing.fat),
                                    ing.per_calorie(ing.saturated_fat),
                                    ing.per_calorie(ing.carbs),
                                    ing.per_calorie(ing.sugar),
                                    ing.per_calorie(ing.fiber),
                                    ing.per_calorie(ing.salt),
                                )
                            } else {
                                (ing.calories, ing.protein, ing.fat, ing.saturated_fat, ing.carbs, ing.sugar, ing.fiber, ing.salt)
                            };
                            view! {
                                <tr class="hover:bg-slate-50">
                                    <td class="px-4 py-3 whitespace-nowrap font-medium text-slate-900">{ing.name}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{}g", ing.package_size_g)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("${:.2}", ing.package_price)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.0} kcal", cal)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", prot)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", fat)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", sat_fat)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", carbs)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", sugar)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.1}g", fiber)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{format!("{:.0}mg", salt)}</td>
                                    <td class="px-4 py-3 whitespace-nowrap text-slate-700">{ing.store}</td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
            <p class="mt-2 text-xs text-slate-500">{format!("* Nutrient values shown {}", unit_suffix)}</p>
        </div>
    }
}

#[component]
fn Ingredients() -> impl IntoView {
    let (view_mode, set_view_mode) = signal(NutrientView::Per100g);
    let (sort_column, set_sort_column) = signal(SortColumn::Name);
    let (sort_direction, set_sort_direction) = signal(SortDirection::None);

    let handle_header_click = move |col: SortColumn| {
        if sort_column.get() == col {
            set_sort_direction.set(sort_direction.get().next());
        } else {
            set_sort_column.set(col);
            set_sort_direction.set(SortDirection::Descending);
        }
    };

    let get_sorted_ingredients = move |category: IngredientCategory| {
        let mut ingredients: Vec<Ingredient> = get_ingredients()
            .into_iter()
            .filter(|i| i.category == category)
            .collect();

        let dir = sort_direction.get();
        let col = sort_column.get();
        let view = view_mode.get();

        if dir != SortDirection::None {
            ingredients.sort_by(|a, b| {
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
                    if view == NutrientView::Per100kcal && !matches!(col, SortColumn::PackageSize | SortColumn::Price | SortColumn::Calories) {
                        ing.per_calorie(raw)
                    } else {
                        raw
                    }
                };

                if col == SortColumn::Name {
                    let cmp = a.name.cmp(b.name);
                    return if dir == SortDirection::Ascending { cmp } else { cmp.reverse() };
                }

                let val_a = get_value(a);
                let val_b = get_value(b);
                let cmp = val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal);
                if dir == SortDirection::Ascending { cmp } else { cmp.reverse() }
            });
        } else {
            // Default: sort by name ascending
            ingredients.sort_by(|a, b| a.name.cmp(b.name));
        }

        ingredients
    };

    view! {
        <div class="mx-auto max-w-7xl py-6">
            <div class="mb-6 flex items-center justify-between">
                <h2 class="text-3xl font-bold text-slate-900">"Ingredient List"</h2>
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

            <IngredientTable
                title=IngredientCategory::Protein.title()
                ingredients=get_sorted_ingredients(IngredientCategory::Protein)
                view_mode=view_mode.get()
                sort_column=sort_column.get()
                sort_direction=sort_direction.get()
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Carbs.title()
                ingredients=get_sorted_ingredients(IngredientCategory::Carbs)
                view_mode=view_mode.get()
                sort_column=sort_column.get()
                sort_direction=sort_direction.get()
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Veggies.title()
                ingredients=get_sorted_ingredients(IngredientCategory::Veggies)
                view_mode=view_mode.get()
                sort_column=sort_column.get()
                sort_direction=sort_direction.get()
                on_header_click=handle_header_click.clone()
            />

            <IngredientTable
                title=IngredientCategory::Other.title()
                ingredients=get_sorted_ingredients(IngredientCategory::Other)
                view_mode=view_mode.get()
                sort_column=sort_column.get()
                sort_direction=sort_direction.get()
                on_header_click=handle_header_click
            />
        </div>
    }
}

#[component]
fn Recipes() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Recipes"</h2>
            <div class="grid gap-6 lg:grid-cols-2">
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 flex items-center justify-between">
                        <h3 class="text-2xl font-bold text-slate-900">"Grilled Chicken Bowl"</h3>
                        <span class="rounded bg-green-100 px-3 py-1 text-sm font-medium text-green-800">"Healthy"</span>
                    </div>
                    <div class="mb-4">
                        <p class="mb-2 text-sm text-slate-600">"Prep: 15 min | Cook: 20 min | Servings: 2"</p>
                        <div class="flex gap-2">
                            <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"High Protein"</span>
                            <span class="rounded bg-purple-100 px-2 py-1 text-xs text-purple-800">"Low Carb"</span>
                        </div>
                    </div>
                    <h4 class="mb-2 font-semibold text-slate-900">"Ingredients:"</h4>
                    <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
                        <li>"300g chicken breast"</li>
                        <li>"200g brown rice"</li>
                        <li>"150g broccoli"</li>
                        <li>"1 tbsp olive oil"</li>
                        <li>"Salt and pepper to taste"</li>
                    </ul>
                    <h4 class="mb-2 font-semibold text-slate-900">"Instructions:"</h4>
                    <ol class="list-inside list-decimal space-y-1 text-slate-700">
                        <li>"Season chicken with salt and pepper"</li>
                        <li>"Grill chicken for 6-8 minutes per side"</li>
                        <li>"Cook rice according to package directions"</li>
                        <li>"Steam broccoli for 5 minutes"</li>
                        <li>"Assemble bowl and drizzle with olive oil"</li>
                    </ol>
                    <div class="mt-4 rounded bg-slate-50 p-3">
                        <p class="text-sm font-medium text-slate-900">"Nutrition per serving: 520 kcal | 45g protein | 50g carbs | 12g fat"</p>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 flex items-center justify-between">
                        <h3 class="text-2xl font-bold text-slate-900">"Salmon with Vegetables"</h3>
                        <span class="rounded bg-blue-100 px-3 py-1 text-sm font-medium text-blue-800">"Omega-3"</span>
                    </div>
                    <div class="mb-4">
                        <p class="mb-2 text-sm text-slate-600">"Prep: 10 min | Cook: 25 min | Servings: 2"</p>
                        <div class="flex gap-2">
                            <span class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-800">"Heart Healthy"</span>
                            <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"Mediterranean"</span>
                        </div>
                    </div>
                    <h4 class="mb-2 font-semibold text-slate-900">"Ingredients:"</h4>
                    <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
                        <li>"400g salmon fillet"</li>
                        <li>"200g broccoli"</li>
                        <li>"1 lemon"</li>
                        <li>"2 tbsp olive oil"</li>
                        <li>"Garlic, herbs, salt, pepper"</li>
                    </ul>
                    <h4 class="mb-2 font-semibold text-slate-900">"Instructions:"</h4>
                    <ol class="list-inside list-decimal space-y-1 text-slate-700">
                        <li>"Preheat oven to 400°F (200°C)"</li>
                        <li>"Place salmon on baking sheet"</li>
                        <li>"Drizzle with olive oil and lemon juice"</li>
                        <li>"Add broccoli around salmon"</li>
                        <li>"Bake for 15-20 minutes"</li>
                    </ol>
                    <div class="mt-4 rounded bg-slate-50 p-3">
                        <p class="text-sm font-medium text-slate-900">"Nutrition per serving: 480 kcal | 40g protein | 10g carbs | 30g fat"</p>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <div class="mb-4 flex items-center justify-between">
                        <h3 class="text-2xl font-bold text-slate-900">"Veggie Stir Fry"</h3>
                        <span class="rounded bg-green-100 px-3 py-1 text-sm font-medium text-green-800">"Vegan"</span>
                    </div>
                    <div class="mb-4">
                        <p class="mb-2 text-sm text-slate-600">"Prep: 10 min | Cook: 15 min | Servings: 2"</p>
                        <div class="flex gap-2">
                            <span class="rounded bg-green-100 px-2 py-1 text-xs text-green-800">"Plant-Based"</span>
                            <span class="rounded bg-yellow-100 px-2 py-1 text-xs text-yellow-800">"Quick"</span>
                        </div>
                    </div>
                    <h4 class="mb-2 font-semibold text-slate-900">"Ingredients:"</h4>
                    <ul class="mb-4 list-inside list-disc space-y-1 text-slate-700">
                        <li>"200g broccoli"</li>
                        <li>"150g bell peppers"</li>
                        <li>"100g snap peas"</li>
                        <li>"200g brown rice"</li>
                        <li>"2 tbsp soy sauce"</li>
                        <li>"1 tbsp sesame oil"</li>
                    </ul>
                    <h4 class="mb-2 font-semibold text-slate-900">"Instructions:"</h4>
                    <ol class="list-inside list-decimal space-y-1 text-slate-700">
                        <li>"Cook rice according to package directions"</li>
                        <li>"Heat sesame oil in wok"</li>
                        <li>"Add vegetables and stir-fry for 5-7 minutes"</li>
                        <li>"Add soy sauce and cook 2 more minutes"</li>
                        <li>"Serve over rice"</li>
                    </ol>
                    <div class="mt-4 rounded bg-slate-50 p-3">
                        <p class="text-sm font-medium text-slate-900">"Nutrition per serving: 380 kcal | 8g protein | 65g carbs | 9g fat"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-4xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Settings"</h2>
            
            <div class="space-y-6">
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Goals"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Target Calories per Meal"</label>
                            <input 
                                type="number" 
                                value="600" 
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Your target calorie intake per meal"</p>
                        </div>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Macro Distribution"</h3>
                    <div class="space-y-4">
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <label class="text-sm font-medium text-slate-700">"Protein"</label>
                                <span class="text-sm font-semibold text-blue-600">"30%"</span>
                            </div>
                            <input 
                                type="range" 
                                min="10" 
                                max="50" 
                                value="30" 
                                class="w-full"
                            />
                        </div>
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <label class="text-sm font-medium text-slate-700">"Carbohydrates"</label>
                                <span class="text-sm font-semibold text-green-600">"40%"</span>
                            </div>
                            <input 
                                type="range" 
                                min="10" 
                                max="60" 
                                value="40" 
                                class="w-full"
                            />
                        </div>
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <label class="text-sm font-medium text-slate-700">"Fat"</label>
                                <span class="text-sm font-semibold text-orange-600">"30%"</span>
                            </div>
                            <input 
                                type="range" 
                                min="10" 
                                max="50" 
                                value="30" 
                                class="w-full"
                            />
                        </div>
                    </div>
                </div>

                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Limits"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Salt Limit (mg)"</label>
                            <input 
                                type="number" 
                                value="2300" 
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Recommended: 2300mg (about 1 teaspoon)"</p>
                        </div>
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Saturated Fat Limit (g)"</label>
                            <input 
                                type="number" 
                                value="20" 
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Recommended: Less than 10% of daily calories"</p>
                        </div>
                    </div>
                </div>

                <div class="flex justify-end">
                    <button class="rounded bg-blue-600 px-6 py-2 font-semibold text-white hover:bg-blue-700">
                        "Save Settings"
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Navigation />
            <main class="min-h-screen bg-slate-100 px-4">
                <Routes fallback=|| "Not found">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/ingredients") view=Ingredients />
                    <Route path=path!("/recipes") view=Recipes />
                    <Route path=path!("/settings") view=Settings />
                </Routes>
            </main>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/styles.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <title>"food.lemmih.com"</title>
            </head>
            <body class="bg-slate-100 text-slate-900">
                <App/>
            </body>
        </html>
    }
}
