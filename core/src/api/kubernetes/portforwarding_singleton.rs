// Port forwarding singleton
//
// Provides a global singleton instance of PortForwardingManager
// to ensure all operations use the same manager instance and share state.

use super::portforwarding::PortForwardingManager;
use crate::api::kubernetes::client::KubernetesClient;
use crate::errors::CoreError;
use std::sync::{Arc, OnceLock};

static PORT_FORWARDING_MANAGER: OnceLock<Arc<PortForwardingManager>> = OnceLock::new();

/// Initialize the port forwarding manager singleton
///
/// # Errors
/// Returns an error if:
/// - The manager has already been initialized
/// - Client initialization fails (though this should be handled by the caller)
pub fn initialize(client: &KubernetesClient) -> Result<(), CoreError> {
    let manager = PortForwardingManager::new(client);
    PORT_FORWARDING_MANAGER.set(Arc::new(manager)).map_err(|_| {
        CoreError::PortForwarding("Port forwarding manager already initialized".to_string())
    })
}

/// Get the port forwarding manager singleton instance
///
/// Returns `None` if the manager has not been initialized yet
#[must_use]
pub fn get() -> Option<Arc<PortForwardingManager>> {
    PORT_FORWARDING_MANAGER.get().cloned()
}

/// Get the port forwarding manager singleton, initializing it if necessary
///
/// # Errors
/// Returns an error if:
/// - Client initialization fails
/// - Manager initialization fails
pub async fn get_or_init(context_name: &str) -> Result<Arc<PortForwardingManager>, CoreError> {
    // Try to get existing instance first
    if let Some(manager) = get() {
        return Ok(manager);
    }

    // Initialize if not already initialized
    let client = KubernetesClient::new_with_context(context_name).await?;
    initialize(&client)?;

    get().ok_or_else(|| {
        CoreError::PortForwarding("Failed to retrieve manager after initialization".to_string())
    })
}

/// Check if the manager has been initialized
#[must_use]
pub fn is_initialized() -> bool {
    PORT_FORWARDING_MANAGER.get().is_some()
}
