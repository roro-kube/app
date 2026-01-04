// Configuration and Git API module
//
// This module provides wrappers for persistence layer configuration and git operations,
// exposing them through the core layer API with automatic error conversion.

use std::path::Path;

use roro_domain::WorkstationConfig;
use roro_persistence;

use crate::errors::CoreError;

/// Get the workstation configuration file path as a string
///
/// Returns the path to `~/.roro/config.json` as a displayable string
///
/// # Errors
/// * `CoreError::Persistence` if the home directory cannot be determined
pub fn get_config_path_string() -> Result<String, CoreError> {
    roro_persistence::get_config_path_string().map_err(Into::into)
}

/// Load the workstation configuration from ~/.roro/config.json
///
/// If the config file doesn't exist, it will be initialized with an empty array.
///
/// # Returns
/// * `Ok(WorkstationConfig)` if the configuration was loaded successfully
/// * `Err(CoreError)` if loading failed
///
/// # Errors
/// * `CoreError::Persistence` if the config file cannot be parsed, created, or the home directory cannot be determined
pub async fn load_workstation_config() -> Result<WorkstationConfig, CoreError> {
    roro_persistence::load_workstation_config()
        .await
        .map_err(Into::into)
}

/// Sync a Git repository - clones if it doesn't exist, fetches latest if it does
///
/// # Arguments
/// * `url` - The remote repository URL (SSH or HTTPS)
/// * `path` - The local path where the repository should be cloned or exists
/// * `credentials` - Optional credentials for authentication (username, password/token)
///
/// # Returns
/// * `Ok(())` if the sync operation succeeded
/// * `Err(CoreError)` if the sync operation failed
///
/// # Errors
/// * `CoreError::Persistence` if network connection fails, authentication fails, or other Git-related errors occur
pub async fn sync_repository(
    url: &str,
    path: &Path,
    credentials: Option<(&str, &str)>,
) -> Result<(), CoreError> {
    roro_persistence::sync_repository(url, path, credentials)
        .await
        .map_err(Into::into)
}
