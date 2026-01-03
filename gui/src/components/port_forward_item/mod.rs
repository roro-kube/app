// Port forwarding item component
//
// Displays a single port forwarding item with status and controls

#![allow(
    clippy::uninlined_format_args,
    clippy::redundant_clone,
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnested_or_patterns,
    unused_imports
)]

mod handlers;
mod ui;

use dioxus::prelude::*;
use roro_core::api::kubernetes::PortForwardingStatus;

/// Port forwarding item component props
#[derive(Props, PartialEq, Clone)]
pub struct PortForwardItemProps {
    pub namespace: String,
    pub pod: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub instance_id: String,
}

/// Port forwarding item component
///
/// Displays port forwarding status and provides start/stop controls
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn PortForwardItem(props: PortForwardItemProps) -> Element {
    // State for forward ID
    let forward_id = use_signal(|| None::<String>);

    // State for current status - None means not started
    let status = use_signal(|| None::<PortForwardingStatus>);

    // State for error message
    let error = use_signal(|| None::<String>);

    // Clone props for use in closures
    let namespace = props.namespace.clone();
    let pod = props.pod.clone();
    let remote_port = props.remote_port;
    let local_port = props.local_port;
    let instance_id = props.instance_id.clone();

    // Start port forward handler
    let handle_start = handlers::create_start_handler(
        forward_id,
        status,
        error,
        namespace.clone(),
        pod.clone(),
        remote_port,
        local_port,
        instance_id.clone(),
    );

    // Stop port forward handler
    let handle_stop = handlers::create_stop_handler(forward_id, status, error);

    // Determine if forward is active
    let is_active = status.read().as_ref().is_some_and(|s| {
        matches!(
            s,
            PortForwardingStatus::Active
                | PortForwardingStatus::Connecting
                | PortForwardingStatus::Reconnecting
        )
    });

    // Get status display text and color
    let (status_text, status_color) = ui::get_status_display(status.read().as_ref());

    rsx! {
        div {
            class: "p-4 border border-gray-300 rounded-lg shadow-sm bg-white",
            div {
                class: "flex items-center justify-between mb-2",
                div {
                    class: "flex-1",
                    h3 {
                        class: "text-lg font-semibold text-gray-800",
                        "Port Forward"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "{props.namespace} / {props.pod}"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "{props.remote_port} → localhost:{props.local_port}"
                    }
                }
                div {
                    class: "flex items-center gap-2",
                    span {
                        class: format!("text-sm font-medium {status_color}"),
                        {status_text}
                    }
                    if is_active {
                        button {
                            class: "px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600",
                            onclick: handle_stop,
                            "Stop"
                        }
                    } else {
                        button {
                            class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600",
                            onclick: handle_start,
                            "Start"
                        }
                    }
                }
            }
            if let Some(err_msg) = error.read().as_ref() {
                div {
                    class: "mt-2 p-2 bg-red-50 border border-red-200 rounded text-sm text-red-700",
                    {err_msg.clone()}
                }
            }
        }
    }
}
