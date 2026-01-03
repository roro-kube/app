// Domain layer error types
// This module defines all error types used in the domain layer.

use thiserror::Error;

/// Errors that can occur in the domain layer
#[derive(Debug, Error)]
pub enum DomainError {
    /// Processing operation failed
    #[error("Processing error: {0}")]
    Processing(String),

    /// Validation failed
    #[error("Validation error: {0}")]
    Validation(String),

    /// Handler not found for operation
    #[error("Handler not found: {0}")]
    HandlerNotFound(String),

    /// Invalid domain entity state
    #[error("Invalid entity state: {0}")]
    InvalidState(String),

    /// Domain operation timeout
    #[error("Operation timeout: {0}")]
    Timeout(String),

    /// App configuration validation failed
    #[error("App config validation error: {0}")]
    AppConfigValidation(String),

    /// Port forward configuration validation failed
    #[error("Port forward config validation error: {0}")]
    PortForwardValidation(String),
}
