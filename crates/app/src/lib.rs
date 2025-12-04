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
