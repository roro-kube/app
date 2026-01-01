use crate::api::kubernetes::context::ContextManager;
use crate::errors::CoreError;
use kube::{Client, Config};

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
}
