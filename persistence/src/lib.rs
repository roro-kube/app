// Persistence layer crate
// This crate handles data storage and retrieval operations.

pub mod config;
pub mod errors;
pub mod git;
pub mod models;
pub mod store;

pub use config::load_global_config;
pub use errors::PersistenceError;
pub use git::{clone_repository, fetch_latest, repository_exists, sync_repository};
pub use store::Store;

// Model re-exports will be added when models are implemented
// pub use models::*;
