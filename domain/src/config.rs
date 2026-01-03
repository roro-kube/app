// Configuration schema definitions
//
// This module contains Rust structs representing the workstation configuration file schema:
// - ~/.roro/config.json: Workstation repository configuration

use serde::{Deserialize, Serialize};

/// App reference
///
/// Represents a single app reference entry in ~/.roro/config.json
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppReference {
    /// App name (unique identifier)
    pub name: String,
    /// Git repository URL
    pub git_url: String,
    /// Local path where the repository should be synced
    /// Defaults to `~/.roro/remote/{name}` if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    /// Sync interval in milliseconds
    /// Defaults to 300 if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_interval: Option<u64>,
    /// Kubernetes context to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubectl_context: Option<String>,
}

impl AppReference {
    /// Get the resolved local path, defaulting to `~/.roro/remote/{name}` if not specified
    ///
    /// # Errors
    /// * Returns an error if the home directory cannot be determined
    pub fn get_local_path(&self) -> Result<std::path::PathBuf, String> {
        if let Some(path) = &self.local_path {
            Ok(std::path::PathBuf::from(path))
        } else {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .map_err(|_| {
                    "Cannot determine home directory. HOME or USERPROFILE environment variable must be set.".to_string()
                })?;
            Ok(std::path::PathBuf::from(home)
                .join(".roro")
                .join("remote")
                .join(&self.name))
        }
    }

    /// Get the resolved sync interval, defaulting to 300 milliseconds if not specified
    pub fn get_sync_interval(&self) -> u64 {
        self.sync_interval.unwrap_or(300)
    }
}

/// Workstation configuration
///
/// Represents the entire configuration array from ~/.roro/config.json
pub type WorkstationConfig = Vec<AppReference>;

