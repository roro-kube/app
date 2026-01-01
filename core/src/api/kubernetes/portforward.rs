use crate::api::kubernetes::client::KubernetesClient;
use crate::errors::CoreError;
use kube::Client;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

type ForwardTaskHandle = (JoinHandle<()>, mpsc::Sender<()>);
type ForwardTaskMap = Arc<RwLock<HashMap<String, ForwardTaskHandle>>>;

#[derive(Debug, Clone)]
pub struct PortForwardConfig {
    pub namespace: String,
    pub pod: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortForwardStatus {
    Connecting,
    Active,
    Failed,
    Reconnecting,
}

#[derive(Debug, Clone)]
pub struct PortForwardState {
    pub id: String,
    pub config: PortForwardConfig,
    pub status: PortForwardStatus,
    pub last_health_check: Option<SystemTime>,
    pub retry_count: u32,
}

pub struct PortForwardManager {
    active_forwards: Arc<RwLock<HashMap<String, PortForwardState>>>,
    client: Client,
    health_check_interval: Duration,
    reconnect_delay: Duration,
    max_retries: u32,
    forward_tasks: ForwardTaskMap,
}

impl PortForwardManager {
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
    /// Returns an error if the port is already in use or if the forward already exists
    pub async fn start_forward(&self, config: PortForwardConfig) -> Result<String, CoreError> {
        self.check_port_available(config.local_port)?;

        let forward_id = format!(
            "{}-{}-{}",
            config.instance_id, config.pod, config.local_port
        );

        let mut forwards = self.active_forwards.write().await;
        if forwards.contains_key(&forward_id) {
            return Err(CoreError::PortForward(format!(
                "Port forward already exists: {forward_id}"
            )));
        }

        let state = PortForwardState {
            id: forward_id.clone(),
            config: config.clone(),
            status: PortForwardStatus::Connecting,
            last_health_check: None,
            retry_count: 0,
        };
        forwards.insert(forward_id.clone(), state);

        drop(forwards);

        self.spawn_forward_task(forward_id.clone(), config).await?;

        Ok(forward_id)
    }

    /// Stop a port forward
    ///
    /// # Errors
    /// Returns an error if the forward is not found
    pub async fn stop_forward(&self, forward_id: &str) -> Result<(), CoreError> {
        let mut forwards = self.active_forwards.write().await;
        if !forwards.contains_key(forward_id) {
            return Err(CoreError::PortForwardNotFound(forward_id.to_string()));
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

    pub async fn list_forwards(&self) -> Vec<PortForwardState> {
        let forwards = self.active_forwards.read().await;
        forwards.values().cloned().collect()
    }

    pub async fn get_forward(&self, forward_id: &str) -> Option<PortForwardState> {
        let forwards = self.active_forwards.read().await;
        forwards.get(forward_id).cloned()
    }

    pub async fn list_forwards_by_instance(&self, instance_id: &str) -> Vec<PortForwardState> {
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
        Err(CoreError::PortForward(
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
            CoreError::PortForwardNotFound(format!("Forward not found: {forward_id}"))
        })?;

        if state.retry_count >= self.max_retries {
            let mut forwards = self.active_forwards.write().await;
            if let Some(s) = forwards.get_mut(forward_id) {
                s.status = PortForwardStatus::Failed;
            }
            return Err(CoreError::PortForward(format!(
                "Max retries exceeded for {forward_id}"
            )));
        }

        {
            let mut forwards = self.active_forwards.write().await;
            if let Some(s) = forwards.get_mut(forward_id) {
                s.status = PortForwardStatus::Reconnecting;
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
                        if !is_healthy && state.status == PortForwardStatus::Active {
                            state.status = PortForwardStatus::Failed;
                        } else if is_healthy && state.status == PortForwardStatus::Active {
                            state.retry_count = 0;
                        }
                    }
                }
            }
        });
    }

    async fn spawn_forward_task(
        &self,
        forward_id: String,
        config: PortForwardConfig,
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
            let _ = client;
            let _ = &config.namespace;

            let portforward_result: Result<(), CoreError> = Err(CoreError::PortForward(
                "Port forwarding API not yet implemented".to_string(),
            ));

            if portforward_result.is_ok() {
                {
                    let mut f = forwards.write().await;
                    if let Some(state) = f.get_mut(&forward_id_clone) {
                        state.status = PortForwardStatus::Active;
                    }
                }

                tokio::select! {
                    _msg = shutdown_rx.recv() => {
                        return;
                    }
                    () = tokio::time::sleep(Duration::from_secs(3600)) => {}
                }

                let mut f = forwards.write().await;
                if let Some(state) = f.get_mut(&forward_id_clone) {
                    if state.status == PortForwardStatus::Active {
                        state.status = PortForwardStatus::Failed;
                    }
                }
            } else {
                let retry_count = {
                    let mut f = forwards.write().await;
                    if let Some(state) = f.get_mut(&forward_id_clone) {
                        state.status = PortForwardStatus::Failed;
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
            }
        });

        let mut task_map = self.forward_tasks.write().await;
        task_map.insert(forward_id, (handle, shutdown_tx));

        Ok(())
    }

    async fn health_check_forward_internal(
        forwards: &Arc<RwLock<HashMap<String, PortForwardState>>>,
        forward_id: &str,
    ) -> bool {
        let state = {
            let f = forwards.read().await;
            f.get(forward_id).cloned()
        };

        if let Some(state) = state {
            if state.status != PortForwardStatus::Active {
                return false;
            }

            if TcpStream::connect(format!("127.0.0.1:{}", state.config.local_port)).is_ok() {
                return true;
            }
        }

        false
    }

    async fn reconnect_forward_internal(
        forwards: &Arc<RwLock<HashMap<String, PortForwardState>>>,
        tasks: &ForwardTaskMap,
        forward_id: &str,
        _config: &PortForwardConfig,
        reconnect_delay: Duration,
        max_retries: u32,
    ) -> Result<(), CoreError> {
        let state = {
            let f = forwards.read().await;
            f.get(forward_id).cloned()
        };

        let state = state.ok_or_else(|| {
            CoreError::PortForwardNotFound(format!("Forward not found: {forward_id}"))
        })?;

        if state.retry_count >= max_retries {
            let mut f = forwards.write().await;
            if let Some(s) = f.get_mut(forward_id) {
                s.status = PortForwardStatus::Failed;
            }
            return Err(CoreError::PortForward(format!(
                "Max retries exceeded for {forward_id}"
            )));
        }

        {
            let mut f = forwards.write().await;
            if let Some(s) = f.get_mut(forward_id) {
                s.status = PortForwardStatus::Reconnecting;
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
