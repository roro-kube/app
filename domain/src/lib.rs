// Domain layer crate
// This crate contains business logic and domain models.

pub mod errors;
pub mod handlers;
pub mod processor;
pub mod types;

pub use errors::DomainError;
pub use handlers::{HandlerRegistry, OperationHandler};
pub use processor::DomainProcessor;
pub use types::{DomainEntity, EntityState, ProcessingContext, ProcessingResult};
