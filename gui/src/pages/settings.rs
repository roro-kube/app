// Settings page component
//
// Displays workspace configuration loaded from core layer API

use crate::components::WorkspaceConfig;
use dioxus::prelude::*;
use roro_core::load_workstation_config;

use super::home::Page;

#[derive(Props, PartialEq, Clone)]
pub struct SettingsProps {
    pub on_navigate: Option<Signal<Page>>,
}

/// Settings page component
///
/// Loads and displays workspace configuration using the `WorkspaceConfig` component.
/// Shows loading and error states appropriately.
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn Settings(props: SettingsProps) -> Element {
    let config_resource = use_resource(|| async move { load_workstation_config().await });

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-8",
            div {
                class: "max-w-4xl mx-auto",
                div {
                    class: "flex items-center justify-between mb-6",
                    h1 {
                        class: "text-3xl font-bold text-gray-900",
                        "Settings"
                    }
                    if let Some(mut nav_signal) = props.on_navigate {
                        button {
                            class: "px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600",
                            onclick: move |_| {
                                nav_signal.set(Page::Home);
                            },
                            "Back"
                        }
                    }
                }
                if let Some(result) = config_resource.read().as_ref() {
                    if let Ok(config) = result {
                        WorkspaceConfig {
                            config: config.clone()
                        }
                    } else if let Err(e) = result {
                        div {
                            class: "p-4 bg-red-50 border border-red-200 rounded text-sm text-red-700",
                            "Failed to load workspace configuration: {e}"
                        }
                    }
                } else {
                    div {
                        class: "p-4 text-gray-600",
                        "Loading workspace configuration..."
                    }
                }
            }
        }
    }
}
