//! Settings module
//!
//! Contains the Settings page with sub-components for different sections:
//! - PresetButtons: Load preset dietary guidelines
//! - DailyGoals: Target calories input
//! - MacroDistribution: Protein/Carbs/Fat sliders with pie chart
//! - DailyLimits: Salt/sodium and saturated fat limits
//! - DailyMinimums: Fiber minimum

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants
// ============================================================================

// Calories per gram for macros
const CALORIES_PER_GRAM_PROTEIN: f64 = 4.0;
const CALORIES_PER_GRAM_CARBS: f64 = 4.0;
const CALORIES_PER_GRAM_FAT: f64 = 9.0;

// Default settings
const DEFAULT_DAILY_CALORIES: i32 = 2000;
const DEFAULT_PROTEIN_PCT: i32 = 22;
const DEFAULT_CARBS_PCT: i32 = 43;
const DEFAULT_FAT_PCT: i32 = 35;
const DEFAULT_SODIUM_MG: i32 = 2300;
const DEFAULT_SAT_FAT_PCT: f64 = 10.0;
const DEFAULT_FIBER_MIN: i32 = 25;

#[cfg(not(feature = "ssr"))]
const SETTINGS_STORAGE_KEY: &str = "user_settings";

#[derive(Clone, Serialize, Deserialize)]
struct SettingsData {
    daily_calories: i32,
    protein_pct: i32,
    carbs_pct: i32,
    fat_pct: i32,
    sodium_mg: i32,
    sat_fat_grams: i32,
    fiber_min: i32,
}

fn sat_fat_grams_from_pct(calories: i32, pct: f64) -> i32 {
    ((calories as f64 * pct / 100.0) / CALORIES_PER_GRAM_FAT).round() as i32
}

fn default_settings() -> SettingsData {
    SettingsData {
        daily_calories: DEFAULT_DAILY_CALORIES,
        protein_pct: DEFAULT_PROTEIN_PCT,
        carbs_pct: DEFAULT_CARBS_PCT,
        fat_pct: DEFAULT_FAT_PCT,
        sodium_mg: DEFAULT_SODIUM_MG,
        sat_fat_grams: sat_fat_grams_from_pct(DEFAULT_DAILY_CALORIES, DEFAULT_SAT_FAT_PCT),
        fiber_min: DEFAULT_FIBER_MIN,
    }
}

#[cfg(not(feature = "ssr"))]
fn load_settings() -> Option<SettingsData> {
    use gloo_storage::{LocalStorage, Storage};

    LocalStorage::get(SETTINGS_STORAGE_KEY).ok()
}

#[cfg(feature = "ssr")]
fn load_settings() -> Option<SettingsData> {
    None
}

#[cfg(not(feature = "ssr"))]
fn save_settings(settings: &SettingsData) {
    use gloo_storage::{LocalStorage, Storage};

    let _ = LocalStorage::set(SETTINGS_STORAGE_KEY, settings);
}

#[cfg(feature = "ssr")]
fn save_settings(_settings: &SettingsData) {}

// ============================================================================
// Types
// ============================================================================

/// Macro types for locking
#[derive(Clone, Copy, PartialEq, Eq)]
enum Macro {
    Protein,
    Carbs,
    Fat,
}

// ============================================================================
// Sub-Components
// ============================================================================

/// Preset buttons for loading common dietary guidelines
#[component]
fn PresetButtons(
    set_daily_calories: WriteSignal<i32>,
    set_protein_pct: WriteSignal<i32>,
    set_carbs_pct: WriteSignal<i32>,
    set_fat_pct: WriteSignal<i32>,
    set_sodium_mg: WriteSignal<i32>,
    set_sat_fat_grams: WriteSignal<i32>,
    set_fiber_min: WriteSignal<i32>,
) -> impl IntoView {
    let load_preset = move |preset: &str| {
        match preset {
            "default" => {
                set_daily_calories.set(DEFAULT_DAILY_CALORIES);
                set_protein_pct.set(DEFAULT_PROTEIN_PCT);
                set_carbs_pct.set(DEFAULT_CARBS_PCT);
                set_fat_pct.set(DEFAULT_FAT_PCT);
                set_sodium_mg.set(DEFAULT_SODIUM_MG);
                set_sat_fat_grams.set(sat_fat_grams_from_pct(
                    DEFAULT_DAILY_CALORIES,
                    DEFAULT_SAT_FAT_PCT,
                ));
                set_fiber_min.set(DEFAULT_FIBER_MIN);
            }
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
      <div class="mb-6 rounded-lg bg-white p-6 shadow-md">
        <h3 class="mb-4 text-xl font-semibold text-slate-900">"Load Preset"</h3>
        <div class="flex flex-wrap gap-3">
          <button
            class="rounded bg-slate-800 px-4 py-2 text-sm font-semibold text-white hover:bg-slate-900"
            on:click=move |_| load_preset("default")
          >
            "Default (22/43/35 · 10% sat fat)"
          </button>
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
    }
}

/// Daily calorie goal input
#[component]
fn DailyGoals(
    daily_calories: ReadSignal<i32>,
    set_daily_calories: WriteSignal<i32>,
) -> impl IntoView {
    view! {
      <div class="rounded-lg bg-white p-6 shadow-md">
        <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Goals"</h3>
        <div class="space-y-4">
          <div>
            <label class="mb-2 block text-sm font-medium text-slate-700">"Target Calories per Day"</label>
            <input
              type="number"
              attr:data-test="settings-calories"
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
    }
}

/// Pie chart visualization for macro distribution
#[component]
fn MacroPieChart(
    protein_pct: ReadSignal<i32>,
    carbs_pct: ReadSignal<i32>,
    fat_pct: ReadSignal<i32>,
) -> impl IntoView {
    let pie_chart_paths = Memo::new(move |_| {
        let protein = protein_pct.get() as f64;
        let carbs = carbs_pct.get() as f64;
        let fat = fat_pct.get() as f64;

        // Center and radius for the pie chart
        let cx = 60.0_f64;
        let cy = 60.0_f64;
        let r = 50.0_f64;

        // Calculate angles (in radians, starting from top)
        let protein_angle = protein / 100.0 * 2.0 * std::f64::consts::PI;
        let carbs_angle = carbs / 100.0 * 2.0 * std::f64::consts::PI;
        let fat_angle = fat / 100.0 * 2.0 * std::f64::consts::PI;

        // Starting angle (top of circle = -PI/2)
        let start = -std::f64::consts::PI / 2.0;

        // Helper to create arc path
        let arc_path = |start_angle: f64, end_angle: f64| -> String {
            let x1 = cx + r * start_angle.cos();
            let y1 = cy + r * start_angle.sin();
            let x2 = cx + r * end_angle.cos();
            let y2 = cy + r * end_angle.sin();
            let large_arc = if (end_angle - start_angle).abs() > std::f64::consts::PI {
                1
            } else {
                0
            };
            format!("M {cx} {cy} L {x1} {y1} A {r} {r} 0 {large_arc} 1 {x2} {y2} Z")
        };

        let protein_end = start + protein_angle;
        let carbs_end = protein_end + carbs_angle;
        let fat_end = carbs_end + fat_angle;

        (
            arc_path(start, protein_end),
            arc_path(protein_end, carbs_end),
            arc_path(carbs_end, fat_end),
        )
    });

    view! {
      <div class="flex-shrink-0 flex flex-col items-center">
        <svg width="120" height="120" viewBox="0 0 120 120">
          // Protein slice (blue)
          <path d=move || pie_chart_paths.get().0 fill="#2563eb" />
          // Carbs slice (green)
          <path d=move || pie_chart_paths.get().1 fill="#16a34a" />
          // Fat slice (orange)
          <path d=move || pie_chart_paths.get().2 fill="#ea580c" />
        </svg>
        // Legend
        <div class="mt-2 flex gap-3 text-xs">
          <div class="flex items-center gap-1">
            <div class="h-3 w-3 rounded-sm bg-blue-600"></div>
            <span>"Protein"</span>
          </div>
          <div class="flex items-center gap-1">
            <div class="h-3 w-3 rounded-sm bg-green-600"></div>
            <span>"Carbs"</span>
          </div>
          <div class="flex items-center gap-1">
            <div class="h-3 w-3 rounded-sm bg-orange-600"></div>
            <span>"Fat"</span>
          </div>
        </div>
      </div>
    }
}

/// Single macro input row (protein, carbs, or fat)
#[component]
fn MacroInputRow(
    macro_type: Macro,
    label: &'static str,
    color_class: &'static str,
    border_color_class: &'static str,
    bg_color_class: &'static str,
    pct_data_test: &'static str,
    grams_data_test: &'static str,
    pct: ReadSignal<i32>,
    grams: Memo<i32>,
    locked_macro: ReadSignal<Option<Macro>>,
    set_locked_macro: WriteSignal<Option<Macro>>,
    adjust_macros: impl Fn(Macro, i32) + Clone + 'static,
    adjust_macros_from_grams: impl Fn(Macro, i32) + Clone + 'static,
) -> impl IntoView {
    let adjust_macros_pct = adjust_macros.clone();
    let adjust_macros_grams = adjust_macros_from_grams;

    view! {
      <div class="flex items-center gap-2">
        <button
          class="flex h-7 w-7 items-center justify-center rounded border text-sm hover:bg-slate-100"
          class=(border_color_class, move || locked_macro.get() == Some(macro_type))
          class=(bg_color_class, move || locked_macro.get() == Some(macro_type))
          class=("border-slate-300", move || locked_macro.get() != Some(macro_type))
          title=move || { if locked_macro.get() == Some(macro_type) { "Click to unlock" } else { "Click to lock" } }
          on:click=move |_| {
            if locked_macro.get() == Some(macro_type) {
              set_locked_macro.set(None);
            } else {
              set_locked_macro.set(Some(macro_type));
            }
          }
        >
          {move || if locked_macro.get() == Some(macro_type) { "🔒" } else { "🔓" }}
        </button>
        <div class=format!("h-3 w-3 rounded-sm {}", color_class)></div>
        <span class="w-24 text-sm font-medium text-slate-700">{label}</span>
        <input
          type="number"
          min="5"
          max="90"
          attr:data-test=pct_data_test
          prop:value=move || pct.get()
          on:input={
            let adjust = adjust_macros_pct.clone();
            move |ev| {
              if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                adjust(macro_type, val);
              }
            }
          }
          class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
        />
        <span class="text-sm text-slate-500">"%"</span>
        <input
          type="number"
          min="0"
          attr:data-test=grams_data_test
          prop:value=move || grams.get()
          on:input={
            let adjust = adjust_macros_grams.clone();
            move |ev| {
              if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                adjust(macro_type, val);
              }
            }
          }
          class="w-16 rounded border border-slate-300 px-2 py-1 text-sm text-right focus:border-blue-500 focus:outline-none"
        />
        <span class="text-sm text-slate-500">"g"</span>
      </div>
    }
}

/// Macro distribution section with pie chart and sliders
#[component]
fn MacroDistribution(
    daily_calories: ReadSignal<i32>,
    protein_pct: ReadSignal<i32>,
    set_protein_pct: WriteSignal<i32>,
    carbs_pct: ReadSignal<i32>,
    set_carbs_pct: WriteSignal<i32>,
    fat_pct: ReadSignal<i32>,
    set_fat_pct: WriteSignal<i32>,
) -> impl IntoView {
    // Which macro is locked (only one can be locked at a time)
    let (locked_macro, set_locked_macro) = signal(Option::<Macro>::None);

    // Function to adjust macros when one changes, keeping total at 100%
    let adjust_macros = move |changed: Macro, new_value: i32| {
        let new_value = new_value.clamp(5, 90);
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
                        let new_fat = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_fat;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        let new_carbs = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_carbs;
                        set_protein_pct.set(new_value);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
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
                        let new_fat = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_fat;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    Some(Macro::Fat) => {
                        let new_protein = (remaining - current_fat).clamp(5, 90);
                        let new_fat = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_value);
                        set_fat_pct.set(new_fat);
                    }
                    _ => {
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
                        let new_carbs = (remaining - current_protein).clamp(5, 90);
                        let new_protein = remaining - new_carbs;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    Some(Macro::Carbs) => {
                        let new_protein = (remaining - current_carbs).clamp(5, 90);
                        let new_carbs = remaining - new_protein;
                        set_protein_pct.set(new_protein);
                        set_carbs_pct.set(new_carbs);
                        set_fat_pct.set(new_value);
                    }
                    _ => {
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

    // Function to update macro from grams input
    let adjust_macros_from_grams = move |changed: Macro, grams: i32| {
        let cals = daily_calories.get() as f64;
        if cals <= 0.0 {
            return;
        }
        let cal_per_gram = match changed {
            Macro::Protein => CALORIES_PER_GRAM_PROTEIN,
            Macro::Carbs => CALORIES_PER_GRAM_CARBS,
            Macro::Fat => CALORIES_PER_GRAM_FAT,
        };
        let new_pct = ((grams as f64 * cal_per_gram / cals) * 100.0).round() as i32;
        adjust_macros(changed, new_pct);
    };

    view! {
      <div class="rounded-lg bg-white p-6 shadow-md">
        <h3 class="mb-4 text-xl font-semibold text-slate-900">"Macro Distribution"</h3>

        <div class="flex flex-col md:flex-row gap-6">
          <MacroPieChart protein_pct=protein_pct carbs_pct=carbs_pct fat_pct=fat_pct />

          <div class="flex-1 space-y-3">
            <div class="rounded bg-blue-50 px-3 py-2 text-xs text-blue-800">
              "Lock a macro to prevent it from auto-adjusting when others change"
            </div>

            <MacroInputRow
              macro_type=Macro::Protein
              label="Protein"
              color_class="bg-blue-600"
              border_color_class="border-blue-500"
              bg_color_class="bg-blue-50"
          pct_data_test="settings-protein-pct"
          grams_data_test="settings-protein-g"
              pct=protein_pct
              grams=protein_grams
              locked_macro=locked_macro
              set_locked_macro=set_locked_macro
              adjust_macros=adjust_macros
              adjust_macros_from_grams=adjust_macros_from_grams
            />

            <MacroInputRow
              macro_type=Macro::Carbs
              label="Carbs"
              color_class="bg-green-600"
              border_color_class="border-green-500"
              bg_color_class="bg-green-50"
          pct_data_test="settings-carbs-pct"
          grams_data_test="settings-carbs-g"
              pct=carbs_pct
              grams=carbs_grams
              locked_macro=locked_macro
              set_locked_macro=set_locked_macro
              adjust_macros=adjust_macros
              adjust_macros_from_grams=adjust_macros_from_grams
            />

            <MacroInputRow
              macro_type=Macro::Fat
              label="Fat"
              color_class="bg-orange-600"
              border_color_class="border-orange-500"
              bg_color_class="bg-orange-50"
          pct_data_test="settings-fat-pct"
          grams_data_test="settings-fat-g"
              pct=fat_pct
              grams=fat_grams
              locked_macro=locked_macro
              set_locked_macro=set_locked_macro
              adjust_macros=adjust_macros
              adjust_macros_from_grams=adjust_macros_from_grams
            />
          </div>
        </div>
      </div>
    }
}

/// Daily limits section (salt/sodium and saturated fat)
#[component]
fn DailyLimits(
    daily_calories: ReadSignal<i32>,
    sodium_mg: ReadSignal<i32>,
    set_sodium_mg: WriteSignal<i32>,
    sat_fat_grams: ReadSignal<i32>,
    set_sat_fat_grams: WriteSignal<i32>,
) -> impl IntoView {
    // Computed salt in grams (linked to sodium)
    let salt_grams = Memo::new(move |_| {
        // 1g salt = 393.4mg sodium; salt_g = sodium_mg / 393.4
        (sodium_mg.get() as f64 / 393.4 * 10.0).round() / 10.0
    });

    // Computed saturated fat percentage
    let sat_fat_pct = Memo::new(move |_| {
        let cals = daily_calories.get() as f64;
        if cals <= 0.0 {
            return 0.0;
        }
        let fat_cals = sat_fat_grams.get() as f64 * CALORIES_PER_GRAM_FAT;
        (fat_cals / cals * 100.0 * 10.0).round() / 10.0
    });

    view! {
      <div class="rounded-lg bg-white p-6 shadow-md">
        <h3 class="mb-4 text-xl font-semibold text-slate-900">"Daily Limits"</h3>
        <div class="space-y-4">
          // Salt/Sodium with linked inputs
          <div>
            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Salt Limit"</label>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="mb-1 block text-xs text-slate-500">"Sodium (mg)"</label>
                <input
                  type="number"
              attr:data-test="settings-sodium-mg"
                  prop:value=move || sodium_mg.get()
                  on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                      set_sodium_mg.set(val.max(0));
                    }
                  }
                  class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
              <div>
                <label class="mb-1 block text-xs text-slate-500">"Salt (g)"</label>
                <input
                  type="number"
                  step="0.1"
                  prop:value=move || salt_grams.get()
                  on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                      let sodium = (val * 393.4).round() as i32;
                      set_sodium_mg.set(sodium.max(0));
                    }
                  }
                  class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
            </div>
            <p class="mt-1 text-xs text-slate-500">"Recommended: 2300mg sodium / 5.8g salt (US), 2360mg / 6g (UK)"</p>
          </div>

          // Saturated Fat with dual inputs (grams and percentage)
          <div>
            <label class="mb-2 block text-sm font-medium text-slate-700">"Daily Saturated Fat Limit"</label>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="mb-1 block text-xs text-slate-500">"Grams"</label>
                <input
                  type="number"
              attr:data-test="settings-satfat-g"
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
                      let cals = daily_calories.get() as f64;
                      let grams = (pct / 100.0 * cals / CALORIES_PER_GRAM_FAT).round() as i32;
                      set_sat_fat_grams.set(grams.max(0));
                    }
                  }
                  class="w-full rounded border border-slate-300 px-4 py-2 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
            </div>
            <p class="mt-1 text-xs text-slate-500">
              "Recommended: Less than 10% of daily calories (AHA recommends <6% for heart health)"
            </p>
          </div>
        </div>
      </div>
    }
}

/// Daily minimums section (fiber)
#[component]
fn DailyMinimums(fiber_min: ReadSignal<i32>, set_fiber_min: WriteSignal<i32>) -> impl IntoView {
    view! {
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
    }
}

// ============================================================================
// Main Component
// ============================================================================

#[component]
pub fn Settings() -> impl IntoView {
    let initial_settings = load_settings().unwrap_or_else(default_settings);

    // Daily calorie goal
    let (daily_calories, set_daily_calories) = signal(initial_settings.daily_calories);

    // Macro distribution (must sum to 100)
    let (protein_pct, set_protein_pct) = signal(initial_settings.protein_pct);
    let (carbs_pct, set_carbs_pct) = signal(initial_settings.carbs_pct);
    let (fat_pct, set_fat_pct) = signal(initial_settings.fat_pct);

    // Salt/Sodium: store internally as sodium in mg
    let (sodium_mg, set_sodium_mg) = signal(initial_settings.sodium_mg);

    // Saturated fat: store as grams
    let (sat_fat_grams, set_sat_fat_grams) = signal(initial_settings.sat_fat_grams);

    // Fiber minimum
    let (fiber_min, set_fiber_min) = signal(initial_settings.fiber_min);

    Effect::new(move |_| {
        let settings = SettingsData {
            daily_calories: daily_calories.get(),
            protein_pct: protein_pct.get(),
            carbs_pct: carbs_pct.get(),
            fat_pct: fat_pct.get(),
            sodium_mg: sodium_mg.get(),
            sat_fat_grams: sat_fat_grams.get(),
            fiber_min: fiber_min.get(),
        };

        save_settings(&settings);
    });

    view! {
      <div class="mx-auto max-w-4xl py-6">
        <h2 class="mb-6 text-3xl font-bold text-slate-900">"Settings"</h2>

        <PresetButtons
          set_daily_calories=set_daily_calories
          set_protein_pct=set_protein_pct
          set_carbs_pct=set_carbs_pct
          set_fat_pct=set_fat_pct
          set_sodium_mg=set_sodium_mg
          set_sat_fat_grams=set_sat_fat_grams
          set_fiber_min=set_fiber_min
        />

        <div class="space-y-6">
          <DailyGoals daily_calories=daily_calories set_daily_calories=set_daily_calories />

          <MacroDistribution
            daily_calories=daily_calories
            protein_pct=protein_pct
            set_protein_pct=set_protein_pct
            carbs_pct=carbs_pct
            set_carbs_pct=set_carbs_pct
            fat_pct=fat_pct
            set_fat_pct=set_fat_pct
          />

          <DailyLimits
            daily_calories=daily_calories
            sodium_mg=sodium_mg
            set_sodium_mg=set_sodium_mg
            sat_fat_grams=sat_fat_grams
            set_sat_fat_grams=set_sat_fat_grams
          />

          <DailyMinimums fiber_min=fiber_min set_fiber_min=set_fiber_min />
        </div>
      </div>
    }
}
