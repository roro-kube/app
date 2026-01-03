// Pod list component
//
// Displays all pods with their ports and port forward buttons

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
use roro_core::api::kubernetes::{
    get_or_init, KubernetesClient, PortForwardingConfig, PortForwardingStatus,
};

#[derive(Clone, Debug)]
struct PodPortInfo {
    namespace: String,
    pod_name: String,
    ports: Vec<u16>,
}

/// Pod list component
///
/// Displays all pods with their ports and port forward buttons
///
/// # Note
/// Dioxus requires component functions to use `PascalCase` naming convention.
#[allow(non_snake_case)] // Required by Dioxus: component functions must use PascalCase
pub fn PodList() -> Element {
    let pods_resource = use_resource(|| async move {
        match KubernetesClient::new_with_context("rancher-desktop").await {
            Ok(client) => {
                match client.list_all_pods().await {
                    Ok(pods) => {
                        let mut pod_info = Vec::new();
                        for pod in pods {
                            if let (Some(pod_name), Some(namespace)) =
                                (pod.metadata.name.as_ref(), pod.metadata.namespace.as_ref())
                            {
                                let ports_map = KubernetesClient::extract_pod_ports(&pod);
                                // Flatten all ports from all containers
                                let mut all_ports = Vec::new();
                                for ports in ports_map.values() {
                                    all_ports.extend(ports.iter().copied());
                                }
                                // Remove duplicates and sort
                                all_ports.sort_unstable();
                                all_ports.dedup();

                                if !all_ports.is_empty() {
                                    pod_info.push(PodPortInfo {
                                        namespace: namespace.clone(),
                                        pod_name: pod_name.clone(),
                                        ports: all_ports,
                                    });
                                }
                            }
                        }
                        Some(pod_info)
                    }
                    Err(e) => {
                        eprintln!("[PodList] Failed to list pods: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("[PodList] Failed to initialize client: {:?}", e);
                None
            }
        }
    });

    rsx! {
        div {
            class: "space-y-4",
            if let Some(Some(pods)) = pods_resource.read().as_ref() {
                for pod_info in pods {
                    div {
                        class: "p-4 border border-gray-300 rounded-lg shadow-sm bg-white",
                        div {
                            class: "mb-2",
                            h3 {
                                class: "text-lg font-semibold text-gray-800",
                                "{pod_info.namespace} / {pod_info.pod_name}"
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2",
                            for port in &pod_info.ports {
                                PodPortButton {
                                    namespace: pod_info.namespace.clone(),
                                    pod_name: pod_info.pod_name.clone(),
                                    remote_port: *port,
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    class: "p-4 text-gray-600",
                    "Loading pods..."
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct PodPortButtonProps {
    namespace: String,
    pod_name: String,
    remote_port: u16,
}

#[allow(non_snake_case)]
fn PodPortButton(props: PodPortButtonProps) -> Element {
    let forward_id = use_signal(|| None::<String>);
    let status = use_signal(|| None::<PortForwardingStatus>);
    let error = use_signal(|| None::<String>);

    // Generate local port (use remote port + 50000 as base)
    let local_port = 50000 + props.remote_port as u32;

    let namespace = props.namespace.clone();
    let pod_name = props.pod_name.clone();
    let remote_port = props.remote_port;
    let instance_id = format!("{}-{}", props.namespace, props.pod_name);

    let handle_start = {
        let mut forward_id = forward_id.clone();
        let mut status = status.clone();
        let mut error = error.clone();
        let namespace = namespace.clone();
        let pod_name = pod_name.clone();
        let instance_id = instance_id.clone();
        move |_| {
            let namespace = namespace.clone();
            let pod_name = pod_name.clone();
            let instance_id = instance_id.clone();
            spawn(async move {
                match get_or_init("rancher-desktop").await {
                    Ok(manager) => {
                        let config = PortForwardingConfig {
                            namespace: namespace.clone(),
                            pod: pod_name.clone(),
                            remote_port,
                            local_port: local_port as u16,
                            instance_id: instance_id.clone(),
                        };

                        error.set(None);
                        status.set(Some(PortForwardingStatus::Connecting));

                        match manager.start_forward(config).await {
                            Ok(id) => {
                                forward_id.set(Some(id));
                                status.set(Some(PortForwardingStatus::Active));
                            }
                            Err(e) => {
                                eprintln!("[PodPortButton] Failed to start port forward: {:?}", e);
                                error.set(Some(format!("{:?}", e)));
                                status.set(Some(PortForwardingStatus::Failed));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[PodPortButton] Failed to get manager: {:?}", e);
                        error.set(Some(format!("Failed to initialize: {:?}", e)));
                        status.set(Some(PortForwardingStatus::Failed));
                    }
                }
            });
        }
    };

    let handle_stop = {
        let mut forward_id = forward_id.clone();
        let mut status = status.clone();
        let mut error = error.clone();
        move |_| {
            let forward_id_val = forward_id.read().clone();
            spawn(async move {
                if let Some(id) = forward_id_val {
                    match get_or_init("rancher-desktop").await {
                        Ok(manager) => match manager.stop_forward(&id).await {
                            Ok(()) => {
                                forward_id.set(None);
                                status.set(None);
                            }
                            Err(e) => {
                                eprintln!("[PodPortButton] Failed to stop: {:?}", e);
                                error.set(Some(format!("{:?}", e)));
                            }
                        },
                        Err(e) => {
                            eprintln!("[PodPortButton] Failed to get manager: {:?}", e);
                            error.set(Some(format!("{:?}", e)));
                        }
                    }
                }
            });
        }
    };

    let is_active = status.read().as_ref().is_some_and(|s| {
        matches!(
            s,
            PortForwardingStatus::Active
                | PortForwardingStatus::Connecting
                | PortForwardingStatus::Reconnecting
        )
    });

    let handle_open_browser = {
        let local_port = local_port;
        move |_| {
            let url = format!("http://localhost:{local_port}");
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", &url])
                    .spawn()
                    .ok();
            }
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open").arg(&url).spawn().ok();
            }
            #[cfg(target_os = "linux")]
            {
                std::process::Command::new("xdg-open")
                    .arg(&url)
                    .spawn()
                    .ok();
            }
        }
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
