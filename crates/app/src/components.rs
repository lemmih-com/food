//! Shared UI components
//!
//! Contains reusable components used across the application.

use leptos::prelude::*;
use leptos_router::components::A;

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
