// Workspace configuration component
//
// Displays workspace configuration (WorkstationConfig) as detailed cards.
// Each card shows all fields from an AppReference entry.

#![allow(
    clippy::uninlined_format_args,
    clippy::redundant_clone,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnested_or_patterns,
    unused_imports
)]

use dioxus::prelude::*;
use roro_domain::{AppReference, WorkstationConfig};

/// Workspace configuration component props
#[derive(Props, PartialEq, Clone)]
pub struct WorkspaceConfigProps {
    pub config: WorkstationConfig,
}

/// Workspace configuration component
///
/// Displays workspace configuration as detailed cards, one for each app reference.
/// Shows all fields including name, git URL, local path, sync interval, and Kubernetes context.
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn WorkspaceConfig(props: WorkspaceConfigProps) -> Element {
    if props.config.is_empty() {
        return rsx! {
            div {
                class: "p-4 text-gray-600",
                "No workspace configuration found"
            }
        };
    }

    rsx! {
        div {
            class: "space-y-4",
            for app in &props.config {
                div {
                    class: "p-4 border border-gray-300 rounded-lg shadow-sm bg-white",
                    h3 {
                        class: "text-lg font-semibold text-gray-800 mb-3",
                        {app.name.clone()}
                    }
                    div {
                        class: "space-y-2",
                        div {
                            class: "text-sm",
                            span {
                                class: "font-medium text-gray-700",
                                "Git URL: "
                            }
                            span {
                                class: "text-gray-600",
                                {app.git_url.clone()}
                            }
                        }
                        div {
                            class: "text-sm",
                            span {
                                class: "font-medium text-gray-700",
                                "Local Path: "
                            }
                            span {
                                class: "text-gray-600",
                                {
                                    if let Some(path) = &app.local_path {
                                        path.clone()
                                    } else {
                                        format!("~/.roro/remote/{}", app.name)
                                    }
                                }
                            }
                        }
                        div {
                            class: "text-sm",
                            span {
                                class: "font-medium text-gray-700",
                                "Sync Interval: "
                            }
                            span {
                                class: "text-gray-600",
                                "{app.get_sync_interval()} ms"
                            }
                        }
                        div {
                            class: "text-sm",
                            span {
                                class: "font-medium text-gray-700",
                                "Kubernetes Context: "
                            }
                            span {
                                class: "text-gray-600",
                                {
                                    match &app.kubectl_context {
                                        Some(ctx) => ctx.as_str(),
                                        None => "Not specified",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
