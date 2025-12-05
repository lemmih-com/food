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

#[component]
fn Ingredients() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Ingredient List"</h2>
            <div class="rounded-lg bg-white shadow-md overflow-hidden">
                <table class="min-w-full divide-y divide-slate-200">
                    <thead class="bg-slate-50">
                        <tr>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Ingredient"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Calories (per 100g)"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Protein"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Fat"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Carbs"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Price"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">"Store"</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Chicken Breast"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"165 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"31g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"3.6g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$6.99/lb"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"Whole Foods, Trader Joe's"</td>
                        </tr>
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Broccoli"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"34 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"2.8g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0.4g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"7g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$2.49/lb"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"Safeway, Whole Foods"</td>
                        </tr>
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Brown Rice"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"112 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"2.6g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0.9g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"24g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$3.99/2lb"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"Trader Joe's, Amazon"</td>
                        </tr>
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Salmon"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"208 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"20g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"13g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$12.99/lb"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"Whole Foods, Costco"</td>
                        </tr>
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Olive Oil"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"884 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"100g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"0g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$9.99/bottle"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"Trader Joe's, Safeway"</td>
                        </tr>
                        <tr class="hover:bg-slate-50">
                            <td class="px-6 py-4 whitespace-nowrap font-medium text-slate-900">"Eggs"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"155 kcal"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"13g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"11g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"1.1g"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"$4.99/dozen"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-slate-700">"All stores"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
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

// Conversion factor: 1g salt = 393.4mg sodium (sodium is ~39.34% of salt by weight)
// So 2300mg sodium ≈ 5.84g salt, and 5g salt ≈ 1967mg sodium

// Calories per gram for macros
const CALORIES_PER_GRAM_PROTEIN: f64 = 4.0;
const CALORIES_PER_GRAM_CARBS: f64 = 4.0;
const CALORIES_PER_GRAM_FAT: f64 = 9.0;

// Macro types for locking
#[derive(Clone, Copy, PartialEq, Eq)]
enum Macro {
    Protein,
    Carbs,
    Fat,
}

#[component]
fn Settings() -> impl IntoView {
    // Daily calorie goal
    let (daily_calories, set_daily_calories) = signal(2000_i32);

    // Macro distribution (must sum to 100)
    let (protein_pct, set_protein_pct) = signal(30_i32);
    let (carbs_pct, set_carbs_pct) = signal(40_i32);
    let (fat_pct, set_fat_pct) = signal(30_i32);

    // Which macro is locked (only one can be locked at a time)
    let (locked_macro, set_locked_macro) = signal(Option::<Macro>::None);

    // Function to adjust macros when one changes, keeping total at 100%
    // The changed macro and any locked macro are preserved; the third adjusts
    let adjust_macros = move |changed: Macro, new_value: i32| {
        let new_value = new_value.clamp(5, 90); // Ensure reasonable bounds
        let locked = locked_macro.get();

        // Unlock the macro being changed
        if locked == Some(changed) {
            set_locked_macro.set(None);
        }
        let locked = if locked == Some(changed) {
            None
        } else {
            locked
        };

        let (current_protein, current_carbs, current_fat) =
            (protein_pct.get(), carbs_pct.get(), fat_pct.get());

        match changed {
            Macro::Protein => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Carbs) => {
                        // Carbs locked, adjust fat
                        let new_fat = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_fat;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        // Fat locked, adjust carbs
                        let new_carbs = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_carbs;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
                        // No lock or protein was locked (now unlocked), distribute to carbs and fat proportionally
                        let old_other_total = current_carbs + current_fat;
                        if old_other_total > 0 {
                            let carbs_ratio = current_carbs as f64 / old_other_total as f64;
                            let new_carbs = (remaining as f64 * carbs_ratio).round() as i32;
                            let new_fat = remaining - new_carbs;
                            set_protein_pct.set(new_value);
                            set_carbs_pct.set(new_carbs.clamp(5, 90));
                            set_fat_pct.set(new_fat.clamp(5, 90));
                        } else {
                            set_protein_pct.set(new_value);
                            set_carbs_pct.set(remaining / 2);
                            set_fat_pct.set(remaining - remaining / 2);
                        }
                    }
                }
            }
            Macro::Carbs => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Protein) => {
                        // Protein locked, adjust fat
                        let new_fat = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_fat;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        // Fat locked, adjust protein
                        let new_protein = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
                        // No lock, distribute to protein and fat proportionally
                        let old_other_total = current_protein + current_fat;
                        if old_other_total > 0 {
                            let protein_ratio = current_protein as f64 / old_other_total as f64;
                            let new_protein = (remaining as f64 * protein_ratio).round() as i32;
                            let new_fat = remaining - new_protein;
                            set_protein_pct.set(new_protein.clamp(5, 90));
                            set_carbs_pct.set(new_value);
                            set_fat_pct.set(new_fat.clamp(5, 90));
                        } else {
                            set_protein_pct.set(remaining / 2);
                            set_carbs_pct.set(new_value);
                            set_fat_pct.set(remaining - remaining / 2);
                        }
                    }
                }
            }
            Macro::Fat => {
                let remaining = 100 - new_value;
                match locked {
                    Some(Macro::Protein) => {
                        // Protein locked, adjust carbs
                        let new_carbs = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_carbs;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    Some(Macro::Carbs) => {
                        // Carbs locked, adjust protein
                        let new_protein = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    _ => {
                        // No lock, distribute to protein and carbs proportionally
                        let old_other_total = current_protein + current_carbs;
                        if old_other_total > 0 {
                            let protein_ratio = current_protein as f64 / old_other_total as f64;
                            let new_protein = (remaining as f64 * protein_ratio).round() as i32;
                            let new_carbs = remaining - new_protein;
                            set_protein_pct.set(new_protein.clamp(5, 90));
                            set_carbs_pct.set(new_carbs.clamp(5, 90));
                            set_fat_pct.set(new_value);
                        } else {
                            set_protein_pct.set(remaining / 2);
                            set_carbs_pct.set(remaining - remaining / 2);
                            set_fat_pct.set(new_value);
                        }
                    }
                }
            }
        }
    };

    // Computed grams for each macro
    let protein_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = protein_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_PROTEIN).round() as i32
    });
    let carbs_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = carbs_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_CARBS).round() as i32
    });
    let fat_grams = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        let pct = fat_pct.get() as f64 / 100.0;
        (cals * pct / CALORIES_PER_GRAM_FAT).round() as i32
    });

    // Salt/Sodium: use_sodium_unit = true means display as sodium (mg), false means salt (g)
    let (use_sodium_unit, set_use_sodium_unit) = signal(true);
    // Store internally as sodium in mg
    let (sodium_mg, set_sodium_mg) = signal(2300_i32);

    // Computed salt in grams (for display when using salt unit)
    let salt_grams = Memo::new(move |_| {
        // 2300mg sodium ≈ 5.84g salt; we use: salt_g = sodium_mg / 393.4
        (sodium_mg.get() as f64 / 393.4 * 10.0).round() / 10.0
    });

    // Saturated fat: store as grams, compute percentage
    let (sat_fat_grams, set_sat_fat_grams) = signal(20_i32);
    let sat_fat_pct = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        if cals <= 0.0 {
            return 0.0;
        }
        let fat_cals = sat_fat_grams.get() as f64 * CALORIES_PER_GRAM_FAT;
        (fat_cals / cals * 100.0 * 10.0).round() / 10.0
    });

    // Fiber minimum
    let (fiber_min, set_fiber_min) = signal(25_i32);

    // Preset loader
    let load_preset = move |preset: &str| {
        match preset {
            "usda" => {
                // dietaryguidelines.gov (USDA) - 2000 cal, 10-35% protein, 45-65% carbs, 20-35% fat
                // Using middle-ground values; sodium 2300mg, sat fat <10%, fiber 28g
                set_daily_calories.set(2000);
                set_protein_pct.set(20);
                set_carbs_pct.set(55);
                set_fat_pct.set(25);
                set_sodium_mg.set(2300);
                set_sat_fat_grams.set(22); // ~10% of 2000 cal
                set_fiber_min.set(28);
            }
            "aha" => {
                // AHA (American Heart Association) - focuses on heart health
                // 2000 cal, lower sat fat (<6%), sodium <2300mg (ideally 1500mg), fiber 25-30g
                set_daily_calories.set(2000);
                set_protein_pct.set(25);
                set_carbs_pct.set(50);
                set_fat_pct.set(25);
                set_sodium_mg.set(1500);
                set_sat_fat_grams.set(13); // ~6% of 2000 cal
                set_fiber_min.set(30);
            }
            "nhs" => {
                // NHS (UK) - 2000 cal for women, 2500 for men; using 2000
                // Sat fat <11%, salt <6g (~2360mg sodium), fiber 30g
                set_daily_calories.set(2000);
                set_protein_pct.set(20);
                set_carbs_pct.set(50);
                set_fat_pct.set(30);
                set_sodium_mg.set(2360);
                set_sat_fat_grams.set(24); // ~11% of 2000 cal
                set_fiber_min.set(30);
            }
            _ => {}
        }
    };

    view! {
        <div class="mx-auto max-w-4xl py-6">
            <h2 class="mb-6 text-3xl font-bold text-slate-900">"Settings"</h2>

            // Preset buttons
            <div class="mb-6 rounded-lg bg-white p-6 shadow-md">
                <h3 class="mb-4 text-xl font-semibold text-slate-900">"Load Preset"</h3>
                <div class="flex flex-wrap gap-3">
                    <button
                        class="rounded bg-emerald-600 px-4 py-2 text-sm font-semibold text-white hover:bg-emerald-700"
                        on:click=move |_| load_preset("usda")
                    >
                        "USDA Dietary Guidelines"
                    </button>
                    <button
                        class="rounded bg-red-600 px-4 py-2 text-sm font-semibold text-white hover:bg-red-700"
                        on:click=move |_| load_preset("aha")
                    >
                        "AHA Guidelines"
                    </button>
                    <button
                        class="rounded bg-blue-600 px-4 py-2 text-sm font-semibold text-white hover:bg-blue-700"
                        on:click=move |_| load_preset("nhs")
                    >
                        "NHS Guidelines"
                    </button>
                </div>
            </div>

            <div class="space-y-6">
                // Daily Goals
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Goals"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Target Calories per Day"</label>
                            <input
                                type="number"
                                prop:value=move || daily_calories.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        set_daily_calories.set(val.max(0));
                                    }
                                }
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Your target calorie intake per day"</p>
                        </div>
                    </div>
                </div>

                // Macro Distribution
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Macro Distribution"</h3>

                    // Info message
                    <div class="mb-4 rounded bg-blue-50 px-3 py-2 text-sm text-blue-800">
                        "Total: 100% - Lock a macro to prevent it from auto-adjusting"
                    </div>

                    <div class="space-y-4">
                        // Protein
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <button
                                        class="flex h-6 w-6 items-center justify-center rounded text-sm hover:bg-slate-100"
                                        title=move || if locked_macro.get() == Some(Macro::Protein) { "Click to unlock" } else { "Click to lock" }
                                        on:click=move |_| {
                                            if locked_macro.get() == Some(Macro::Protein) {
                                                set_locked_macro.set(None);
                                            } else {
                                                set_locked_macro.set(Some(Macro::Protein));
                                            }
                                        }
                                    >
                                        {move || if locked_macro.get() == Some(Macro::Protein) { "🔒" } else { "🔓" }}
                                    </button>
                                    <label class="text-sm font-medium text-slate-700">"Protein"</label>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm font-semibold text-blue-600">{move || format!("{}%", protein_pct.get())}</span>
                                    <span class="text-xs text-slate-500">{move || format!("({}g)", protein_grams.get())}</span>
                                </div>
                            </div>
                            <input
                                type="range"
                                min="5"
                                max="90"
                                prop:value=move || protein_pct.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        adjust_macros(Macro::Protein, val);
                                    }
                                }
                                class="w-full"
                            />
                        </div>

                        // Carbohydrates
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <button
                                        class="flex h-6 w-6 items-center justify-center rounded text-sm hover:bg-slate-100"
                                        title=move || if locked_macro.get() == Some(Macro::Carbs) { "Click to unlock" } else { "Click to lock" }
                                        on:click=move |_| {
                                            if locked_macro.get() == Some(Macro::Carbs) {
                                                set_locked_macro.set(None);
                                            } else {
                                                set_locked_macro.set(Some(Macro::Carbs));
                                            }
                                        }
                                    >
                                        {move || if locked_macro.get() == Some(Macro::Carbs) { "🔒" } else { "🔓" }}
                                    </button>
                                    <label class="text-sm font-medium text-slate-700">"Carbohydrates"</label>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm font-semibold text-green-600">{move || format!("{}%", carbs_pct.get())}</span>
                                    <span class="text-xs text-slate-500">{move || format!("({}g)", carbs_grams.get())}</span>
                                </div>
                            </div>
                            <input
                                type="range"
                                min="5"
                                max="90"
                                prop:value=move || carbs_pct.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        adjust_macros(Macro::Carbs, val);
                                    }
                                }
                                class="w-full"
                            />
                        </div>

                        // Fat
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <button
                                        class="flex h-6 w-6 items-center justify-center rounded text-sm hover:bg-slate-100"
                                        title=move || if locked_macro.get() == Some(Macro::Fat) { "Click to unlock" } else { "Click to lock" }
                                        on:click=move |_| {
                                            if locked_macro.get() == Some(Macro::Fat) {
                                                set_locked_macro.set(None);
                                            } else {
                                                set_locked_macro.set(Some(Macro::Fat));
                                            }
                                        }
                                    >
                                        {move || if locked_macro.get() == Some(Macro::Fat) { "🔒" } else { "🔓" }}
                                    </button>
                                    <label class="text-sm font-medium text-slate-700">"Fat"</label>
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-sm font-semibold text-orange-600">{move || format!("{}%", fat_pct.get())}</span>
                                    <span class="text-xs text-slate-500">{move || format!("({}g)", fat_grams.get())}</span>
                                </div>
                            </div>
                            <input
                                type="range"
                                min="5"
                                max="90"
                                prop:value=move || fat_pct.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        adjust_macros(Macro::Fat, val);
                                    }
                                }
                                class="w-full"
                            />
                        </div>
                    </div>
                </div>

                // Daily Limits
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Limits"</h3>
                    <div class="space-y-4">
                        // Salt/Sodium with toggle
                        <div>
                            <div class="mb-2 flex items-center justify-between">
                                <label class="block text-sm font-medium text-slate-700">
                                    {move || if use_sodium_unit.get() { "Daily Sodium Limit (mg)" } else { "Daily Salt Limit (g)" }}
                                </label>
                                <button
                                    class="rounded bg-slate-200 px-3 py-1 text-xs font-medium text-slate-700 hover:bg-slate-300"
                                    on:click=move |_| set_use_sodium_unit.set(!use_sodium_unit.get())
                                >
                                    {move || if use_sodium_unit.get() { "Switch to Salt (g)" } else { "Switch to Sodium (mg)" }}
                                </button>
                            </div>
                            {move || {
                                if use_sodium_unit.get() {
                                    view! {
                                        <input
                                            type="number"
                                            prop:value=move || sodium_mg.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                    set_sodium_mg.set(val.max(0));
                                                }
                                            }
                                            class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        />
                                    }.into_any()
                                } else {
                                    view! {
                                        <input
                                            type="number"
                                            step="0.1"
                                            prop:value=move || salt_grams.get()
                                            on:input=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    // Convert salt grams back to sodium mg
                                                    let sodium = (val * 393.4).round() as i32;
                                                    set_sodium_mg.set(sodium.max(0));
                                                }
                                            }
                                            class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                        />
                                    }.into_any()
                                }
                            }}
                            <p class="mt-1 text-xs text-slate-500">
                                {move || if use_sodium_unit.get() {
                                    format!("Equivalent to {:.1}g salt. Recommended: 2300mg (US) / 2360mg (UK)", salt_grams.get())
                                } else {
                                    format!("Equivalent to {}mg sodium. Recommended: ~5-6g salt per day", sodium_mg.get())
                                }}
                            </p>
                        </div>

                        // Saturated Fat with dual inputs (grams and percentage)
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Saturated Fat Limit"</label>
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"Grams"</label>
                                    <input
                                        type="number"
                                        prop:value=move || sat_fat_grams.get()
                                        on:input=move |ev| {
                                            if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                set_sat_fat_grams.set(val.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                                <div>
                                    <label class="mb-1 block text-xs text-slate-500">"% of Daily Calories"</label>
                                    <input
                                        type="number"
                                        step="0.1"
                                        prop:value=move || sat_fat_pct.get()
                                        on:input=move |ev| {
                                            if let Ok(pct) = event_target_value(&ev).parse::<f64>() {
                                                // Convert percentage to grams
                                                let cals = daily_calories.get() as f64;
                                                let grams = (pct / 100.0 * cals / CALORIES_PER_GRAM_FAT).round() as i32;
                                                set_sat_fat_grams.set(grams.max(0));
                                            }
                                        }
                                        class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                            </div>
                            <p class="mt-1 text-xs text-slate-500">"Recommended: Less than 10% of daily calories (AHA recommends <6% for heart health)"</p>
                        </div>
                    </div>
                </div>

                // Daily Minimums
                <div class="rounded-lg bg-white p-6 shadow-md">
                    <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Minimums"</h3>
                    <div class="space-y-4">
                        <div>
                            <label class="mb-2 block text-sm font-medium text-slate-700">"Minimum Fiber Intake (g)"</label>
                            <input
                                type="number"
                                prop:value=move || fiber_min.get()
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                        set_fiber_min.set(val.max(0));
                                    }
                                }
                                class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                            />
                            <p class="mt-1 text-xs text-slate-500">"Recommended: 25-30g per day for adults"</p>
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
