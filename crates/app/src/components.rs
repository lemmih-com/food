//! Shared UI components
//!
//! Contains reusable components used across the application.

use leptos::prelude::*;
use leptos_router::components::A;
use wasm_bindgen::JsCast;

// ============================================================================
// Icon Components
// ============================================================================

/// Close (X) icon - used in modals and dismissible elements
#[component]
pub fn CloseIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-6 w-6".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    }
}

/// Plus icon - used for add buttons
#[component]
pub fn PlusIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-4 w-4".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
    }
}

/// Edit (pencil) icon - used for edit buttons
#[component]
pub fn EditIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-5 w-5".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
        />
      </svg>
    }
}

/// Lock icon (locked state)
#[component]
pub fn LockIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-4 w-4".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
        />
      </svg>
    }
}

/// Unlock icon (unlocked state)
#[component]
pub fn UnlockIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-4 w-4".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"
        />
      </svg>
    }
}

/// Upload icon
#[component]
pub fn UploadIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-4 w-4".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
        />
      </svg>
    }
}

/// Image placeholder icon
#[component]
pub fn ImageIcon(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-12 w-12".to_string()
    } else {
        class
    };
    view! {
      <svg class=class fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
        />
      </svg>
    }
}

// ============================================================================
// Modal Backdrop Helper
// ============================================================================

/// Helper function to check if click was on backdrop (not content)
/// Used for click-outside-to-close functionality in modals
pub fn is_backdrop_click(ev: &web_sys::MouseEvent, backdrop_id: &str) -> bool {
    ev.target()
        .and_then(|t| {
            t.dyn_ref::<web_sys::HtmlElement>()
                .map(|e| e.id() == backdrop_id)
        })
        .unwrap_or(false)
}

/// Error display component for use inside modals
#[component]
pub fn ModalError(error: RwSignal<Option<String>>) -> impl IntoView {
    view! {
      <Show when=move || error.get().is_some()>
        <div class="mb-4 rounded bg-red-100 dark:bg-red-900/30 px-4 py-2 text-sm text-red-700 dark:text-red-400">
          {move || error.get().unwrap_or_default()}
        </div>
      </Show>
    }
}

/// Standard modal footer with Cancel and Save buttons
#[component]
pub fn ModalFooter(
    saving: ReadSignal<bool>,
    on_cancel: impl Fn() + Clone + Send + Sync + 'static,
    on_save: impl Fn() + Clone + Send + Sync + 'static,
    #[prop(default = "Save")] save_label: &'static str,
    #[prop(default = "Saving...")] saving_label: &'static str,
) -> impl IntoView {
    view! {
      <div class="mt-6 flex justify-end gap-3">
        <button
          class="rounded bg-slate-200 dark:bg-slate-600 px-4 py-2 font-medium text-slate-700 dark:text-slate-200 hover:bg-slate-300 dark:hover:bg-slate-500"
          on:click={
            let on_cancel = on_cancel.clone();
            move |_| on_cancel()
          }
        >
          "Cancel"
        </button>
        <button
          class="rounded bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-700 disabled:bg-blue-300 dark:disabled:bg-blue-800"
          disabled=move || saving.get()
          on:click={
            let on_save = on_save.clone();
            move |_| on_save()
          }
        >
          {move || if saving.get() { saving_label } else { save_label }}
        </button>
      </div>
    }
}

/// Delete confirmation component for modals
#[component]
pub fn DeleteConfirmation(
    show_confirm: RwSignal<bool>,
    on_delete: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
    view! {
      <Show
        when=move || show_confirm.get()
        fallback=move || {
          view! {
            <button
              class="rounded bg-red-100 dark:bg-red-900/30 px-4 py-2 font-medium text-red-700 dark:text-red-400 hover:bg-red-200 dark:hover:bg-red-900/50"
              on:click=move |_| show_confirm.set(true)
            >
              "Delete"
            </button>
          }
        }
      >
        <div class="flex items-center gap-2">
          <span class="text-sm text-red-700 dark:text-red-400">"Are you sure?"</span>
          <button
            class="rounded bg-red-600 px-3 py-1 text-sm font-medium text-white hover:bg-red-700"
            on:click={
              let on_delete = on_delete.clone();
              move |_| on_delete()
            }
          >
            "Yes, delete"
          </button>
          <button
            class="rounded bg-slate-200 dark:bg-slate-600 px-3 py-1 text-sm font-medium text-slate-700 dark:text-slate-200 hover:bg-slate-300 dark:hover:bg-slate-500"
            on:click=move |_| show_confirm.set(false)
          >
            "Cancel"
          </button>
        </div>
      </Show>
    }
}

// ============================================================================
// Star Icon Component
// ============================================================================

/// SVG star icon component used for ratings.
///
/// # Props
/// - `filled`: Whether the star should be filled or empty
/// - `class`: Additional CSS classes
#[component]
pub fn StarIcon(filled: bool, #[prop(optional, into)] class: String) -> impl IntoView {
    let fill = if filled { "currentColor" } else { "none" };
    let color_class = if filled {
        "text-yellow-400"
    } else {
        "text-slate-300 dark:text-slate-600"
    };

    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill=fill
        stroke="currentColor"
        stroke-width="1.5"
        class=format!("{} {}", color_class, class)
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
        />
      </svg>
    }
}

// ============================================================================
// Navigation Link Component
// ============================================================================

/// Reusable navigation link component with consistent styling.
///
/// # Props
/// - `href`: Link destination
/// - `label`: Link text
/// - `mobile`: If true, uses mobile-friendly styling (block display)
#[component]
pub fn NavLink(
    href: &'static str,
    label: &'static str,
    #[prop(default = false)] mobile: bool,
) -> impl IntoView {
    let class = if mobile {
        "block rounded px-3 py-2 text-sm font-medium hover:bg-slate-700 dark:hover:bg-slate-800"
    } else {
        "rounded px-3 py-2 text-sm font-medium hover:bg-slate-700 dark:hover:bg-slate-800"
    };

    view! {
      <A href=href attr:class=class>
        {label}
      </A>
    }
}

// ============================================================================
// Form Input Utilities
// ============================================================================

/// Standard input field CSS classes
pub const INPUT_CLASS: &str = "w-full rounded border border-slate-300 dark:border-slate-600 px-3 py-2 text-sm bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500";

/// Standard label CSS classes
pub const LABEL_CLASS: &str = "block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1";

// ============================================================================
// Content Type Detection
// ============================================================================

/// Detect content type from a data URL prefix
///
/// # Arguments
/// * `data_url_prefix` - The part before the comma in a data URL (e.g., "data:image/png;base64")
///
/// # Returns
/// The detected MIME type, defaulting to "image/jpeg" if unknown
pub fn detect_content_type(data_url_prefix: &str) -> &'static str {
    if data_url_prefix.contains("png") {
        "image/png"
    } else if data_url_prefix.contains("gif") {
        "image/gif"
    } else if data_url_prefix.contains("webp") {
        "image/webp"
    } else {
        "image/jpeg"
    }
}
