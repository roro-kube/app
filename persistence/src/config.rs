// Configuration loading module
// This module provides functionality for loading the workstation configuration from ~/.roro/config.json

use std::path::PathBuf;

use roro_domain::WorkstationConfig;
use tokio::fs;

use crate::errors::PersistenceError;

/// Get the path to the workstation configuration file
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

/// Get the workstation configuration file path as a string
///
/// Returns the path to `~/.roro/config.json` as a displayable string
///
/// # Errors
/// * `PersistenceError::InvalidInput` if the home directory cannot be determined
pub fn get_config_path_string() -> Result<String, PersistenceError> {
    get_config_path().map(|p| p.display().to_string())
}

/// Load the workstation configuration from ~/.roro/config.json
///
/// If the config file doesn't exist, it will be initialized with an empty array.
///
/// # Returns
/// * `Ok(WorkstationConfig)` if the configuration was loaded successfully
/// * `Err(PersistenceError)` if loading failed
///
/// # Errors
/// * `PersistenceError::Serialization` if the config file cannot be parsed or created
/// * `PersistenceError::InvalidInput` if the home directory cannot be determined
pub async fn load_workstation_config() -> Result<WorkstationConfig, PersistenceError> {
    let config_path = get_config_path()?;

    // Check if config file exists
    let contents = match fs::read_to_string(&config_path).await {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Config file doesn't exist, initialize it with an empty array
            let roro_dir = config_path.parent().ok_or_else(|| {
                PersistenceError::InvalidInput(
                    "Cannot determine config directory".to_string(),
                )
            })?;

            // Ensure ~/.roro directory exists
            fs::create_dir_all(roro_dir)
                .await
                .map_err(|e| {
                    PersistenceError::Serialization(format!(
                        "Failed to create directory {}: {}",
                        roro_dir.display(),
                        e
                    ))
                })?;

            // Initialize with empty array (empty WorkstationConfig)
            let empty_config: WorkstationConfig = Vec::new();
            let json_content = serde_json::to_string_pretty(&empty_config).map_err(|e| {
                PersistenceError::Serialization(format!(
                    "Failed to serialize empty config: {}",
                    e
                ))
            })?;

            // Write the empty config file
            fs::write(&config_path, json_content)
                .await
                .map_err(|e| {
                    PersistenceError::Serialization(format!(
                        "Failed to write configuration file {}: {}",
                        config_path.display(),
                        e
                    ))
                })?;

            // Return the empty config
            return Ok(empty_config);
        }
        Err(e) => {
            return Err(PersistenceError::Serialization(format!(
                "Failed to read configuration file {}: {}",
                config_path.display(),
                e
            )));
        }
    };

    let config: WorkstationConfig = serde_json::from_str(&contents).map_err(|e| {
        PersistenceError::Serialization(format!(
            "Failed to parse configuration file {}: {}",
            config_path.display(),
            e
        ))
    })?;

    Ok(config)
}

