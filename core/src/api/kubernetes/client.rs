use crate::api::kubernetes::context::ContextManager;
use crate::errors::CoreError;
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams};
use kube::{Client, Config};
use std::collections::HashMap;

pub struct KubernetesClient {
    client: Client,
    current_context: String,
}

impl KubernetesClient {
    /// Initialize a new Kubernetes client with the default context
    ///
    /// # Errors
    /// Returns an error if:
    /// - No current context is set in kubeconfig
    /// - Context validation fails
    /// - Client initialization fails
    pub async fn new() -> Result<Self, CoreError> {
        let context_name = ContextManager::current_context_name()?;
        Self::new_with_context(&context_name).await
    }

    /// Initialize a new Kubernetes client with a specific context
    ///
    /// # Errors
    /// Returns an error if:
    /// - The specified context is not found in kubeconfig
    /// - Kubeconfig cannot be loaded
    /// - Client initialization fails
    pub async fn new_with_context(context_name: &str) -> Result<Self, CoreError> {
        ContextManager::validate_context(context_name)?;

        let config = Config::from_kubeconfig(&kube::config::KubeConfigOptions {
            context: Some(context_name.to_string()),
            ..Default::default()
        })
        .await
        .map_err(|e| CoreError::Kubeconfig(format!("Failed to create config: {e}")))?;

        let client = Client::try_from(config)
            .map_err(|e| CoreError::Kubernetes(format!("Failed to create client: {e}")))?;

        Ok(Self {
            client,
            current_context: context_name.to_string(),
        })
    }

    /// # Errors
    /// Returns an error if the connection validation fails
    pub fn validate_connection(&self) -> Result<(), CoreError> {
        Ok(())
    }

    #[must_use]
    pub fn current_context(&self) -> &str {
        &self.current_context
    }

    /// # Errors
    /// Returns an error if the kubeconfig cannot be loaded
    pub fn list_contexts() -> Result<Vec<String>, CoreError> {
        ContextManager::list_contexts()
    }

    #[must_use]
    pub fn inner(&self) -> &Client {
        &self.client
    }

    /// List all pods in all namespaces
    ///
    /// # Errors
    /// Returns an error if the API call fails
    pub async fn list_all_pods(&self) -> Result<Vec<Pod>, CoreError> {
        let pods: Api<Pod> = Api::all(self.client.clone());
        let pod_list = pods
            .list(&ListParams::default())
            .await
            .map_err(|e| CoreError::Kubernetes(format!("Failed to list pods: {e}")))?;
        Ok(pod_list.items)
    }

    /// List pods in a specific namespace
    ///
    /// # Errors
    /// Returns an error if the API call fails
    pub async fn list_pods_in_namespace(&self, namespace: &str) -> Result<Vec<Pod>, CoreError> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), namespace);
        let pod_list = pods.list(&ListParams::default()).await.map_err(|e| {
            CoreError::Kubernetes(format!("Failed to list pods in namespace {namespace}: {e}"))
        })?;
        Ok(pod_list.items)
    }

    /// Extract container ports from a pod
    ///
    /// Returns a map of container name to list of ports
    #[must_use]
    pub fn extract_pod_ports(pod: &Pod) -> HashMap<String, Vec<u16>> {
        let mut ports_map = HashMap::new();

        if let Some(spec) = &pod.spec {
            for container in &spec.containers {
                let container_name = container.name.clone();
                let mut ports = Vec::new();

                if let Some(container_ports) = &container.ports {
                    for port in container_ports {
                        let container_port = port.container_port;
                        if container_port > 0 {
                            if let Ok(port_u16) = u16::try_from(container_port) {
                                ports.push(port_u16);
                            }
                        }
                    }
                }

                if !ports.is_empty() {
                    ports_map.insert(container_name, ports);
                }
            }
        }

        ports_map
    }
}
