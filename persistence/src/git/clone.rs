// Git repository cloning
//
// This module provides functionality for cloning Git repositories.

use crate::errors::PersistenceError;
use crate::git::credentials::create_callbacks;
use git2::build::RepoBuilder;
use git2::FetchOptions;
use std::path::Path;
use tokio::task;

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
        // Try clone with filter=tree:0 first
        let callbacks = create_callbacks(credentials.clone());
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Set filter via custom headers
        fetch_options.custom_headers(&["filter=tree:0"]);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let clone_result = builder.clone(&url, &path);

        // If filter approach fails, try depth 1 (shallow clone)
        if clone_result.is_err() {
            let callbacks_fallback = create_callbacks(credentials_clone);
            let mut fetch_options_fallback = FetchOptions::new();
            fetch_options_fallback.remote_callbacks(callbacks_fallback);
            fetch_options_fallback.depth(1);

            let mut builder_fallback = RepoBuilder::new();
            builder_fallback.fetch_options(fetch_options_fallback);

            builder_fallback.clone(&url, &path).map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {error_msg}"))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {error_msg}"))
                } else {
                    PersistenceError::Git(format!("Failed to clone repository: {error_msg}"))
                }
            })?;
        } else {
            clone_result.map_err(|e| {
                let error_msg = e.message().to_string();
                if error_msg.contains("authentication") || error_msg.contains("credential") {
                    PersistenceError::Authentication(format!("Failed to authenticate: {error_msg}"))
                } else if error_msg.contains("network") || error_msg.contains("connection") {
                    PersistenceError::Network(format!("Network error: {error_msg}"))
                } else {
                    PersistenceError::Git(format!("Failed to clone repository: {error_msg}"))
                }
            })?;
        }

        Ok(())
    })
    .await
    .map_err(|e| PersistenceError::Git(format!("Task join error: {e}")))?
}
