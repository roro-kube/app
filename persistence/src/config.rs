// Configuration loading module
// This module provides functionality for loading the global configuration from ~/.roro/config.json

use std::path::PathBuf;

use roro_domain::GlobalConfig;
use tokio::fs;

use crate::errors::PersistenceError;

/// Get the path to the global configuration file
///
/// Returns `~/.roro/config.json` resolved to an absolute path
///
/// # Errors
/// * `PersistenceError::InvalidInput` if the home directory cannot be determined
fn get_config_path() -> Result<PathBuf, PersistenceError> {
    // Try HOME first (works on Unix and often on Windows)
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| {
            PersistenceError::InvalidInput(
                "Cannot determine home directory. HOME or USERPROFILE environment variable must be set.".to_string(),
            )
        })?;

    Ok(PathBuf::from(home).join(".roro").join("config.json"))
}

/// Load the global configuration from ~/.roro/config.json
///
/// # Returns
/// * `Ok(GlobalConfig)` if the configuration was loaded successfully
/// * `Err(PersistenceError)` if loading failed
///
/// # Errors
/// * `PersistenceError::NotFound` if the config file doesn't exist
/// * `PersistenceError::Serialization` if the config file cannot be parsed
/// * `PersistenceError::InvalidInput` if the home directory cannot be determined
pub async fn load_global_config() -> Result<GlobalConfig, PersistenceError> {
    let config_path = get_config_path()?;

    let contents = fs::read_to_string(&config_path)
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                PersistenceError::NotFound(format!(
                    "Configuration file not found: {}",
                    config_path.display()
                ))
            } else {
                PersistenceError::Serialization(format!(
                    "Failed to read configuration file {}: {}",
                    config_path.display(),
                    e
                ))
            }
        })?;

    let config: GlobalConfig = serde_json::from_str(&contents).map_err(|e| {
        PersistenceError::Serialization(format!(
            "Failed to parse configuration file {}: {}",
            config_path.display(),
            e
        ))
    })?;

    Ok(config)
}

