// Kubernetes client tests
//
// Tests for KubernetesClient initialization, context selection, and connection validation.

use roro_core::api::kubernetes::{ContextManager, KubernetesClient};
use roro_core::errors::CoreError;

#[tokio::test]
async fn test_context_manager_list_contexts() {
    let result = ContextManager::list_contexts();

    match result {
        Ok(contexts) => {
            for context in contexts {
                assert!(!context.is_empty());
            }
        }
        Err(e) => {
            assert!(
                matches!(e, CoreError::Kubeconfig(_)),
                "Unexpected error type: {e}"
            );
        }
    }
}

#[tokio::test]
async fn test_context_manager_current_context() {
    let result = ContextManager::current_context_name();

    match result {
        Ok(context) => {
            assert!(!context.is_empty());
        }
        Err(e) => {
            assert!(
                matches!(e, CoreError::Kubeconfig(_)),
                "Unexpected error type: {e}"
            );
        }
    }
}

#[tokio::test]
async fn test_context_manager_validate_context_invalid() {
    let result = ContextManager::validate_context("non-existent-context-12345");

    if let Err(e) = result {
        match e {
            CoreError::ContextNotFound(msg) => {
                assert!(msg.contains("non-existent-context-12345"));
            }
            CoreError::Kubeconfig(_) => {}
            _ => panic!("Unexpected error type: {e}"),
        }
    }
}

#[tokio::test]
async fn test_kubernetes_client_list_contexts() {
    let result = KubernetesClient::list_contexts();

    match result {
        Ok(contexts) => {
            for context in contexts {
                assert!(!context.is_empty());
            }
        }
        Err(e) => {
            assert!(
                matches!(e, CoreError::Kubeconfig(_)),
                "Unexpected error type: {e}"
            );
        }
    }
}

#[tokio::test]
async fn test_kubernetes_client_new_with_invalid_context() {
    let result = KubernetesClient::new_with_context("non-existent-context-12345").await;

    if let Err(e) = result {
        match e {
            CoreError::ContextNotFound(msg) => {
                assert!(msg.contains("non-existent-context-12345"));
            }
            CoreError::Kubeconfig(_) | CoreError::Kubernetes(_) => {}
            _ => panic!("Unexpected error type: {e}"),
        }
    }
}

#[tokio::test]
async fn test_kubernetes_client_new_default() {
    let result = KubernetesClient::new().await;

    match result {
        Ok(client) => {
            let context = client.current_context();
            assert!(!context.is_empty());

            let validation_result = client.validate_connection();
            if let Err(e) = validation_result {
                match e {
                    CoreError::Connection(_) => {}
                    _ => panic!("Unexpected error type during connection validation: {e}"),
                }
            }
        }
        Err(e) => {
            assert!(
                matches!(
                    e,
                    CoreError::Kubeconfig(_)
                        | CoreError::ContextNotFound(_)
                        | CoreError::Kubernetes(_)
                ),
                "Unexpected error type: {e}"
            );
        }
    }
}

#[tokio::test]
async fn test_kubernetes_client_current_context() {
    let result = KubernetesClient::new().await;

    if let Ok(client) = result {
        let context = client.current_context();
        assert!(!context.is_empty());
    }
}

#[tokio::test]
async fn test_kubernetes_client_validate_connection() {
    let result = KubernetesClient::new().await;

    if let Ok(client) = result {
        let validation_result = client.validate_connection();
        assert!(validation_result.is_ok());
    }
}

#[tokio::test]
async fn test_kubernetes_client_inner() {
    let result = KubernetesClient::new().await;

    if let Ok(client) = result {
        let inner_client = client.inner();
        assert!(!client.current_context().is_empty());
        let _ = inner_client;
    }
}

#[test]
fn test_error_types() {
    let errors = vec![
        CoreError::Kubernetes("test kubernetes error".to_string()),
        CoreError::Kubeconfig("test kubeconfig error".to_string()),
        CoreError::Connection("test connection error".to_string()),
        CoreError::ContextNotFound("test-context".to_string()),
    ];

    for error in errors {
        let msg = format!("{error}");
        assert!(!msg.is_empty());
        match error {
            CoreError::Kubernetes(_) => assert!(msg.contains("Kubernetes error")),
            CoreError::Kubeconfig(_) => assert!(msg.contains("Kubeconfig error")),
            CoreError::Connection(_) => assert!(msg.contains("Connection error")),
            CoreError::ContextNotFound(_) => assert!(msg.contains("Context not found")),
            _ => panic!("Unexpected error variant"),
        }
    }
}
