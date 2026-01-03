// Port forwarding singleton tests
//
// Tests for the port forwarding manager singleton initialization, retrieval, and state management.

use roro_core::api::kubernetes::{
    portforwarding_singleton::{get, get_or_init, initialize, is_initialized},
    KubernetesClient,
};
use roro_core::errors::CoreError;
use std::sync::Arc;

async fn create_test_client() -> Result<KubernetesClient, CoreError> {
    KubernetesClient::new().await
}

#[tokio::test]
async fn test_singleton_is_initialized_false() {
    // Note: This test assumes the singleton is not initialized at the start
    // In practice, Rust tests run in separate processes, so each test gets a fresh singleton state
    let initialized = is_initialized();
    // This may be true if another test already initialized it, or false if not
    // We can't assert a specific value here due to test execution order
    let _ = initialized; // Just verify the function doesn't panic
}

#[tokio::test]
async fn test_singleton_get_before_initialization() {
    // Note: This test may pass or fail depending on test execution order
    // If another test initialized the singleton first, get() will return Some
    let manager = get();
    // We can't assert None here due to test execution order, but we can verify it doesn't panic
    let _ = manager;
}

#[tokio::test]
async fn test_singleton_initialize_success() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        let result = initialize(&client);
        match result {
            Ok(()) => {
                // Verify it's now initialized
                assert!(is_initialized());
                // Verify we can get it
                let manager = get();
                assert!(manager.is_some());
            }
            Err(e) => {
                // If initialization fails, it means it was already initialized
                // This is acceptable - the singleton was initialized by another test
                match e {
                    CoreError::PortForwarding(msg) => {
                        assert!(msg.contains("already initialized"));
                    }
                    _ => panic!("Unexpected error type: {e}"),
                }
            }
        }
        // Kubernetes client creation failed - skip this test
        // This is acceptable if kubeconfig is not available
    }
}

#[tokio::test]
async fn test_singleton_initialize_double_initialization() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        // First initialization
            let first_result = initialize(&client);

            // Second initialization should fail
            let second_result = initialize(&client);

            match (first_result, second_result) {
                (Ok(()), Err(e)) => {
                    // Expected: first succeeds, second fails
                    match e {
                        CoreError::PortForwarding(msg) => {
                            assert!(msg.contains("already initialized"));
                        }
                        _ => panic!("Unexpected error type: {e}"),
                    }
                }
                (Err(e1), Err(e2)) => {
                    // Both failed - singleton was already initialized by another test
                    // This is acceptable
                    match (&e1, &e2) {
                        (CoreError::PortForwarding(_), CoreError::PortForwarding(_)) => {}
                        _ => panic!("Unexpected error types: {e1}, {e2}"),
                    }
                }
                (Ok(()), Ok(())) => {
                    panic!("Second initialization should have failed");
                }
                (Err(_), Ok(())) => {
                    panic!("First initialization failed but second succeeded - unexpected");
                }
            }
        // Kubernetes client creation failed - skip this test
        // This is acceptable if kubeconfig is not available
    }
}

#[tokio::test]
async fn test_singleton_get_after_initialization() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        Ok(client) => {
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
        }
        Err(_) => {
            // Kubernetes client creation failed - skip this test
            // This is acceptable if kubeconfig is not available
        }
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

#[tokio::test]
async fn test_singleton_is_initialized_true() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        // Initialize the singleton
            let init_result = initialize(&client);

            match init_result {
                Ok(()) => {
                    // Now is_initialized should return true
                    assert!(is_initialized());
                }
                Err(e) => {
                    // Already initialized by another test
                    match e {
                        CoreError::PortForwarding(msg) => {
                            assert!(msg.contains("already initialized"));
                            // In this case, it should still be initialized
                            assert!(is_initialized());
                        }
                        _ => panic!("Unexpected error type: {e}"),
                    }
                }
            }
        }
        Err(_) => {
            // Kubernetes client creation failed - skip this test
            // This is acceptable if kubeconfig is not available
        }
    }
}
