// CLI commands module
//
// This module contains all CLI command implementations.
// Each command is a thin controller that delegates to the Core layer.

pub mod status;

pub use status::StatusCommand;

/// Trait for CLI commands
///
/// This trait provides a common interface for all CLI commands,
/// allowing for consistent error handling and output formatting.
#[async_trait::async_trait]
pub trait Command: Send + Sync {
    /// Execute the command
    ///
    /// # Returns
    /// * `Ok(())` if the command executed successfully
    /// * `Err(String)` if the command failed (error message for user)
    async fn execute(&self) -> Result<(), String>;
}

