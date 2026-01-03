// Dioxus components module
//
// This module will contain reusable Dioxus UI components.
// Components will be added in future tasks.

mod pod_list;
mod port_forward_item;

pub use pod_list::PodList;
#[allow(unused_imports)]
pub use port_forward_item::PortForwardItem;

use dioxus::prelude::*;

/// Simple greeting component that displays a welcome message
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
/// This allow attribute is required by the framework, not optional.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn Greeting() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen",
            h1 {
                class: "text-4xl font-bold text-gray-800",
                "Hello, Roro Kube!"
            }
        }
    }
}
