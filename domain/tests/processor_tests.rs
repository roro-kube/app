// Domain processor tests
//
// Tests for DomainProcessor initialization and processing operations.

use roro_domain::{
    DomainEntity, DomainError, DomainProcessor, EntityState, HandlerRegistry, OperationHandler,
    ProcessingContext, ProcessingResult,
};
use std::sync::Arc;

// Mock handler for testing
struct MockHandler {
    should_succeed: bool,
}

#[async_trait::async_trait]
impl OperationHandler for MockHandler {
    async fn handle(
        &self,
        context: &mut ProcessingContext,
        _entity: &DomainEntity,
    ) -> Result<ProcessingResult, DomainError> {
        if self.should_succeed {
            context.logs.push("Processing completed".to_string());
            Ok(ProcessingResult {
                entity_id: context.context_id.clone(),
                state: EntityState::Complete,
                output: std::collections::HashMap::new(),
            })
        } else {
            Err(DomainError::Processing("Mock handler failed".to_string()))
        }
    }
}

#[tokio::test]
async fn test_domain_processor_new() {
    let registry = Arc::new(HandlerRegistry::new());
    let _processor = DomainProcessor::new(registry);

    // Processor should be created successfully
    // Just verify it doesn't panic
}

#[tokio::test]
async fn test_domain_processor_process_with_valid_handler() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(MockHandler {
        should_succeed: true,
    });
    registry.register("test-operation".to_string(), handler);

    let processor = DomainProcessor::new(Arc::new(registry));

    let entity = DomainEntity {
        id: "test-entity".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };

    let result = processor.process(entity, "test-operation").await;

    let Ok(processing_result) = result else {
        panic!("processor should succeed");
    };
    assert_eq!(processing_result.state, EntityState::Complete);
    assert!(processing_result.entity_id.contains("test-entity"));
}

#[tokio::test]
async fn test_domain_processor_process_with_missing_handler() {
    let registry = Arc::new(HandlerRegistry::new());
    let processor = DomainProcessor::new(registry);

    let entity = DomainEntity {
        id: "test-entity".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };

    let result = processor.process(entity, "non-existent-operation").await;

    match result {
        Err(e) => match e {
            DomainError::HandlerNotFound(op) => {
                assert_eq!(op, "non-existent-operation");
            }
            _ => panic!("Expected HandlerNotFound error"),
        },
        Ok(_) => panic!("Expected error but got Ok"),
    }
}

#[tokio::test]
async fn test_domain_processor_context_creation() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(MockHandler {
        should_succeed: true,
    });
    registry.register("test-operation".to_string(), handler);

    let processor = DomainProcessor::new(Arc::new(registry));

    let entity = DomainEntity {
        id: "entity-123".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };

    let result = processor.process(entity, "test-operation").await;

    let Ok(processing_result) = result else {
        panic!("processor should succeed");
    };
    // Context ID should be generated as "ctx_{entity_id}"
    assert!(processing_result.entity_id.contains("entity-123"));
}

#[tokio::test]
async fn test_domain_processor_handler_invocation() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(MockHandler {
        should_succeed: true,
    });
    registry.register("test-operation".to_string(), handler);

    let processor = DomainProcessor::new(Arc::new(registry));

    let entity = DomainEntity {
        id: "test-entity".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };

    let result = processor.process(entity, "test-operation").await;

    let Ok(r) = result else {
        panic!("processor should succeed");
    };
    assert_eq!(r.state, EntityState::Complete);
}

#[tokio::test]
async fn test_domain_processor_handler_failure() {
    let mut registry = HandlerRegistry::new();
    let handler = Arc::new(MockHandler {
        should_succeed: false,
    });
    registry.register("test-operation".to_string(), handler);

    let processor = DomainProcessor::new(Arc::new(registry));

    let entity = DomainEntity {
        id: "test-entity".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };

    let result = processor.process(entity, "test-operation").await;

    match result {
        Err(e) => match e {
            DomainError::Processing(msg) => {
                assert!(msg.contains("Mock handler failed"));
            }
            _ => panic!("Expected Processing error"),
        },
        Ok(_) => panic!("Expected error but got Ok"),
    }
}
