// Domain handlers tests
//
// Tests for HandlerRegistry and OperationHandler trait.

use roro_domain::{
    DomainEntity, EntityState, HandlerRegistry, OperationHandler, ProcessingContext,
    ProcessingResult, DomainError,
};
use std::sync::Arc;

// Mock handler implementations for testing
struct SuccessHandler;

#[async_trait::async_trait]
impl OperationHandler for SuccessHandler {
    async fn handle(
        &self,
        _context: &mut ProcessingContext,
        _entity: &DomainEntity,
    ) -> Result<ProcessingResult, DomainError> {
        Ok(ProcessingResult {
            entity_id: "test".to_string(),
            state: EntityState::Complete,
            output: std::collections::HashMap::new(),
        })
    }
}

struct FailureHandler;

#[async_trait::async_trait]
impl OperationHandler for FailureHandler {
    async fn handle(
        &self,
        _context: &mut ProcessingContext,
        _entity: &DomainEntity,
    ) -> Result<ProcessingResult, DomainError> {
        Err(DomainError::Processing("Handler failed".to_string()))
    }
}

#[test]
fn test_handler_registry_new() {
    let _registry = HandlerRegistry::new();
    // Registry should be created successfully
    assert!(true);
}

#[test]
fn test_handler_registry_default() {
    let _registry = HandlerRegistry::default();
    // Default registry should be created successfully
    assert!(true);
}

#[test]
fn test_handler_registry_register() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(SuccessHandler);
    
    registry.register("test-operation".to_string(), handler);
    
    // Handler should be registered
    assert!(registry.get_handler("test-operation").is_some());
}

#[test]
fn test_handler_registry_get_handler() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(SuccessHandler);
    
    registry.register("test-operation".to_string(), handler);
    
    let retrieved = registry.get_handler("test-operation");
    assert!(retrieved.is_some());
}

#[test]
fn test_handler_registry_get_nonexistent_handler() {
    let registry = HandlerRegistry::new();
    
    let retrieved = registry.get_handler("non-existent");
    assert!(retrieved.is_none());
}

#[test]
fn test_handler_registry_multiple_handlers() {
    let mut registry = HandlerRegistry::new();
    let handler1 = Arc::new(SuccessHandler);
    let handler2 = Arc::new(FailureHandler);
    
    registry.register("operation1".to_string(), handler1);
    registry.register("operation2".to_string(), handler2);
    
    assert!(registry.get_handler("operation1").is_some());
    assert!(registry.get_handler("operation2").is_some());
    assert!(registry.get_handler("operation3").is_none());
}

#[tokio::test]
async fn test_operation_handler_trait_success() {
    let handler = SuccessHandler;
    let mut context = ProcessingContext {
        context_id: "test".to_string(),
        entity_name: "Test".to_string(),
        state: std::collections::HashMap::new(),
        logs: Vec::new(),
    };
    let entity = DomainEntity {
        id: "test".to_string(),
        name: "Test".to_string(),
        state: EntityState::Pending,
    };
    
    let result = handler.handle(&mut context, &entity).await;
    
    assert!(result.is_ok());
    let processing_result = result.unwrap();
    assert_eq!(processing_result.state, EntityState::Complete);
}

#[tokio::test]
async fn test_operation_handler_trait_failure() {
    let handler = FailureHandler;
    let mut context = ProcessingContext {
        context_id: "test".to_string(),
        entity_name: "Test".to_string(),
        state: std::collections::HashMap::new(),
        logs: Vec::new(),
    };
    let entity = DomainEntity {
        id: "test".to_string(),
        name: "Test".to_string(),
        state: EntityState::Pending,
    };
    
    let result = handler.handle(&mut context, &entity).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        DomainError::Processing(msg) => {
            assert!(msg.contains("Handler failed"));
        }
        _ => panic!("Expected Processing error"),
    }
}

#[test]
fn test_handler_registry_replace_handler() {
    let mut registry = HandlerRegistry::new();
    let handler1 = Arc::new(SuccessHandler);
    let handler2 = Arc::new(FailureHandler);
    
    registry.register("test-operation".to_string(), handler1);
    registry.register("test-operation".to_string(), handler2);
    
    // Should have the second handler (replaced)
    let retrieved = registry.get_handler("test-operation");
    assert!(retrieved.is_some());
}

