// Home page component
//
// Main home page that displays port forwarding items and other content

use crate::components::PodList;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct HomeProps {
    pub on_navigate: Option<Signal<Page>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Settings,
}

/// Home page component
///
/// Displays the main home page with port forwarding items
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn Home(props: HomeProps) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-8",
            div {
                class: "max-w-4xl mx-auto",
                div {
                    class: "flex items-center justify-between mb-6",
                    h1 {
                        class: "text-3xl font-bold text-gray-900",
                        "Roro Kube"
                    }
                    if let Some(mut nav_signal) = props.on_navigate {
                        button {
                            class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: move |_| {
                                nav_signal.set(Page::Settings);
                            },
                            "Settings"
                        }
                    }
                }
                PodList {}
            }
        }
    }
}
