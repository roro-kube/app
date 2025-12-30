// Persistence layer crate
// This crate handles data storage and retrieval operations.

pub mod errors;
pub mod models;
pub mod store;

pub use errors::PersistenceError;
pub use store::Store;

// Model re-exports will be added when models are implemented
// pub use models::*;

