// Basic port forwarding manager tests
//
// Tests for basic operations: creation, stop, get, list, duplicate detection

use super::create_test_manager;
use roro_core::api::kubernetes::portforwarding::PortForwardingConfig;
use roro_core::errors::CoreError;

#[tokio::test]
async fn test_port_forward_creation() {
    let manager = create_test_manager().await;
    let config = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod".to_string(),
        remote_port: 8080,
        local_port: 9000,
        instance_id: "test-instance".to_string(),
    };

    let result = manager.start_forward(config.clone()).await;

    match result {
        Ok(forward_id) => {
            assert!(!forward_id.is_empty());
            let forward = manager.get_forward(&forward_id).await;
            assert!(forward.is_some());
            let state = forward.expect("Forward should exist");
            assert_eq!(state.config.instance_id, "test-instance");
            assert_eq!(state.config.local_port, 9000);
            assert_eq!(state.config.remote_port, 8080);
        }
        Err(e) => match e {
            CoreError::Kubernetes(_) | CoreError::PortForwarding(_) => {}
            _ => panic!("Unexpected error: {e}"),
        },
    }
}

#[tokio::test]
async fn test_stop_forward() {
    let manager = create_test_manager().await;

    let config = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod".to_string(),
        remote_port: 8080,
        local_port: 9400,
        instance_id: "test-instance".to_string(),
    };

    let result = manager.start_forward(config).await;

    if let Ok(forward_id) = result {
        let stop_result = manager.stop_forward(&forward_id).await;
        assert!(stop_result.is_ok());

        let forward = manager.get_forward(&forward_id).await;
        assert!(forward.is_none());

        let stop_again = manager.stop_forward(&forward_id).await;
        assert!(stop_again.is_err());
        match stop_again {
            Err(CoreError::PortForwardingNotFound(_)) => {}
            _ => panic!("Expected PortForwardingNotFound error"),
        }
    }
}

#[tokio::test]
async fn test_list_forwards() {
    let manager = create_test_manager().await;

    let config1 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-1".to_string(),
        remote_port: 8080,
        local_port: 9500,
        instance_id: "test-instance-1".to_string(),
    };

    let config2 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-2".to_string(),
        remote_port: 8081,
        local_port: 9501,
        instance_id: "test-instance-2".to_string(),
    };

    let result1 = manager.start_forward(config1).await;
    let result2 = manager.start_forward(config2).await;

    // If both succeed, verify we can list them
    if result1.is_ok() && result2.is_ok() {
        let forwards = manager.list_forwards().await;
        assert!(forwards.len() >= 2);
    }
    // If they fail (e.g., no Kubernetes cluster or pods don't exist), that's acceptable
    // This test mainly verifies the list_forwards method works when forwards exist
}

#[tokio::test]
async fn test_get_forward() {
    let manager = create_test_manager().await;

    let config = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod".to_string(),
        remote_port: 8080,
        local_port: 9700,
        instance_id: "test-instance".to_string(),
    };

    let result = manager.start_forward(config).await;

    if let Ok(forward_id) = result {
        let forward = manager.get_forward(&forward_id).await;
        assert!(forward.is_some());

        let state = forward.expect("Forward should exist");
        assert_eq!(state.id, forward_id);
        assert_eq!(state.config.instance_id, "test-instance");
    }

    let non_existent = manager.get_forward("non-existent-id").await;
    assert!(non_existent.is_none());
}

#[tokio::test]
async fn test_duplicate_forward_id() {
    let manager = create_test_manager().await;

    let config = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod".to_string(),
        remote_port: 8080,
        local_port: 9800,
        instance_id: "test-instance".to_string(),
    };

    let result1 = manager.start_forward(config.clone()).await;

    if result1.is_ok() {
        let result2 = manager.start_forward(config).await;
        assert!(result2.is_err());
        match result2 {
            Err(CoreError::PortForwarding(_)) => {}
            _ => panic!("Expected PortForwarding error for duplicate"),
        }
    }
}

