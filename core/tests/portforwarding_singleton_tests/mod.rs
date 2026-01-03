// Port forwarding singleton tests
//
// Tests for the port forwarding manager singleton initialization, retrieval, and state management.

mod initialization;
mod retrieval;

use roro_core::api::kubernetes::KubernetesClient;
use roro_core::errors::CoreError;

/// Create a test client for use in tests
pub async fn create_test_client() -> Result<KubernetesClient, CoreError> {
    KubernetesClient::new().await
}

