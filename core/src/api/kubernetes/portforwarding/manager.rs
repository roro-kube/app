// Port forwarding manager
//
// This module provides the main PortForwardingManager implementation.

use crate::api::kubernetes::client::KubernetesClient;
use crate::api::kubernetes::portforwarding::health;
use crate::api::kubernetes::portforwarding::task::{spawn_forward_task, ForwardTaskMap};
use crate::api::kubernetes::portforwarding::types::{
    PortForwardingConfig, PortForwardingState, PortForwardingStatus,
};
use crate::errors::CoreError;
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams};
use kube::Client;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

pub struct PortForwardingManager {
    active_forwards: Arc<RwLock<HashMap<String, PortForwardingState>>>,
    client: Client,
    health_check_interval: Duration,
    reconnect_delay: Duration,
    max_retries: u32,
    forward_tasks: ForwardTaskMap,
}

impl PortForwardingManager {
    #[must_use]
    pub fn new(client: &KubernetesClient) -> Self {
        Self {
            active_forwards: Arc::new(RwLock::new(HashMap::new())),
            client: client.inner().clone(),
            health_check_interval: Duration::from_secs(30),
            reconnect_delay: Duration::from_secs(5),
            max_retries: 5,
            forward_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[must_use]
    pub fn with_health_check_interval(mut self, interval: Duration) -> Self {
        self.health_check_interval = interval;
        self
    }

    #[must_use]
    pub fn with_reconnect_delay(mut self, delay: Duration) -> Self {
        self.reconnect_delay = delay;
        self
    }

    #[must_use]
    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Start a port forward
    ///
    /// # Errors
    /// Returns an error if the port is already in use, if the forward already exists, or if the pod doesn't exist
    pub async fn start_forward(
        &self,
        mut config: PortForwardingConfig,
    ) -> Result<String, CoreError> {
        self.check_port_available(config.local_port)?;

        // Validate pod exists before creating the forward state
        // Try exact match first, then try to find a pod that starts with the name
        // (for deployment-generated pod names like "app-abc123-xyz")
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), &config.namespace);
        let actual_pod_name = if pods.get(&config.pod).await.is_ok() {
            config.pod.clone() // Exact match found
        } else {
            // Try to find a pod that starts with the configured name
            let pod_list = pods.list(&ListParams::default()).await.map_err(|e| {
                CoreError::PortForwarding(format!(
                    "Failed to list pods in namespace {}: {}",
                    config.namespace, e
                ))
            })?;

            // Find a pod whose name starts with the configured pod name
            let matching_pod = pod_list.iter().find(|p| {
                p.metadata
                    .name
                    .as_ref()
                    .is_some_and(|n| n.starts_with(&config.pod))
            });

            match matching_pod {
                Some(pod) => pod.metadata.name.clone().ok_or_else(|| {
                    CoreError::PortForwarding(format!(
                        "Pod {} not found in namespace {}",
                        config.pod, config.namespace
                    ))
                })?,
                None => {
                    return Err(CoreError::PortForwarding(format!(
                        "Pod {} not found in namespace {} (and no pods starting with this name)",
                        config.pod, config.namespace
                    )));
                }
            }
        };

        // Update config with the actual pod name if it was different
        if config.pod != actual_pod_name {
            config.pod = actual_pod_name.clone();
        }

        // Calculate forward_id after resolving the pod name
        let forward_id = format!(
            "{}-{}-{}",
            config.instance_id, config.pod, config.local_port
        );

        let mut forwards = self.active_forwards.write().await;
        if forwards.contains_key(&forward_id) {
            return Err(CoreError::PortForwarding(format!(
                "Port forward already exists: {forward_id}"
            )));
        }

        let state = PortForwardingState {
            id: forward_id.clone(),
            config: config.clone(),
            status: PortForwardingStatus::Connecting,
            last_health_check: None,
            retry_count: 0,
        };
        forwards.insert(forward_id.clone(), state);

        drop(forwards);

        // Spawn the forward task
        spawn_forward_task(
            self.client.clone(),
            Arc::clone(&self.active_forwards),
            Arc::clone(&self.forward_tasks),
            forward_id.clone(),
            config,
            self.reconnect_delay,
            self.max_retries,
        )
        .await?;

        // Set status to Active immediately after spawning - the task is running
        // It will handle its own errors and update status if needed
        {
            let mut forwards = self.active_forwards.write().await;
            if let Some(state) = forwards.get_mut(&forward_id) {
                state.status = PortForwardingStatus::Active;
            }
        }

        Ok(forward_id)
    }

    /// Stop a port forward
    ///
    /// # Errors
    /// Returns an error if the forward is not found
    pub async fn stop_forward(&self, forward_id: &str) -> Result<(), CoreError> {
        let mut forwards = self.active_forwards.write().await;
        if !forwards.contains_key(forward_id) {
            return Err(CoreError::PortForwardingNotFound(forward_id.to_string()));
        }
        forwards.remove(forward_id);
        drop(forwards);

        let mut tasks = self.forward_tasks.write().await;
        if let Some((handle, shutdown_tx)) = tasks.remove(forward_id) {
            drop(shutdown_tx);
            handle.abort();
        }

        Ok(())
    }

    pub async fn list_forwards(&self) -> Vec<PortForwardingState> {
        let forwards = self.active_forwards.read().await;
        forwards.values().cloned().collect()
    }

    pub async fn get_forward(&self, forward_id: &str) -> Option<PortForwardingState> {
        let forwards = self.active_forwards.read().await;
        forwards.get(forward_id).cloned()
    }

    pub async fn list_forwards_by_instance(&self, instance_id: &str) -> Vec<PortForwardingState> {
        let forwards = self.active_forwards.read().await;
        forwards
            .values()
            .filter(|state| state.config.instance_id == instance_id)
            .cloned()
            .collect()
    }

    /// Check if a port is available
    ///
    /// # Errors
    /// Returns an error if the port is already in use
    pub fn check_port_available(&self, local_port: u16) -> Result<(), CoreError> {
        TcpListener::bind(("127.0.0.1", local_port))
            .map_err(|_| CoreError::PortConflict(local_port))?;
        Ok(())
    }

    /// Find an available port starting from the given port
    ///
    /// # Errors
    /// Returns an error if no available ports are found
    pub fn find_available_port(&self, start_port: u16) -> Result<u16, CoreError> {
        for port in start_port..=65535 {
            if TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return Ok(port);
            }
        }
        Err(CoreError::PortForwarding(
            "No available ports found".to_string(),
        ))
    }

    /// Reconnect a failed port forward
    ///
    /// # Errors
    /// Returns an error if the forward is not found or max retries exceeded
    pub async fn reconnect_forward(&self, forward_id: &str) -> Result<(), CoreError> {
        let state = {
            let forwards = self.active_forwards.read().await;
            forwards.get(forward_id).cloned()
        };

        let state = state.ok_or_else(|| {
            CoreError::PortForwardingNotFound(format!("Forward not found: {forward_id}"))
        })?;

        if state.retry_count >= self.max_retries {
            let mut forwards = self.active_forwards.write().await;
            if let Some(s) = forwards.get_mut(forward_id) {
                s.status = PortForwardingStatus::Failed;
            }
            return Err(CoreError::PortForwarding(format!(
                "Max retries exceeded for {forward_id}"
            )));
        }

        {
            let mut forwards = self.active_forwards.write().await;
            if let Some(s) = forwards.get_mut(forward_id) {
                s.status = PortForwardingStatus::Reconnecting;
                s.retry_count += 1;
            }
        }

        tokio::time::sleep(self.reconnect_delay * state.retry_count).await;

        let mut tasks = self.forward_tasks.write().await;
        if let Some((handle, shutdown_tx)) = tasks.remove(forward_id) {
            drop(shutdown_tx);
            handle.abort();
        }
        drop(tasks);

        spawn_forward_task(
            self.client.clone(),
            Arc::clone(&self.active_forwards),
            Arc::clone(&self.forward_tasks),
            forward_id.to_string(),
            state.config,
            self.reconnect_delay,
            self.max_retries,
        )
        .await?;

        Ok(())
    }

    pub fn start_health_monitoring(&self) {
        let forwards = Arc::clone(&self.active_forwards);
        let interval = self.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;

                let forward_ids: Vec<String> = {
                    let f = forwards.read().await;
                    f.keys().cloned().collect()
                };

                for forward_id in forward_ids {
                    let is_healthy = health::health_check_forward(&forwards, &forward_id).await;

                    let mut f = forwards.write().await;
                    if let Some(state) = f.get_mut(&forward_id) {
                        state.last_health_check = Some(SystemTime::now());
                        if !is_healthy && state.status == PortForwardingStatus::Active {
                            state.status = PortForwardingStatus::Failed;
                        } else if is_healthy && state.status == PortForwardingStatus::Active {
                            state.retry_count = 0;
                        }
                    }
                }
            }
        });
    }
}
