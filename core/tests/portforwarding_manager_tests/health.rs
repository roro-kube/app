// Health monitoring and reconnection tests
//
// Tests for health monitoring and reconnection functionality

use super::create_test_manager;
use roro_core::api::kubernetes::portforwarding::PortForwardingConfig;
use roro_core::api::kubernetes::KubernetesClient;
use roro_core::errors::CoreError;
use roro_core::api::kubernetes::portforwarding::PortForwardingManager;
use std::time::Duration;

#[tokio::test]
async fn test_health_monitoring_start() {
    let manager = create_test_manager().await;

    manager.start_health_monitoring();

    tokio::time::sleep(Duration::from_millis(100)).await;
}

#[tokio::test]
async fn test_reconnect_forward() {
    let manager = create_test_manager().await;

    let config = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod".to_string(),
        remote_port: 8080,
        local_port: 9900,
        instance_id: "test-instance".to_string(),
    };

    let result = manager.start_forward(config).await;

    if let Ok(forward_id) = result {
        let reconnect_result = manager.reconnect_forward(&forward_id).await;

        match reconnect_result {
            Ok(()) => {}
            Err(e) => match e {
                CoreError::PortForwarding(_) | CoreError::Kubernetes(_) => {}
                _ => panic!("Unexpected error: {e}"),
            },
        }
    }
}

#[tokio::test]
async fn test_reconnect_non_existent_forward() {
    let manager = create_test_manager().await;

    let result = manager.reconnect_forward("non-existent-id").await;
    assert!(result.is_err());
    match result {
        Err(CoreError::PortForwardingNotFound(_)) => {}
        _ => panic!("Expected PortForwardingNotFound error"),
    }
}

#[tokio::test]
async fn test_manager_configuration() {
    let client = KubernetesClient::new().await;
    if let Ok(c) = client {
        let manager = PortForwardingManager::new(&c)
            .with_health_check_interval(Duration::from_secs(60))
            .with_reconnect_delay(Duration::from_secs(10))
            .with_max_retries(10);

        let config = PortForwardingConfig {
            namespace: "default".to_string(),
            pod: "test-pod".to_string(),
            remote_port: 8080,
            local_port: 9950,
            instance_id: "test-instance".to_string(),
        };

        let _ = manager.start_forward(config).await;
    }
}

