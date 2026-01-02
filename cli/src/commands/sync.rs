// Sync command
//
// Command for syncing configurations from Git repositories.

use std::path::PathBuf;

use roro_persistence::{sync_repository, PersistenceError};

use super::Command;

/// Sync command - syncs configurations from Git repositories
///
/// This command synchronizes configurations from Git repositories,
/// pulling the latest changes and updating local configurations.
pub struct SyncCommand {
    url: String,
    path: PathBuf,
    username: Option<String>,
    password: Option<String>,
}

impl SyncCommand {
    /// Create a new sync command
    ///
    /// # Arguments
    /// * `url` - The remote repository URL (SSH or HTTPS)
    /// * `path` - The local path where the repository should be synced
    /// * `username` - Optional username for authentication
    /// * `password` - Optional password/token for authentication
    pub fn new(url: String, path: PathBuf, username: Option<String>, password: Option<String>) -> Self {
        Self {
            url,
            path,
            username,
            password,
        }
    }
}

#[async_trait::async_trait]
impl Command for SyncCommand {
    async fn execute(&self) -> Result<(), String> {
        let credentials = match (&self.username, &self.password) {
            (Some(u), Some(p)) => Some((u.as_str(), p.as_str())),
            _ => None,
        };

        sync_repository(&self.url, &self.path, credentials)
            .await
            .map_err(|e| match e {
                PersistenceError::Network(msg) => format!("Network error: {}", msg),
                PersistenceError::Authentication(msg) => format!("Authentication error: {}", msg),
                PersistenceError::Git(msg) => format!("Git error: {}", msg),
                _ => format!("Error: {}", e),
            })?;

        println!("Successfully synced repository from {} to {:?}", self.url, self.path);
        Ok(())
    }
}
