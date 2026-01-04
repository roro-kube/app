// Core layer crate
// This crate orchestrates between business logic and persistence layers,
// providing unified APIs and handling cross-cutting concerns.

pub mod api;
pub mod bridge;
pub mod errors;
pub mod validation;

// Public API exports
pub use api::{get_config_path_string, load_workstation_config, sync_repository};
pub use errors::CoreError;
