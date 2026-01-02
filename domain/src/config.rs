// Configuration schema definitions
//
// This module contains Rust structs representing the global configuration file schema:
// - ~/.roro/config.json: Global repository configuration

use serde::{Deserialize, Serialize};

/// App configuration
///
/// Represents a single app configuration entry in ~/.roro/config.json
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Git repository URL
    pub git_url: String,
    /// Local path where the repository should be synced
    pub local_path: String,
    /// Sync interval in milliseconds
    pub sync_interval: u64,
    /// Kubernetes context to use
    pub kubectl_context: String,
}

/// Global configuration
///
/// Represents the entire configuration array from ~/.roro/config.json
pub type GlobalConfig = Vec<AppConfig>;

