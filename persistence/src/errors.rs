// Persistence layer error types
// This module defines all error types used in the persistence layer.

use thiserror::Error;

/// Errors that can occur in the persistence layer
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(String),

    /// Entity not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

