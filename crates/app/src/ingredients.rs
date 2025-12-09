//! Ingredients module
//!
//! Contains ingredient data structures, sorting logic, and the Ingredients page components.

use leptos::prelude::*;

// ============================================================================
// Data Types
// ============================================================================

/// Ingredient category for organizing into separate tables
#[derive(Clone, Copy, PartialEq, Eq)]
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
}

/// All nutrient values are per 100g
#[derive(Clone, Copy)]
pub struct Ingredient {
    pub name: &'static str,
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
    pub store: &'static str,
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
// Data
// ============================================================================

pub fn get_ingredients() -> Vec<Ingredient> {
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
          "px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider cursor-pointer hover:bg-slate-100 select-none {}",
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

#[component]
fn IngredientTable(
    title: &'static str,
    category: IngredientCategory,
    view_mode: ReadSignal<NutrientView>,
    sort_column: ReadSignal<SortColumn>,
    sort_direction: ReadSignal<SortDirection>,
    on_header_click: impl Fn(SortColumn) + Clone + 'static,
) -> impl IntoView {
    let get_sorted_ingredients = move || {
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
                    let cmp = a.name.cmp(b.name);
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
            ingredients.sort_by(|a, b| a.name.cmp(b.name));
        }

        ingredients
    };

    // Column width classes
    let w_name = "w-36"; // Ingredient name
    let w_pkg = "w-20"; // Package size
    let w_price = "w-16"; // Price
    let w_cal = "w-20"; // Calories
    let w_nutr = "w-16"; // Nutrient columns (protein, fat, etc.)
    let w_salt = "w-20"; // Salt (needs more space for mg)
    let w_store = "w-28"; // Store

    let cell_class = "px-3 py-3 whitespace-nowrap text-slate-700";

    view! {
      <div class="mb-8">
        <h3 class="mb-3 text-xl font-semibold text-slate-800">{title}</h3>
        <div class="rounded-lg bg-white shadow-md overflow-hidden overflow-x-auto">
          <table class="w-full table-fixed divide-y divide-slate-200 text-sm">
            <thead class="bg-slate-50">
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
                  col=SortColumn::PackageSize
                  label="Package"
                  width_class=w_pkg
                  sort_column=sort_column
                  sort_direction=sort_direction
                  on_click=on_header_click.clone()
                />
                <SortableHeader
                  col=SortColumn::Price
                  label="Price"
                  width_class=w_price
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
                <th class=format!(
                  "px-3 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider {}",
                  w_store,
                )>"Store"</th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-slate-200">
              <For each=get_sorted_ingredients key=|ing| ing.name let:ing>
                <tr class="hover:bg-slate-50">
                  <td class=format!("{} font-medium text-slate-900 truncate", cell_class)>{ing.name}</td>
                  <td class=cell_class>{format!("{}g", ing.package_size_g)}</td>
                  <td class=cell_class>{format!("${:.2}", ing.package_price)}</td>
                  <td class=cell_class>
                    {move || {
                      let cal = if view_mode.get() == NutrientView::Per100kcal { 100.0 } else { ing.calories };
                      format!("{:.0} kcal", cal)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.protein)
                      } else {
                        ing.protein
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.fat)
                      } else {
                        ing.fat
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.saturated_fat)
                      } else {
                        ing.saturated_fat
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.carbs)
                      } else {
                        ing.carbs
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.sugar)
                      } else {
                        ing.sugar
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.fiber)
                      } else {
                        ing.fiber
                      };
                      format!("{:.1}g", val)
                    }}
                  </td>
                  <td class=cell_class>
                    {move || {
                      let val = if view_mode.get() == NutrientView::Per100kcal {
                        ing.per_calorie(ing.salt)
                      } else {
                        ing.salt
                      };
                      format!("{:.0}mg", val)
                    }}
                  </td>
                  <td class=format!("{} truncate", cell_class)>{ing.store}</td>
                </tr>
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
          category=IngredientCategory::Protein
          view_mode=view_mode
          sort_column=sort_column
          sort_direction=sort_direction
          on_header_click=handle_header_click
        />

        <IngredientTable
          title=IngredientCategory::Carbs.title()
          category=IngredientCategory::Carbs
          view_mode=view_mode
          sort_column=sort_column
          sort_direction=sort_direction
          on_header_click=handle_header_click
        />

        <IngredientTable
          title=IngredientCategory::Veggies.title()
          category=IngredientCategory::Veggies
          view_mode=view_mode
          sort_column=sort_column
          sort_direction=sort_direction
          on_header_click=handle_header_click
        />

        <IngredientTable
          title=IngredientCategory::Other.title()
          category=IngredientCategory::Other
          view_mode=view_mode
          sort_column=sort_column
          sort_direction=sort_direction
          on_header_click=handle_header_click
        />
      </div>
    }
}
