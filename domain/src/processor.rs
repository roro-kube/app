// Domain layer processor
// This module contains the main business logic processor.

use crate::errors::DomainError;
use crate::handlers::HandlerRegistry;
use crate::types::{DomainEntity, ProcessingContext, ProcessingResult};
use std::sync::Arc;

/// Main processor for domain operations
pub struct DomainProcessor {
    handlers: Arc<HandlerRegistry>,
}

impl DomainProcessor {
    /// Create a new domain processor with a handler registry
    pub fn new(handlers: Arc<HandlerRegistry>) -> Self {
        Self { handlers }
    }

    /// Process a domain entity
    pub async fn process(
        &self,
        entity: DomainEntity,
        operation_type: &str,
    ) -> Result<ProcessingResult, DomainError> {
        let handler = self
            .handlers
            .get_handler(operation_type)
            .ok_or_else(|| DomainError::HandlerNotFound(operation_type.to_string()))?;

        let mut context = ProcessingContext {
            context_id: format!("ctx_{}", entity.id),
            entity_name: entity.name.clone(),
            state: std::collections::HashMap::new(),
            logs: Vec::new(),
        };

        handler.handle(&mut context, &entity).await
    }
}

