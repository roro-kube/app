// Workspace discovery and loading
//
// This module handles discovering workspace directories, loading configuration files,
// and watching for changes to enable live updates.

use crate::errors::CoreError;
use roro_domain::{
    AppConfig, EnvironmentConfig, WorkspaceConfig,
};
use serde_json;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::watch;

/// Workspace manager that handles discovery and loading of configuration files
pub struct WorkspaceManager {
    workspace_root: PathBuf,
    workspace_config: Option<WorkspaceConfig>,
    apps: Vec<(String, AppConfig)>,
    environments: Vec<EnvironmentConfig>,
    config_sender: Option<watch::Sender<WorkspaceState>>,
}

/// Current state of the workspace
#[derive(Debug, Clone)]
pub struct WorkspaceState {
    pub workspace_config: Option<WorkspaceConfig>,
    pub apps: Vec<(String, AppConfig)>,
    pub environments: Vec<EnvironmentConfig>,
}

impl WorkspaceManager {
    /// Create a new workspace manager
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            workspace_config: None,
            apps: Vec::new(),
            environments: Vec::new(),
            config_sender: None,
        }
    }

    /// Discover and load workspace configuration
    ///
    /// Looks for .kube-apps directory or workspace.json in the current directory
    pub async fn discover_and_load(&mut self) -> Result<(), CoreError> {
        // Try to find workspace root
        let workspace_path = self.find_workspace_root().await?;
        self.workspace_root = workspace_path;

        // Load workspace.json
        let workspace_json_path = self.workspace_root.join("workspace.json");
        if workspace_json_path.exists() {
            self.workspace_config = Some(self.load_workspace_config(&workspace_json_path).await?);
        }

        // Load all apps
        self.load_apps().await?;

        // Load all environments
        self.load_environments().await?;

        Ok(())
    }

    /// Find the workspace root directory
    ///
    /// Looks for .kube-apps directory or workspace.json file
    async fn find_workspace_root(&self) -> Result<PathBuf, CoreError> {
        let mut current = std::env::current_dir()
            .map_err(|e| CoreError::Internal(format!("Failed to get current directory: {e}")))?;

        loop {
            // Check for .kube-apps directory
            let kube_apps = current.join(".kube-apps");
            if kube_apps.is_dir() {
                return Ok(kube_apps);
            }

            // Check for workspace.json
            let workspace_json = current.join("workspace.json");
            if workspace_json.is_file() {
                return Ok(current);
            }

            // Move up one directory
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => {
                    return Err(CoreError::NotFound(
                        "Workspace root not found. Expected .kube-apps directory or workspace.json file".to_string(),
                    ));
                }
            }
        }
    }

    /// Load workspace.json configuration
    async fn load_workspace_config(&self, path: &Path) -> Result<WorkspaceConfig, CoreError> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read workspace.json: {e}")))?;

        let config: WorkspaceConfig = serde_json::from_str(&content)
            .map_err(|e| CoreError::Validation(format!("Failed to parse workspace.json: {e}")))?;

        config
            .validate()
            .map_err(|e| CoreError::Validation(format!("Invalid workspace.json: {e}")))?;

        Ok(config)
    }

    /// Load all app configurations from apps/ directory
    async fn load_apps(&mut self) -> Result<(), CoreError> {
        let apps_dir = self.workspace_root.join("apps");
        if !apps_dir.exists() {
            // No apps directory is okay
            return Ok(());
        }

        let mut entries = fs::read_dir(&apps_dir)
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read apps directory: {e}")))?;

        let mut apps = Vec::new();

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read apps directory entry: {e}")))?
        {
            let path = entry.path();
            if path.is_dir() {
                let app_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| {
                        CoreError::Internal("Invalid app directory name".to_string())
                    })?
                    .to_string();

                let app_json_path = path.join("app.json");
                if app_json_path.exists() {
                    let app_config = self.load_app_config(&app_json_path).await?;
                    apps.push((app_name, app_config));
                }
            }
        }

        self.apps = apps;
        Ok(())
    }

    /// Load app.json configuration
    async fn load_app_config(&self, path: &Path) -> Result<AppConfig, CoreError> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read app.json: {e}")))?;

        let config: AppConfig = serde_json::from_str(&content)
            .map_err(|e| CoreError::Validation(format!("Failed to parse app.json: {e}")))?;

        config
            .validate()
            .map_err(|e| CoreError::Validation(format!("Invalid app.json: {e}")))?;

        Ok(config)
    }

    /// Load all environment configurations
    async fn load_environments(&mut self) -> Result<(), CoreError> {
        let envs_dir = self.workspace_root.join("environments");
        if !envs_dir.exists() {
            // No environments directory is okay
            return Ok(());
        }

        let mut entries = fs::read_dir(&envs_dir)
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read environments directory: {e}")))?;

        let mut environments = Vec::new();

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read environments directory entry: {e}")))?
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let env_config = self.load_environment_config(&path).await?;
                environments.push(env_config);
            }
        }

        self.environments = environments;
        Ok(())
    }

    /// Load environment.json configuration
    async fn load_environment_config(&self, path: &Path) -> Result<EnvironmentConfig, CoreError> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to read environment.json: {e}")))?;

        let config: EnvironmentConfig = serde_json::from_str(&content)
            .map_err(|e| CoreError::Validation(format!("Failed to parse environment.json: {e}")))?;

        config
            .validate()
            .map_err(|e| CoreError::Validation(format!("Invalid environment.json: {e}")))?;

        Ok(config)
    }

    /// Get the current workspace state
    pub fn get_state(&self) -> WorkspaceState {
        WorkspaceState {
            workspace_config: self.workspace_config.clone(),
            apps: self.apps.clone(),
            environments: self.environments.clone(),
        }
    }

    /// Get workspace root path
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Get workspace configuration
    pub fn workspace_config(&self) -> Option<&WorkspaceConfig> {
        self.workspace_config.as_ref()
    }

    /// Get all apps
    pub fn apps(&self) -> &[(String, AppConfig)] {
        &self.apps
    }

    /// Get all environments
    pub fn environments(&self) -> &[EnvironmentConfig] {
        &self.environments
    }
}

