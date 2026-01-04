// Sync command
//
// Command for syncing configurations from Git repositories.

use roro_core::{get_config_path_string, sync_repository, CoreError};
use roro_domain::WorkstationConfig;

use super::Command;

/// Sync command - syncs configurations from Git repositories
///
/// This command synchronizes configurations from Git repositories,
/// pulling the latest changes and updating local configurations.
pub struct SyncCommand {
    app_name: String,
    workstation_config: WorkstationConfig,
}

impl SyncCommand {
    /// Create a new sync command
    ///
    /// # Arguments
    /// * `app_name` - The name of the app reference to sync
    /// * `workstation_config` - The workstation configuration containing app references
    #[must_use]
    pub fn new(app_name: String, workstation_config: WorkstationConfig) -> Self {
        Self {
            app_name,
            workstation_config,
        }
    }
}

#[async_trait::async_trait]
impl Command for SyncCommand {
    async fn execute(&self) -> Result<(), String> {
        // Find the app reference by name
        let app_reference = self
            .workstation_config
            .iter()
            .find(|app| app.name == self.app_name)
            .ok_or_else(|| {
                let config_path =
                    get_config_path_string().unwrap_or_else(|_| "~/.roro/config.json".to_string());
                format!(
                    "App '{}' not found in workstation configuration (config file: {})",
                    self.app_name, config_path
                )
            })?;

        // Get the resolved local path (with default if not specified)
        let local_path = app_reference.get_local_path()?;

        // Sync the repository (credentials will be handled by git credential manager)
        sync_repository(&app_reference.git_url, &local_path, None)
            .await
            .map_err(|e| {
                // Format error message - CoreError::Persistence already includes the inner error message
                match &e {
                    CoreError::Persistence(pe) => {
                        // The PersistenceError Display already formats as "Network error: ...", etc.
                        // CoreError wraps it as "Persistence error: Network error: ..."
                        // Strip the "Persistence error: " prefix for cleaner messages
                        let msg = format!("{pe}");
                        if msg.starts_with("Persistence error: ") {
                            msg.strip_prefix("Persistence error: ")
                                .unwrap_or(&msg)
                                .to_string()
                        } else {
                            format!("Error: {pe}")
                        }
                    }
                    _ => format!("Error: {e}"),
                }
            })?;

        println!(
            "Successfully synced repository '{}' from {} to {}",
            self.app_name,
            app_reference.git_url,
            local_path.display()
        );
        Ok(())
    }
}
