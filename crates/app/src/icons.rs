//! Icon components using Phosphor Icons
//!
//! This module provides a reusable Icon component that renders SVG icons
//! from the Phosphor Icons set.

use leptos::prelude::*;

/// Icon names available in the app
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconName {
    House,
    ShoppingCart,
    ChefHat,
    Gear,
}

/// Reusable icon component using Phosphor Icons
#[component]
pub fn Icon(
    /// The icon to display
    name: IconName,
    /// CSS classes to apply to the icon
    #[prop(default = "")]
    class: &'static str,
    /// Size of the icon in pixels (default: 20)
    #[prop(default = 20)]
    size: i32,
) -> impl IntoView {
    let icon_svg = match name {
        IconName::House => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor"><path d="M240,208H224V136l2.34,2.34A8,8,0,0,0,240,130.34Zm-32-29.66,2.34,2.34A8,8,0,0,0,224,173.66V208H168V152a8,8,0,0,0-8-8H96a8,8,0,0,0-8,8v56H40V120a8,8,0,0,0-2.34-5.66L128,44.9l32,24V64a8,8,0,0,0,16,0V83.1l26.34-19.76A8,8,0,0,0,200,69.1V120a8,8,0,0,0,8,8v80H208V178.34A8,8,0,0,0,208,178.34ZM152,152v56H104V152ZM56,112h40v40a8,8,0,0,0,8,8h48a8,8,0,0,0,8-8V112h40V69.1a8,8,0,0,0-3.12-6.38L128,36.9,59.12,62.72A8,8,0,0,0,56,69.1Z"/></svg>"#,
        IconName::ShoppingCart => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor"><path d="M104,216a16,16,0,1,1-16-16A16,16,0,0,1,104,216Zm88-16a16,16,0,1,1,16,16A16,16,0,0,1,192,200ZM239.71,74.14l-25.64,92.28A24.06,24.06,0,0,1,191,184H92.16A24.06,24.06,0,0,1,69,166.42L33.92,40H16a8,8,0,0,1,0-16H40a8,8,0,0,1,7.71,5.86L57.19,64H232a8,8,0,0,1,7.71,6.14l12,40A8,8,0,0,1,244,120H72.5l4.36,16H191a8,8,0,0,1,0,16H80.82l4.36,16H191a8,8,0,0,1,0,16H92.16a8,8,0,0,1,0-16H75.18l14.81-56H232a24.06,24.06,0,0,1,23.71,19.86Z"/></svg>"#,
        IconName::ChefHat => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor"><path d="M240,112a56,56,0,0,0-56-56c-.19,0-.38,0-.57,0L197.53,33a8,8,0,0,0-6.9-4H65.37a8,8,0,0,0-6.9,4L56.57,56c-.19,0-.38,0-.57,0A56,56,0,0,0,0,112v8a40,40,0,0,0,40,40h8v56a16,16,0,0,0,16,16H184a16,16,0,0,0,16-16V160h8a40,40,0,0,0,40-40Z"/></svg>"#,
        IconName::Gear => r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor"><path d="M128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm97.61-29.19a8,8,0,0,1-1.73,8.72l-20,11.81a8,8,0,0,1-11.06-2.33A52,52,0,0,0,168,186.39V200a8,8,0,0,1-8,8H96a8,8,0,0,1-8-8V186.39a52,52,0,0,0-25.79-38.36,8,8,0,0,1-11.06-2.33L31.12,134.53a8,8,0,0,1-1.73-8.72A56,56,0,0,1,56.83,78.19L45.14,58.17a8,8,0,0,1,2.27-11.05,56,56,0,0,1,101.18,0,8,8,0,0,1,2.27,11.05L135.17,78.19a56,56,0,0,1,30.44,52.62Zm-69.61,8.72A8,8,0,0,1,160,120a40,40,0,0,1-40,40,8,8,0,0,1,0-16,24,24,0,0,0,24-24,8,8,0,0,1,6.24-7.75ZM96,120a8,8,0,0,1,6.24,7.75A24,24,0,0,0,126.24,152,8,8,0,0,1,128,168a40,40,0,0,1-40-40A8,8,0,0,1,96,120Z"/></svg>"#,
    };

    view! {
        <span
            class=class
            style=format!("width: {}px; height: {}px;", size, size)
            inner_html=icon_svg
        />
    }
}