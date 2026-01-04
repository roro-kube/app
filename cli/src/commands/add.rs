// Add command
//
// Command for adding app references to the workstation configuration.

use roro_domain::{AppReference};
use roro_persistence::{load_workstation_config, save_workstation_config};

use super::Command;

/// Add command - adds an app reference to the workstation configuration
///
/// This command adds a new app reference to the workstation configuration file.
pub struct AddCommand {
    name: String,
    git_url: String,
    local_path: Option<String>,
    sync_interval: Option<u64>,
    kubectl_context: Option<String>,
    force: bool,
}

impl AddCommand {
    /// Create a new add command
    ///
    /// # Arguments
    /// * `name` - The name of the app (unique identifier)
    /// * `git_url` - The Git repository URL
    /// * `local_path` - Optional local path where the repository should be synced
    /// * `sync_interval` - Optional sync interval in milliseconds
    /// * `kubectl_context` - Optional Kubernetes context to use
    /// * `force` - If true, overwrite existing app reference with the same name
    pub fn new(
        name: String,
        git_url: String,
        local_path: Option<String>,
        sync_interval: Option<u64>,
        kubectl_context: Option<String>,
        force: bool,
    ) -> Self {
        Self {
            name,
            git_url,
            local_path,
            sync_interval,
            kubectl_context,
            force,
        }
    }
}

#[async_trait::async_trait]
impl Command for AddCommand {
    async fn execute(&self) -> Result<(), String> {
        // Load the current workstation configuration
        let mut config = load_workstation_config().await.map_err(|e| {
            format!("Failed to load workstation configuration: {e}")
        })?;

        // Check if an app with this name already exists
        if let Some(existing_index) = config.iter().position(|app| app.name == self.name) {
            if self.force {
                // Remove the existing entry to overwrite it
                config.remove(existing_index);
            } else {
                return Err(format!(
                    "App '{}' already exists in workstation configuration. Use --force to overwrite",
                    self.name
                ));
            }
        }

        // Create the new app reference
        let app_reference = AppReference {
            name: self.name.clone(),
            git_url: self.git_url.clone(),
            local_path: self.local_path.clone(),
            sync_interval: self.sync_interval,
            kubectl_context: self.kubectl_context.clone(),
        };

        // Add the app reference to the configuration
        config.push(app_reference);

        // Save the updated configuration
        save_workstation_config(&config).await.map_err(|e| {
            format!("Failed to save workstation configuration: {e}")
        })?;

        println!("Successfully added app '{}' to workstation configuration", self.name);
        Ok(())
    }
}

