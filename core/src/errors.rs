use thiserror::Error;

use roro_domain::DomainError;
use roro_persistence::PersistenceError;

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

    /// Kubernetes API error
    #[error("Kubernetes error: {0}")]
    Kubernetes(String),

    /// Kubeconfig loading or parsing error
    #[error("Kubeconfig error: {0}")]
    Kubeconfig(String),

    /// Cluster connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Context not found error
    #[error("Context not found: {0}")]
    ContextNotFound(String),

    /// Port forward error
    #[error("Port forward error: {0}")]
    PortForward(String),

    /// Port conflict error
    #[error("Port conflict: port {0} is already in use")]
    PortConflict(u16),

    /// Port forward not found error
    #[error("Port forward not found: {0}")]
    PortForwardNotFound(String),
}
