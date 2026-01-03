// Domain layer crate
// This crate contains business logic and domain models.

pub mod config;
pub mod errors;
pub mod handlers;
pub mod processor;
pub mod types;

pub use config::{AppReference, WorkstationConfig};
pub use errors::DomainError;
pub use handlers::{HandlerRegistry, OperationHandler};
pub use processor::DomainProcessor;
pub use types::{
    AppConfig, DomainEntity, EntityState, PortForwardingConfig, PortValue, ProcessingContext,
    ProcessingResult,
};
