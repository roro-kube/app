// Directory management for Git operations
//
// This module provides functionality for ensuring required directories exist.

use crate::errors::PersistenceError;
use tokio::fs;

/// Ensure that ~/.roro and ~/.roro/remote directories exist
///
/// # Returns
/// * `Ok(())` if the directories were created or already exist
/// * `Err(PersistenceError)` if directory creation failed
pub async fn ensure_roro_directories() -> Result<(), PersistenceError> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| {
            PersistenceError::InvalidInput(
                "Cannot determine home directory. HOME or USERPROFILE environment variable must be set.".to_string(),
            )
        })?;

    let roro_dir = std::path::PathBuf::from(&home).join(".roro");
    let remote_dir = roro_dir.join("remote");

    // Create ~/.roro if it doesn't exist
    fs::create_dir_all(&roro_dir).await.map_err(|e| {
        PersistenceError::Git(format!(
            "Failed to create directory {}: {}",
            roro_dir.display(),
            e
        ))
    })?;

    // Create ~/.roro/remote if it doesn't exist
    fs::create_dir_all(&remote_dir).await.map_err(|e| {
        PersistenceError::Git(format!(
            "Failed to create directory {}: {}",
            remote_dir.display(),
            e
        ))
    })?;

    Ok(())
}
