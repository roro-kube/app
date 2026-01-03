// Home page component
//
// Main home page that displays port forwarding items and other content

use crate::components::PodList;
use dioxus::prelude::*;

/// Home page component
///
/// Displays the main home page with port forwarding items
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-8",
            div {
                class: "max-w-4xl mx-auto",
                h1 {
                    class: "text-3xl font-bold text-gray-900 mb-6",
                    "Roro Kube"
                }
                PodList {}
            }
        }
    }
}
