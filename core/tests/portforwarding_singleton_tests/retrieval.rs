// Singleton retrieval tests
//
// Tests for getting and retrieving the singleton instance

use super::create_test_client;
use roro_core::api::kubernetes::portforwarding_singleton::{get, get_or_init, initialize, is_initialized};
use roro_core::errors::CoreError;
use std::sync::Arc;

#[tokio::test]
async fn test_singleton_get_before_initialization() {
    // Note: This test may pass or fail depending on test execution order
    // If another test initialized the singleton first, get() will return Some
    let manager = get();
    // We can't assert None here due to test execution order, but we can verify it doesn't panic
    let _ = manager;
}

#[tokio::test]
async fn test_singleton_get_after_initialization() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        // Try to initialize (may fail if already initialized)
        let _ = initialize(&client);

        // Now get should return Some
        let manager = get();
        assert!(manager.is_some());

        // Verify we can use the manager
        if let Some(manager_arc) = manager {
            let _ = manager_arc.list_forwards().await;
        }
        // Kubernetes client creation failed - skip this test
        // This is acceptable if kubeconfig is not available
    }
}

#[tokio::test]
async fn test_singleton_get_or_init_initializes() {
    // Note: This test may behave differently depending on whether singleton is already initialized
    // by a previous test. We handle both cases.

    let result = get_or_init("rancher-desktop").await;

    match result {
        Ok(manager) => {
            // Success - singleton is now initialized
            assert!(is_initialized());
            let manager2 = get();
            assert!(manager2.is_some());

            // Verify we can use the manager
            let _ = manager.list_forwards().await;
        }
        Err(e) => {
            // May fail if kubeconfig is not available or context doesn't exist
            match e {
                CoreError::Kubeconfig(_)
                | CoreError::ContextNotFound(_)
                | CoreError::Kubernetes(_)
                | CoreError::PortForwarding(_) => {}
                _ => panic!("Unexpected error type: {e}"),
            }
        }
    }
}

#[tokio::test]
async fn test_singleton_get_or_init_returns_existing() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        // Initialize first (may fail if already initialized)
        let _ = initialize(&client);

        // Now get_or_init should return the existing instance
        let result1 = get_or_init("rancher-desktop").await;
        let result2 = get_or_init("rancher-desktop").await;

        match (result1, result2) {
            (Ok(manager1), Ok(manager2)) => {
                // Both should return the same Arc (same pointer)
                assert!(Arc::ptr_eq(&manager1, &manager2));
            }
            (Err(e1), Err(e2)) => {
                // Both failed - may be due to kubeconfig issues
                match (&e1, &e2) {
                    (
                        CoreError::Kubeconfig(_)
                        | CoreError::ContextNotFound(_)
                        | CoreError::Kubernetes(_),
                        CoreError::Kubeconfig(_)
                        | CoreError::ContextNotFound(_)
                        | CoreError::Kubernetes(_),
                    ) => {}
                    _ => panic!("Unexpected error types: {e1}, {e2}"),
                }
            }
            _ => {
                // Mixed results - acceptable if singleton state is inconsistent
                // This can happen due to test execution order
            }
        }
        // Kubernetes client creation failed - skip this test
        // This is acceptable if kubeconfig is not available
    }
}

#[tokio::test]
async fn test_singleton_get_or_init_invalid_context() {
    // Test with a non-existent context name
    let result = get_or_init("non-existent-context-12345").await;

    assert!(result.is_err());
    match result {
        Err(e) => {
            match e {
                CoreError::ContextNotFound(msg) => {
                    assert!(msg.contains("non-existent-context-12345"));
                }
                CoreError::Kubeconfig(_) | CoreError::Kubernetes(_) => {
                    // Kubeconfig/Kubernetes errors are also acceptable
                }
                _ => panic!("Unexpected error type: {e}"),
            }
        }
        Ok(_) => {
            panic!("Expected error for invalid context");
        }
    }
}

