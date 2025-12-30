// Domain layer handlers
// This module defines the handler trait and registry for domain operations.

use crate::errors::DomainError;
use crate::types::{DomainEntity, ProcessingContext, ProcessingResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for operation handlers that process domain operations
#[async_trait]
pub trait OperationHandler: Send + Sync {
    /// Handle a domain operation
    async fn handle(
        &self,
        context: &mut ProcessingContext,
        entity: &DomainEntity,
    ) -> Result<ProcessingResult, DomainError>;
}

/// Registry for managing operation handlers
pub struct HandlerRegistry {
    handlers: HashMap<String, Arc<dyn OperationHandler>>,
}

impl HandlerRegistry {
    /// Create a new handler registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for an operation type
    pub fn register(&mut self, operation_type: String, handler: Arc<dyn OperationHandler>) {
        self.handlers.insert(operation_type, handler);
    }

    /// Get a handler for an operation type
    pub fn get_handler(&self, operation_type: &str) -> Option<&Arc<dyn OperationHandler>> {
        self.handlers.get(operation_type)
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

