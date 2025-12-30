// Core layer error types
// This module defines all error types used in the core layer.
// Core errors can transform errors from domain and persistence layers.

use thiserror::Error;

use domain::DomainError;
use persistence::PersistenceError;

/// Errors that can occur in the core layer
#[derive(Debug, Error)]
pub enum CoreError {
    /// Domain layer error
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    /// Persistence layer error
    #[error("Persistence error: {0}")]
    Persistence(#[from] PersistenceError),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Bridge transformation error
    #[error("Bridge error: {0}")]
    Bridge(String),
}

