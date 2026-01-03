// Event handlers for port forward item component
//
// This module provides event handlers for port forwarding operations.

use dioxus::prelude::*;
use roro_core::api::kubernetes::{get_or_init, PortForwardingConfig, PortForwardingStatus};

/// Create a handler for starting a port forward
#[allow(clippy::too_many_arguments)]
pub fn create_start_handler(
    forward_id: Signal<Option<String>>,
    status: Signal<Option<PortForwardingStatus>>,
    error: Signal<Option<String>>,
    namespace: String,
    pod: String,
    remote_port: u16,
    local_port: u16,
    instance_id: String,
) -> impl Fn(Event<MouseData>) + 'static {
    move |_| {
        let mut forward_id = forward_id;
        let mut status = status;
        let mut error = error;
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
                            let mut status_for_check = status;
                            // Use the same manager instance to check status
                            if let Some(state) = manager.get_forward(&id_for_status).await {
                                println!(
                                    "[PortForwardItem] Port forward status check: {:?}",
                                    state.status
                                );
                                status_for_check.set(Some(state.status));
                            } else {
                                // Forward was removed, set to failed
                                println!("[PortForwardItem] Port forward not found after start");
                                status_for_check.set(Some(PortForwardingStatus::Failed));
                            }
                        }
                        Err(e) => {
                            eprintln!("[PortForwardItem] Failed to start port forward: {:?}", e);
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
}

/// Create a handler for stopping a port forward
pub fn create_stop_handler(
    forward_id: Signal<Option<String>>,
    status: Signal<Option<PortForwardingStatus>>,
    error: Signal<Option<String>>,
) -> impl Fn(Event<MouseData>) + 'static {
    move |_| {
        let mut forward_id = forward_id;
        let mut status = status;
        let mut error = error;
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
                                eprintln!("[PortForwardItem] Failed to stop port forward: {:?}", e);
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
}
