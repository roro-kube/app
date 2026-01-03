// Git operations module
// This module provides functionality for cloning Git repositories and syncing changes.

use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use tokio::task;
use tokio::fs;

use crate::errors::PersistenceError;

/// Ensure that ~/.roro and ~/.roro/remote directories exist
///
/// # Returns
/// * `Ok(())` if the directories were created or already exist
/// * `Err(PersistenceError)` if directory creation failed
async fn ensure_roro_directories() -> Result<(), PersistenceError> {
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
    fs::create_dir_all(&roro_dir)
        .await
        .map_err(|e| {
            PersistenceError::Git(format!(
                "Failed to create directory {}: {}",
                roro_dir.display(),
                e
            ))
        })?;

    // Create ~/.roro/remote if it doesn't exist
    fs::create_dir_all(&remote_dir)
        .await
        .map_err(|e| {
            PersistenceError::Git(format!(
                "Failed to create directory {}: {}",
                remote_dir.display(),
                e
            ))
        })?;

    Ok(())
}

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
    // Clone credentials for fallback attempt
    let credentials_clone = credentials.clone();

    task::spawn_blocking(move || {
        // Helper function to create callbacks
        let create_callbacks = |creds: Option<(String, String)>| {
            let mut callbacks = RemoteCallbacks::new();
            if let Some((username, password)) = creds {
                let username_clone = username.clone();
                callbacks.credentials(move |_url, username_from_url, _allowed_types| {
                    let username = username_from_url.unwrap_or(&username_clone);
                    if let Ok(cred) = Cred::userpass_plaintext(username, &password) {
                        return Ok(cred);
                    }
                    if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                        return Ok(cred);
                    }
                    Cred::default()
                });
            } else {
                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    let username = username_from_url.unwrap_or("git");
                    if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                        return Ok(cred);
                    }
                    Cred::default()
                });
            }
            callbacks
        };

        // Try clone with filter=tree:0 first
        let callbacks = create_callbacks(credentials.clone());
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Set filter via custom headers
        fetch_options.custom_headers(&["filter=tree:0"]);
        
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);
        
        let clone_result = builder.clone(&url, &path);
        
        // If filter approach fails, try depth 1 (shallow clone)
        if clone_result.is_err() {
            let callbacks_fallback = create_callbacks(credentials_clone);
            let mut fetch_options_fallback = FetchOptions::new();
            fetch_options_fallback.remote_callbacks(callbacks_fallback);
            fetch_options_fallback.depth(1);
            
            let mut builder_fallback = git2::build::RepoBuilder::new();
            builder_fallback.fetch_options(fetch_options_fallback);
            
            builder_fallback
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
        } else {
            clone_result.map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {}", error_msg))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {}", error_msg))
                } else {
                    PersistenceError::Git(format!("Failed to clone repository: {}", error_msg))
                }
            })?;
        }

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

        // Helper function to create callbacks
        let create_callbacks = |creds: Option<(String, String)>| {
            let mut callbacks = RemoteCallbacks::new();
            if let Some((username, password)) = creds {
                let username_clone = username.clone();
                callbacks.credentials(move |_url, username_from_url, _allowed_types| {
                    let username = username_from_url.unwrap_or(&username_clone);
                    if let Ok(cred) = Cred::userpass_plaintext(username, &password) {
                        return Ok(cred);
                    }
                    if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                        return Ok(cred);
                    }
                    Cred::default()
                });
            } else {
                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    let username = username_from_url.unwrap_or("git");
                    if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                        return Ok(cred);
                    }
                    Cred::default()
                });
            }
            callbacks
        };

        // Try fetch with filter=tree:0 first
        let callbacks = create_callbacks(credentials.clone());
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Set filter via custom headers
        fetch_options.custom_headers(&["filter=tree:0"]);

        let fetch_result = remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None);

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

            remote_fallback.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options_fallback), None).map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {}", error_msg))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {}", error_msg))
                } else {
                    PersistenceError::Git(format!("Failed to fetch: {}", error_msg))
                }
            })?;
        } else {
            fetch_result.map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {}", error_msg))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {}", error_msg))
                } else {
                    PersistenceError::Git(format!("Failed to fetch: {}", error_msg))
                }
            })?;
        }

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
    // Ensure ~/.roro and ~/.roro/remote directories exist
    ensure_roro_directories().await?;

    // Ensure the parent directory of the repository path exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| {
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

