use roro_core::api::kubernetes::{
    portforwarding::{PortForwardingConfig, PortForwardingManager},
    KubernetesClient,
};
use roro_core::errors::CoreError;
use std::time::Duration;

async fn create_test_manager() -> PortForwardingManager {
    let client = KubernetesClient::new().await;
    match client {
        Ok(c) => PortForwardingManager::new(&c),
        Err(_) => {
            panic!("Failed to create Kubernetes client for tests");
        }
    }
}

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
            let state = forward.unwrap();
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
async fn test_port_conflict_detection() {
    let manager = create_test_manager().await;

    let listener = std::net::TcpListener::bind("127.0.0.1:9100").unwrap();

    let result = manager.check_port_available(9100);
    assert!(result.is_err());
    match result {
        Err(CoreError::PortConflict(9100)) => {}
        _ => panic!("Expected PortConflict error"),
    }

    drop(listener);

    let result = manager.check_port_available(9100);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_find_available_port() {
    let manager = create_test_manager().await;

    let listener = std::net::TcpListener::bind("127.0.0.1:9200").unwrap();

    let result = manager.find_available_port(9200);
    assert!(result.is_ok());
    let port = result.unwrap();
    assert!(port > 9200);

    drop(listener);
}

#[tokio::test]
async fn test_multiple_ports_per_instance() {
    let manager = create_test_manager().await;

    let config1 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-1".to_string(),
        remote_port: 8080,
        local_port: 9300,
        instance_id: "test-instance".to_string(),
    };

    let config2 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-2".to_string(),
        remote_port: 8081,
        local_port: 9301,
        instance_id: "test-instance".to_string(),
    };

    let result1 = manager.start_forward(config1).await;
    let result2 = manager.start_forward(config2).await;

    if result1.is_ok() && result2.is_ok() {
        let forward_id1 = result1.unwrap();
        let forward_id2 = result2.unwrap();

        assert_ne!(forward_id1, forward_id2);

        let forwards = manager.list_forwards_by_instance("test-instance").await;
        assert!(forwards.len() >= 2);

        let fwd1 = manager.get_forward(&forward_id1).await;
        let fwd2 = manager.get_forward(&forward_id2).await;

        assert!(fwd1.is_some());
        assert!(fwd2.is_some());

        assert_eq!(fwd1.unwrap().config.instance_id, "test-instance");
        assert_eq!(fwd2.unwrap().config.instance_id, "test-instance");
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

    let _ = manager.start_forward(config1).await;
    let _ = manager.start_forward(config2).await;

    let forwards = manager.list_forwards().await;
    assert!(forwards.len() >= 2);
}

#[tokio::test]
async fn test_list_forwards_by_instance() {
    let manager = create_test_manager().await;

    let config1 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-1".to_string(),
        remote_port: 8080,
        local_port: 9600,
        instance_id: "instance-a".to_string(),
    };

    let config2 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-2".to_string(),
        remote_port: 8081,
        local_port: 9601,
        instance_id: "instance-a".to_string(),
    };

    let config3 = PortForwardingConfig {
        namespace: "default".to_string(),
        pod: "test-pod-3".to_string(),
        remote_port: 8082,
        local_port: 9602,
        instance_id: "instance-b".to_string(),
    };

    let _ = manager.start_forward(config1).await;
    let _ = manager.start_forward(config2).await;
    let _ = manager.start_forward(config3).await;

    let instance_a_forwards = manager.list_forwards_by_instance("instance-a").await;
    assert!(instance_a_forwards.len() >= 2);

    for forward in &instance_a_forwards {
        assert_eq!(forward.config.instance_id, "instance-a");
    }

    let instance_b_list = manager.list_forwards_by_instance("instance-b").await;
    assert!(instance_b_list.len() >= 1);

    for forward in &instance_b_list {
        assert_eq!(forward.config.instance_id, "instance-b");
    }
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

        let state = forward.unwrap();
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
            Ok(_) => {}
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
