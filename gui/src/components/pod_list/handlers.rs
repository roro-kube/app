// Event handlers for pod list components
//
// This module provides event handlers for port forwarding operations.

use dioxus::prelude::*;
use roro_core::api::kubernetes::{get_or_init, PortForwardingConfig, PortForwardingStatus};

/// Open a URL in the default browser
pub fn open_browser(local_port: u16) {
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

/// Create a handler for starting a port forward
#[allow(clippy::too_many_arguments)]
pub fn create_start_handler(
    forward_id: Signal<Option<String>>,
    status: Signal<Option<PortForwardingStatus>>,
    error: Signal<Option<String>>,
    namespace: String,
    pod_name: String,
    remote_port: u16,
    local_port: u16,
    instance_id: String,
) -> impl Fn(Event<MouseData>) + 'static {
    move |_| {
        let mut forward_id = forward_id;
        let mut status = status;
        let mut error = error;
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
                        local_port,
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
}
