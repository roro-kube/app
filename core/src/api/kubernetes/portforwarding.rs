use crate::api::kubernetes::client::KubernetesClient;
use crate::errors::CoreError;
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams};
use kube::Client;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::io;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

type ForwardTaskHandle = (JoinHandle<()>, mpsc::Sender<()>);
type ForwardTaskMap = Arc<RwLock<HashMap<String, ForwardTaskHandle>>>;

#[derive(Debug, Clone)]
pub struct PortForwardingConfig {
    pub namespace: String,
    pub pod: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortForwardingStatus {
    Connecting,
    Active,
    Failed,
    Reconnecting,
}

#[derive(Debug, Clone)]
pub struct PortForwardingState {
    pub id: String,
    pub config: PortForwardingConfig,
    pub status: PortForwardingStatus,
    pub last_health_check: Option<SystemTime>,
    pub retry_count: u32,
}

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
        self.spawn_forward_task(forward_id.clone(), config).await?;

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

        self.spawn_forward_task(forward_id.to_string(), state.config)
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
                    let is_healthy =
                        Self::health_check_forward_internal(&forwards, &forward_id).await;

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

    #[allow(clippy::too_many_lines)]
    async fn spawn_forward_task(
        &self,
        forward_id: String,
        config: PortForwardingConfig,
    ) -> Result<(), CoreError> {
        let client = self.client.clone();
        let forwards = Arc::clone(&self.active_forwards);
        let _tasks = Arc::clone(&self.forward_tasks);
        let manager_forwards = Arc::clone(&self.active_forwards);
        let manager_tasks = Arc::clone(&self.forward_tasks);
        let reconnect_delay = self.reconnect_delay;
        let max_retries = self.max_retries;

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
                        let _ = Self::reconnect_forward_internal(
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

        let mut task_map = self.forward_tasks.write().await;
        task_map.insert(forward_id, (handle, shutdown_tx));

        Ok(())
    }

    async fn health_check_forward_internal(
        forwards: &Arc<RwLock<HashMap<String, PortForwardingState>>>,
        forward_id: &str,
    ) -> bool {
        let state = {
            let f = forwards.read().await;
            f.get(forward_id).cloned()
        };

        if let Some(state) = state {
            if state.status != PortForwardingStatus::Active {
                return false;
            }

            if TcpStream::connect(format!("127.0.0.1:{}", state.config.local_port)).is_ok() {
                return true;
            }
        }

        false
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
}
