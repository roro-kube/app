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

mod handlers;
mod port_button;

use dioxus::prelude::*;
use roro_core::api::kubernetes::KubernetesClient;

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
                                port_button::PodPortButton {
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
