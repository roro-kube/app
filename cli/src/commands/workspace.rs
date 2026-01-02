// Workspace command
//
// Implements workspace-related subcommands like `roro-kube workspace status`

use super::Command;
use roro_core::WorkspaceManager;

/// Workspace status command - shows workspace status
pub struct WorkspaceStatusCommand;

impl WorkspaceStatusCommand {
    /// Create a new workspace status command
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Command for WorkspaceStatusCommand {
    async fn execute(&self) -> Result<(), String> {
        // Discover workspace
        let mut workspace_manager = WorkspaceManager::new(std::env::current_dir().unwrap());
        workspace_manager
            .discover_and_load()
            .map_err(|e| format!("Failed to discover workspace: {e}"))?;

        let state = workspace_manager.get_state();

        println!("Workspace Status");
        println!("================");

        // Show workspace root
        println!("\nWorkspace Root: {}", workspace_manager.workspace_root().display());

        // Show workspace configuration
        if let Some(config) = &state.workspace_config {
            println!("\nWorkspace Configuration:");
            println!("  Version: {}", config.version);
            println!("  Git Remote: {}", config.git.remote);
            println!("  Git Branch: {}", config.git.branch);
            println!("  Auto Sync: {}", config.git.auto_sync);
            println!("  Sync Interval: {} seconds", config.git.sync_interval_seconds);
            println!("  User: {} ({})", config.user.username, config.user.identity_provider);
            println!("  Cluster Context: {}", config.cluster.context);
            println!("  Namespace Prefix: {}", config.cluster.namespace_prefix);
        } else {
            println!("\n⚠️  No workspace.json found");
        }

        // Show apps
        println!("\nApps: {}", state.apps.len());
        if state.apps.is_empty() {
            println!("  (no apps configured)");
        } else {
            for (name, app) in &state.apps {
                println!("  - {} (v{})", name, app.metadata.version);
                if !app.metadata.description.is_empty() {
                    println!("    {}", app.metadata.description);
                }
            }
        }

        // Show environments
        println!("\nEnvironments: {}", state.environments.len());
        if state.environments.is_empty() {
            println!("  (no environments configured)");
        } else {
            for env in &state.environments {
                println!("  - {}", env.name);
            }
        }

        Ok(())
    }
}

impl Default for WorkspaceStatusCommand {
    fn default() -> Self {
        Self::new()
    }
}

