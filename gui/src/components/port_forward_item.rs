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

use dioxus::prelude::*;
use roro_core::api::kubernetes::{get_or_init, PortForwardingConfig, PortForwardingStatus};

/// Port forwarding item component
///
/// Displays port forwarding status and provides start/stop controls
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
#[derive(Props, PartialEq, Clone)]
pub struct PortForwardItemProps {
    pub namespace: String,
    pub pod: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub instance_id: String,
}

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
    let handle_start = {
        let mut forward_id = forward_id;
        let mut status = status;
        let mut error = error;
        let namespace = namespace.clone();
        let pod = pod.clone();
        let instance_id = instance_id.clone();
        move |_| {
            let namespace = namespace.clone();
            let pod = pod.clone();
            let instance_id = instance_id.clone();
            spawn(async move {
                // Get or initialize the singleton manager
                match get_or_init("rancher-desktop").await {
                    Ok(manager) => {
                        let config = PortForwardingConfig {
                            namespace,
                            pod,
                            remote_port,
                            local_port,
                            instance_id,
                        };

                        println!(
                            "[PortForwardItem] Starting port forward: {}:{} -> localhost:{}",
                            config.namespace, config.pod, config.local_port
                        );

                        error.set(None);
                        status.set(Some(PortForwardingStatus::Connecting));

                        match manager.start_forward(config).await {
                            Ok(id) => {
                                println!(
                                    "[PortForwardItem] Port forward started successfully with ID: {}",
                                    id
                                );
                                forward_id.set(Some(id.clone()));

                                // Check status immediately after starting using the same manager
                                let id_for_status = id.clone();
                                let mut status_for_check = status.clone();
                                // Use the same manager instance to check status
                                if let Some(state) = manager.get_forward(&id_for_status).await {
                                    println!(
                                        "[PortForwardItem] Port forward status check: {:?}",
                                        state.status
                                    );
                                    status_for_check.set(Some(state.status));
                                } else {
                                    // Forward was removed, set to failed
                                    println!(
                                        "[PortForwardItem] Port forward not found after start"
                                    );
                                    status_for_check.set(Some(PortForwardingStatus::Failed));
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "[PortForwardItem] Failed to start port forward: {:?}",
                                    e
                                );
                                error.set(Some(format!("{:?}", e)));
                                status.set(Some(PortForwardingStatus::Failed));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[PortForwardItem] Failed to initialize manager: {:?}", e);
                        error.set(Some(format!("Failed to initialize: {:?}", e)));
                        status.set(Some(PortForwardingStatus::Failed));
                    }
                }
            });
        }
    };

    // Stop port forward handler
    let handle_stop = {
        let mut forward_id = forward_id;
        let mut status = status;
        let mut error = error;
        move |_| {
            let forward_id_val = forward_id.read().clone();
            spawn(async move {
                if let Some(id) = forward_id_val {
                    println!("[PortForwardItem] Stopping port forward: {}", id);

                    // Get the singleton manager (should already be initialized)
                    match get_or_init("rancher-desktop").await {
                        Ok(manager) => {
                            error.set(None);

                            match manager.stop_forward(&id).await {
                                Ok(()) => {
                                    println!("[PortForwardItem] Port forward stopped successfully");
                                    forward_id.set(None);
                                    status.set(None); // Reset to not started
                                }
                                Err(e) => {
                                    eprintln!(
                                        "[PortForwardItem] Failed to stop port forward: {:?}",
                                        e
                                    );
                                    error.set(Some(format!("{:?}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[PortForwardItem] Failed to get manager: {:?}", e);
                            error.set(Some(format!("Failed to get manager: {:?}", e)));
                        }
                    }
                }
            });
        }
    };

    // Determine if forward is active
    let is_active = status.read().as_ref().is_some_and(|s| {
        matches!(
            s,
            PortForwardingStatus::Active
                | PortForwardingStatus::Connecting
                | PortForwardingStatus::Reconnecting
        )
    });

    // Get status display text
    let status_text = match status.read().as_ref() {
        None => "Not Started",
        Some(PortForwardingStatus::Connecting) => "Connecting...",
        Some(PortForwardingStatus::Active) => "Active",
        Some(PortForwardingStatus::Failed) => "Failed",
        Some(PortForwardingStatus::Reconnecting) => "Reconnecting...",
    };

    // Get status color
    let status_color = match status.read().as_ref() {
        None => "text-gray-600",
        Some(PortForwardingStatus::Active) => "text-green-600",
        Some(PortForwardingStatus::Failed) => "text-red-600",
        Some(PortForwardingStatus::Connecting) | Some(PortForwardingStatus::Reconnecting) => {
            "text-yellow-600"
        }
    };

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
