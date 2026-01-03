// Pod port button component
//
// This module provides the PodPortButton component for displaying port forwarding controls.

#![allow(
    clippy::uninlined_format_args,
    clippy::redundant_clone,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnested_or_patterns,
    unused_imports
)]

use dioxus::prelude::*;
use roro_core::api::kubernetes::PortForwardingStatus;

use super::handlers;

#[derive(Props, PartialEq, Clone)]
pub struct PodPortButtonProps {
    pub namespace: String,
    pub pod_name: String,
    pub remote_port: u16,
}

#[allow(non_snake_case)]
pub fn PodPortButton(props: PodPortButtonProps) -> Element {
    let forward_id = use_signal(|| None::<String>);
    let status = use_signal(|| None::<PortForwardingStatus>);
    let error = use_signal(|| None::<String>);

    // Generate local port (use remote port + 50000 as base)
    let local_port = 50000 + props.remote_port as u32;

    let namespace = props.namespace.clone();
    let pod_name = props.pod_name.clone();
    let remote_port = props.remote_port;
    let instance_id = format!("{}-{}", props.namespace, props.pod_name);

    let handle_start = handlers::create_start_handler(
        forward_id,
        status,
        error,
        namespace.clone(),
        pod_name.clone(),
        remote_port,
        local_port as u16,
        instance_id.clone(),
    );

    let handle_stop = handlers::create_stop_handler(forward_id, status, error);

    let is_active = status.read().as_ref().is_some_and(|s| {
        matches!(
            s,
            PortForwardingStatus::Active
                | PortForwardingStatus::Connecting
                | PortForwardingStatus::Reconnecting
        )
    });

    let handle_open_browser = move |_| {
        handlers::open_browser(local_port as u16);
    };

    rsx! {
        div {
            class: "flex items-center gap-2 px-3 py-1 border border-gray-300 rounded",
            if is_active {
                span {
                    class: "text-sm text-blue-600 cursor-pointer hover:underline",
                    onclick: handle_open_browser,
                    "{remote_port} → {local_port}"
                }
            } else {
                span {
                    class: "text-sm text-gray-700",
                    "{remote_port} → {local_port}"
                }
            }
            if is_active {
                button {
                    class: "px-2 py-1 bg-red-500 text-white text-xs rounded hover:bg-red-600",
                    onclick: handle_stop,
                    "Stop"
                }
            } else {
                button {
                    class: "px-2 py-1 bg-blue-500 text-white text-xs rounded hover:bg-blue-600",
                    onclick: handle_start,
                    "Forward"
                }
            }
        }
    }
}
