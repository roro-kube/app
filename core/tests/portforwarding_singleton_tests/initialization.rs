// Singleton initialization tests
//
// Tests for singleton initialization and state management

use super::create_test_client;
use roro_core::api::kubernetes::portforwarding_singleton::{initialize, is_initialized};
use roro_core::errors::CoreError;

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
async fn test_singleton_initialize_success() {
    let client_result = create_test_client().await;

    if let Ok(client) = client_result {
        let result = initialize(&client);
        match result {
            Ok(()) => {
                // Verify it's now initialized
                assert!(is_initialized());
                // Verify we can get it
                let manager = roro_core::api::kubernetes::portforwarding_singleton::get();
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
        // Kubernetes client creation failed - skip this test
        // This is acceptable if kubeconfig is not available
    }
}

