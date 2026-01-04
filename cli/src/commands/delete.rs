// Delete command
//
// Command for deleting app references from the workstation configuration.

use roro_persistence::{load_workstation_config, save_workstation_config};

use super::Command;

/// Delete command - deletes an app reference from the workstation configuration
///
/// This command removes an app reference from the workstation configuration file.
pub struct DeleteCommand {
    name: String,
}

impl DeleteCommand {
    /// Create a new delete command
    ///
    /// # Arguments
    /// * `name` - The name of the app to delete
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait::async_trait]
impl Command for DeleteCommand {
    async fn execute(&self) -> Result<(), String> {
        // Load the current workstation configuration
        let mut config = load_workstation_config().await.map_err(|e| {
            format!("Failed to load workstation configuration: {e}")
        })?;

        // Find and remove the app reference by name
        let initial_len = config.len();
        config.retain(|app| app.name != self.name);

        // Check if the app was actually removed
        if config.len() == initial_len {
            return Err(format!(
                "App '{}' not found in workstation configuration",
                self.name
            ));
        }

        // Save the updated configuration
        save_workstation_config(&config).await.map_err(|e| {
            format!("Failed to save workstation configuration: {e}")
        })?;

        println!("Successfully deleted app '{}' from workstation configuration", self.name);
        Ok(())
    }
}
