// Port forwarding task management
//
// This module handles spawning and managing port forwarding tasks.

use crate::api::kubernetes::portforwarding::types::{PortForwardingConfig, PortForwardingStatus};
use crate::errors::CoreError;
use k8s_openapi::api::core::v1::Pod;
use kube::api::Api;
use kube::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

use super::types::PortForwardingState;

pub type ForwardTaskHandle = (JoinHandle<()>, mpsc::Sender<()>);
pub type ForwardTaskMap = Arc<RwLock<HashMap<String, ForwardTaskHandle>>>;

/// Spawn a port forwarding task
#[allow(clippy::too_many_lines)]
pub async fn spawn_forward_task(
    client: Client,
    forwards: Arc<RwLock<HashMap<String, PortForwardingState>>>,
    forward_tasks: ForwardTaskMap,
    forward_id: String,
    config: PortForwardingConfig,
    reconnect_delay: Duration,
    max_retries: u32,
) -> Result<(), CoreError> {
    let manager_forwards = Arc::clone(&forwards);
    let manager_tasks = Arc::clone(&forward_tasks);

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    let forward_id_clone = forward_id.clone();

    let handle = tokio::spawn(async move {
        // Create Pod API
        let pods: Api<Pod> = Api::namespaced(client.clone(), &config.namespace);

        // Note: Pod validation is done in start_forward() before spawning this task
        // So we can assume the pod exists here

        // Set up local TCP listener
        let listener = match TokioTcpListener::bind(("127.0.0.1", config.local_port)).await {
            Ok(listener) => listener,
            Err(_e) => {
                let mut f = forwards.write().await;
                if let Some(state) = f.get_mut(&forward_id_clone) {
                    state.status = PortForwardingStatus::Failed;
                }
                drop(f);

                let retry_count = {
                    let mut f = forwards.write().await;
                    if let Some(state) = f.get_mut(&forward_id_clone) {
                        state.retry_count
                    } else {
                        return;
                    }
                };

                if retry_count < max_retries {
                    tokio::time::sleep(reconnect_delay * retry_count).await;
                    let _ = reconnect_forward_internal(
                        &manager_forwards,
                        &manager_tasks,
                        &forward_id_clone,
                        &config,
                        reconnect_delay,
                        max_retries,
                    )
                    .await;
                }
                return;
            }
        };

        // Listener is bound and ready - task is running successfully
        // Status is already set to Active in start_forward() after spawning
        // If we reach here, the task is healthy and running

        // Main forwarding loop
        loop {
            tokio::select! {
                _msg = shutdown_rx.recv() => {
                    // Graceful shutdown - user requested stop
                    // Don't change status to Failed on graceful shutdown
                    // The forward will be removed from active_forwards in stop_forward()
                    return;
                }
                result = listener.accept() => {
                    match result {
                        Ok((local_stream, _)) => {
                            // Spawn a task to handle this connection
                            // Create a new portforwarder for each connection
                            let pods_clone = pods.clone();
                            let pod_name = config.pod.clone();
                            let remote_port = config.remote_port;

                            tokio::spawn(async move {
                                // Create a new portforwarder for this connection
                                match pods_clone.portforward(&pod_name, &[remote_port]).await {
                                    Ok(mut pf) => {
                                        // Take the stream for the remote port
                                        if let Some(remote_stream) = pf.take_stream(remote_port) {
                                            // Status is already Active when listener is bound
                                            // No need to update status here

                                            // Split streams for bidirectional copying
                                            let (mut local_read, mut local_write) = io::split(local_stream);
                                            let (mut remote_read, mut remote_write) = io::split(remote_stream);

                                            // Spawn tasks for bidirectional data copying
                                            let local_to_remote = tokio::spawn(async move {
                                                let _ = io::copy(&mut local_read, &mut remote_write).await;
                                            });

                                            let remote_to_local = tokio::spawn(async move {
                                                let _ = io::copy(&mut remote_read, &mut local_write).await;
                                            });

                                            // Wait for either direction to finish
                                            tokio::select! {
                                                _ = local_to_remote => {}
                                                _ = remote_to_local => {}
                                            }
                                        } else {
                                            eprintln!(
                                                "[PortForward] Failed to take stream for remote port {remote_port}"
                                            );
                                            // Log error but don't change status - individual connection failures
                                            // shouldn't affect the overall port forward status
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "[PortForward] Failed to create portforwarder for remote port {remote_port}: {e}"
                                        );
                                        // Log error but don't change status - individual connection failures
                                        // shouldn't affect the overall port forward status
                                        // The port forward is still active and can accept other connections
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("[PortForward] Failed to accept connection: {e}");
                            // Update status on accept error
                            let mut f = forwards.write().await;
                            if let Some(state) = f.get_mut(&forward_id_clone) {
                                if state.status == PortForwardingStatus::Active {
                                    state.status = PortForwardingStatus::Failed;
                                }
                            }
                            // Break the loop on accept error
                            break;
                        }
                    }
                }
            }
        }

        // If we reach here, the loop exited unexpectedly (not via shutdown)
        // This means there was an error - update status to Failed
        let mut f = forwards.write().await;
        if let Some(state) = f.get_mut(&forward_id_clone) {
            if state.status == PortForwardingStatus::Active {
                state.status = PortForwardingStatus::Failed;
            }
        }
    });

    let mut task_map = forward_tasks.write().await;
    task_map.insert(forward_id, (handle, shutdown_tx));

    Ok(())
}

async fn reconnect_forward_internal(
    forwards: &Arc<RwLock<HashMap<String, PortForwardingState>>>,
    tasks: &ForwardTaskMap,
    forward_id: &str,
    _config: &PortForwardingConfig,
    reconnect_delay: Duration,
    max_retries: u32,
) -> Result<(), CoreError> {
    let state = {
        let f = forwards.read().await;
        f.get(forward_id).cloned()
    };

    let state = state.ok_or_else(|| {
        CoreError::PortForwardingNotFound(format!("Forward not found: {forward_id}"))
    })?;

    if state.retry_count >= max_retries {
        let mut f = forwards.write().await;
        if let Some(s) = f.get_mut(forward_id) {
            s.status = PortForwardingStatus::Failed;
        }
        return Err(CoreError::PortForwarding(format!(
            "Max retries exceeded for {forward_id}"
        )));
    }

    {
        let mut f = forwards.write().await;
        if let Some(s) = f.get_mut(forward_id) {
            s.status = PortForwardingStatus::Reconnecting;
            s.retry_count += 1;
        }
    }

    tokio::time::sleep(reconnect_delay * state.retry_count).await;

    let mut t = tasks.write().await;
    if let Some((handle, shutdown_tx)) = t.remove(forward_id) {
        drop(shutdown_tx);
        handle.abort();
    }

    Ok(())
}
