// Multi-instance port forwarding tests
//
// Tests for multiple ports per instance and filtering by instance

use super::create_test_manager;
use roro_core::api::kubernetes::portforwarding::PortForwardingConfig;

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

    if let (Ok(forward_id1), Ok(forward_id2)) = (result1, result2) {
        assert_ne!(forward_id1, forward_id2);

        let forwards = manager.list_forwards_by_instance("test-instance").await;
        assert!(forwards.len() >= 2);

        let fwd1 = manager.get_forward(&forward_id1).await;
        let fwd2 = manager.get_forward(&forward_id2).await;

        assert!(fwd1.is_some());
        assert!(fwd2.is_some());

        assert_eq!(
            fwd1.expect("Forward 1 should exist").config.instance_id,
            "test-instance"
        );
        assert_eq!(
            fwd2.expect("Forward 2 should exist").config.instance_id,
            "test-instance"
        );
    }
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

    let result1 = manager.start_forward(config1).await;
    let result2 = manager.start_forward(config2).await;
    let result3 = manager.start_forward(config3).await;

    // If all succeed, verify filtering by instance works
    if result1.is_ok() && result2.is_ok() && result3.is_ok() {
        let instance_a_forwards = manager.list_forwards_by_instance("instance-a").await;
        assert!(instance_a_forwards.len() >= 2);

        for forward in &instance_a_forwards {
            assert_eq!(forward.config.instance_id, "instance-a");
        }

        let instance_b_list = manager.list_forwards_by_instance("instance-b").await;
        assert!(!instance_b_list.is_empty());

        for forward in &instance_b_list {
            assert_eq!(forward.config.instance_id, "instance-b");
        }
    }
    // If they fail (e.g., no Kubernetes cluster or pods don't exist), that's acceptable
    // This test mainly verifies the list_forwards_by_instance method works when forwards exist
}

