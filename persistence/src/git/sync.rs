// Git repository synchronization
//
// This module provides functionality for syncing Git repositories (clone or fetch).

use crate::errors::PersistenceError;
use crate::git::clone::clone_repository;
use crate::git::directories::ensure_roro_directories;
use crate::git::fetch::{fetch_latest, repository_exists};
use std::path::Path;
use tokio::fs;

/// Sync a Git repository - clones if it doesn't exist, fetches latest if it does
///
/// # Arguments
/// * `url` - The remote repository URL (SSH or HTTPS)
/// * `path` - The local path where the repository should be cloned or exists
/// * `credentials` - Optional credentials for authentication (username, password/token)
///
/// # Returns
/// * `Ok(())` if the sync operation succeeded
/// * `Err(PersistenceError)` if the sync operation failed
///
/// # Errors
/// * `PersistenceError::Network` if network connection fails
/// * `PersistenceError::Authentication` if authentication fails
/// * `PersistenceError::Git` for other Git-related errors
pub async fn sync_repository(
    url: &str,
    path: &Path,
    credentials: Option<(&str, &str)>,
) -> Result<(), PersistenceError> {
    // Ensure ~/.roro and ~/.roro/remote directories exist
    ensure_roro_directories().await?;

    // Ensure the parent directory of the repository path exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| {
            PersistenceError::Git(format!(
                "Failed to create directory {}: {}",
                parent.display(),
                e
            ))
        })?;
    }

    // Check if repository already exists
    let exists = repository_exists(path).await?;

    if exists {
        // Repository exists, fetch latest changes
        fetch_latest(path, credentials).await
    } else {
        // Clone the repository
        clone_repository(url, path, credentials).await
    }
}
