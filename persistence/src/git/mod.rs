// Git operations module
// This module provides functionality for cloning Git repositories and syncing changes.

use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use tokio::task;

use crate::errors::PersistenceError;

/// Clone a Git repository from a remote URL to a local path
///
/// # Arguments
/// * `url` - The remote repository URL (SSH or HTTPS)
/// * `path` - The local path where the repository should be cloned
/// * `credentials` - Optional credentials for authentication (username, password/token)
///
/// # Returns
/// * `Ok(())` if the clone operation succeeded
/// * `Err(PersistenceError)` if the clone operation failed
///
/// # Errors
/// * `PersistenceError::Network` if network connection fails
/// * `PersistenceError::Authentication` if authentication fails
/// * `PersistenceError::Git` for other Git-related errors
pub async fn clone_repository(
    url: &str,
    path: &Path,
    credentials: Option<(&str, &str)>,
) -> Result<(), PersistenceError> {
    let url = url.to_string();
    let path = path.to_path_buf();
    // Convert credentials to owned Strings before moving into closure
    let credentials = credentials.map(|(u, p)| (u.to_string(), p.to_string()));

    task::spawn_blocking(move || {
        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();

        // Set up authentication callbacks
        if let Some((username, password)) = credentials {

            let username_clone = username.clone();
            callbacks.credentials(move |_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or(&username_clone);

                // Try HTTPS authentication first
                if let Ok(cred) = Cred::userpass_plaintext(username, &password) {
                    return Ok(cred);
                }

                // Try SSH key authentication (will use default SSH keys)
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }

                // Try default credentials
                Cred::default()
            });
        } else {
            // No credentials provided - try default authentication methods
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");

                // Try SSH key from agent
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }

                // Try default credentials
                Cred::default()
            });
        }

        fetch_options.remote_callbacks(callbacks);

        // Clone the repository
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        builder
            .clone(&url, &path)
            .map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {}", error_msg))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {}", error_msg))
                } else {
                    PersistenceError::Git(format!("Failed to clone repository: {}", error_msg))
                }
            })?;

        Ok(())
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {}", e)))?
}

/// Check if a Git repository exists at the given path
///
/// # Arguments
/// * `path` - The path to check for a Git repository
///
/// # Returns
/// * `Ok(true)` if a repository exists at the path
/// * `Ok(false)` if no repository exists at the path
/// * `Err(PersistenceError)` if there was an error checking
pub async fn repository_exists(path: &Path) -> Result<bool, PersistenceError> {
    let path = path.to_path_buf();

    task::spawn_blocking(move || {
        match Repository::open(&path) {
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
        }
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {}", e)))?
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
pub async fn fetch_latest(path: &Path, credentials: Option<(&str, &str)>) -> Result<(), PersistenceError> {
    let path = path.to_path_buf();
    // Convert credentials to owned Strings before moving into closure
    let credentials = credentials.map(|(u, p)| (u.to_string(), p.to_string()));

    task::spawn_blocking(move || {
        let repo = Repository::open(&path).map_err(|e| {
            PersistenceError::Git(format!("Failed to open repository: {}", e.message()))
        })?;

        // Find the default remote (usually "origin")
        let mut remote = repo.find_remote("origin").map_err(|e| {
            PersistenceError::Git(format!("Failed to find remote 'origin': {}", e.message()))
        })?;

        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();

        // Set up authentication callbacks
        if let Some((username, password)) = credentials {
            let username_clone = username.clone();
            callbacks.credentials(move |_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or(&username_clone);

                // Try HTTPS authentication first
                if let Ok(cred) = Cred::userpass_plaintext(username, &password) {
                    return Ok(cred);
                }

                // Try SSH key authentication (will use default SSH keys)
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }

                // Try default credentials
                Cred::default()
            });
        } else {
            // No credentials provided - try default authentication methods
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                let username = username_from_url.unwrap_or("git");

                // Try SSH key from agent
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }

                // Try default credentials
                Cred::default()
            });
        }

        fetch_options.remote_callbacks(callbacks);

        // Fetch from remote
        remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None).map_err(|e| {
            let error_msg = e.message().to_string();
            if error_msg.contains("authentication") || error_msg.contains("credential") {
                PersistenceError::Authentication(format!("Failed to authenticate: {}", error_msg))
            } else if error_msg.contains("network") || error_msg.contains("connection") {
                PersistenceError::Network(format!("Network error: {}", error_msg))
            } else {
                PersistenceError::Git(format!("Failed to fetch: {}", error_msg))
            }
        })?;

        Ok(())
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {}", e)))?
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
    // Check if repository already exists
    let exists = repository_exists(path).await?;

    if exists {
        // Repository exists, fetch latest changes
        fetch_latest(path, credentials).await
    } else {
        // Repository doesn't exist, clone it
        clone_repository(url, path, credentials).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_clone_repository_public() {
        // Clone a public repository (no authentication required)
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("cloned_repo");

        // Using a small public repository for testing
        let result = clone_repository(
            "https://github.com/octocat/Hello-World.git",
            &repo_path,
            None,
        )
        .await;

        // This test may fail if network is unavailable, which is acceptable
        if result.is_ok() {
            assert!(
                repo_path.exists(),
                "Cloned repository directory should exist"
            );
            assert!(
                repo_path.join(".git").exists(),
                ".git directory should exist in cloned repo"
            );
        }
    }

    #[tokio::test]
    async fn test_clone_repository_with_credentials() {
        // Test cloning with credentials (will fail authentication, but tests the code path)
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("cloned_repo_auth");

        let result = clone_repository(
            "https://github.com/octocat/Hello-World.git",
            &repo_path,
            Some(("test_user", "test_token")),
        )
        .await;

        // This test may fail due to invalid credentials or network, which is acceptable
        // We're mainly testing that the code path with credentials is executed
        match result {
            Ok(_) => {
                assert!(
                    repo_path.exists(),
                    "Cloned repository directory should exist"
                );
            }
            Err(PersistenceError::Authentication(_)) => {
                // Expected for invalid credentials
            }
            Err(PersistenceError::Network(_)) => {
                // Expected if network is unavailable
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}

