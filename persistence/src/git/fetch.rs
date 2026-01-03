// Git repository fetching
//
// This module provides functionality for fetching changes from Git repositories.

use crate::errors::PersistenceError;
use crate::git::credentials::create_callbacks;
use git2::{FetchOptions, Repository};
use std::path::Path;
use tokio::task;

/// Check if a Git repository exists at the given path
///
/// # Arguments
/// * `path` - The path to check for a Git repository
///
/// # Returns
/// * `Ok(true)` if a repository exists at the path
/// * `Ok(false)` if no repository exists at the path
/// * `Err(PersistenceError)` if there was an error checking
///
/// # Errors
/// * `PersistenceError::Git` if there was an error checking the repository
pub async fn repository_exists(path: &Path) -> Result<bool, PersistenceError> {
    let path = path.to_path_buf();

    task::spawn_blocking(move || match Repository::open(&path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.code() == git2::ErrorCode::NotFound {
                Ok(false)
            } else {
                Err(PersistenceError::Git(format!(
                    "Error checking repository: {}",
                    e.message()
                )))
            }
        }
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {e}")))?
}

/// Fetch the latest changes from the remote repository
///
/// # Arguments
/// * `path` - The path to the local repository
/// * `credentials` - Optional credentials for authentication (username, password/token)
///
/// # Returns
/// * `Ok(())` if the fetch operation succeeded
/// * `Err(PersistenceError)` if the fetch operation failed
///
/// # Errors
/// * `PersistenceError::Network` if network connection fails
/// * `PersistenceError::Authentication` if authentication fails
/// * `PersistenceError::Git` for other Git-related errors
pub async fn fetch_latest(
    path: &Path,
    credentials: Option<(&str, &str)>,
) -> Result<(), PersistenceError> {
    let path = path.to_path_buf();
    // Convert credentials to owned Strings before moving into closure
    let credentials = credentials.map(|(u, p)| (u.to_string(), p.to_string()));
    // Clone credentials for fallback attempt
    let credentials_clone = credentials.clone();

    task::spawn_blocking(move || {
        let repo = Repository::open(&path).map_err(|e| {
            PersistenceError::Git(format!("Failed to open repository: {}", e.message()))
        })?;

        // Find the default remote (usually "origin")
        let mut remote = repo.find_remote("origin").map_err(|e| {
            PersistenceError::Git(format!("Failed to find remote 'origin': {}", e.message()))
        })?;

        // Try fetch with filter=tree:0 first
        let callbacks = create_callbacks(credentials.clone());
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Set filter via custom headers
        fetch_options.custom_headers(&["filter=tree:0"]);

        let fetch_result = remote.fetch(
            &["refs/heads/*:refs/remotes/origin/*"],
            Some(&mut fetch_options),
            None,
        );

        // If filter approach fails, try depth 1 (shallow fetch)
        if fetch_result.is_err() {
            // Get a fresh remote reference for fallback
            let mut remote_fallback = repo.find_remote("origin").map_err(|e| {
                PersistenceError::Git(format!("Failed to find remote 'origin': {}", e.message()))
            })?;

            let callbacks_fallback = create_callbacks(credentials_clone);
            let mut fetch_options_fallback = FetchOptions::new();
            fetch_options_fallback.remote_callbacks(callbacks_fallback);
            fetch_options_fallback.depth(1);

            remote_fallback
                .fetch(
                    &["refs/heads/*:refs/remotes/origin/*"],
                    Some(&mut fetch_options_fallback),
                    None,
                )
                .map_err(|e| {
                    let error_msg = e.message().to_string();
                    if error_msg.contains("authentication") || error_msg.contains("credential") {
                        PersistenceError::Authentication(format!(
                            "Failed to authenticate: {error_msg}"
                        ))
                    } else if error_msg.contains("network") || error_msg.contains("connection") {
                        PersistenceError::Network(format!("Network error: {error_msg}"))
                    } else {
                        PersistenceError::Git(format!("Failed to fetch: {error_msg}"))
                    }
                })?;
        } else {
            fetch_result.map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {error_msg}"))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {error_msg}"))
                } else {
                    PersistenceError::Git(format!("Failed to fetch: {error_msg}"))
                }
            })?;
        }

        Ok(())
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {e}")))?
}
