// Dioxus components module
// 
// This module will contain reusable Dioxus UI components.
// Components will be added in future tasks.

use dioxus::prelude::*;

/// Simple greeting component that displays a welcome message
#[allow(non_snake_case)]
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

