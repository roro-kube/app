// Git operations module
//
// This module provides functionality for cloning Git repositories and syncing changes.

mod clone;
mod credentials;
mod directories;
mod fetch;
mod sync;

#[cfg(test)]
mod tests;

pub use clone::clone_repository;
pub use fetch::{fetch_latest, repository_exists};
pub use sync::sync_repository;
