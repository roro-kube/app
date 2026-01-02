// Test command
//
// Implements the `roro-kube test` command for testing workspace configuration
// and validating that everything is set up correctly.

use super::Command;
use roro_core::WorkspaceManager;

/// Test command - validates workspace configuration
pub struct TestCommand {
    /// Verbose output
    verbose: bool,
}

impl TestCommand {
    /// Create a new test command
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

#[async_trait::async_trait]
impl Command for TestCommand {
    async fn execute(&self) -> Result<(), String> {
        println!("Running workspace tests...\n");

        // Test workspace discovery
        if self.verbose {
            println!("Test 1: Workspace discovery");
        }
        let mut workspace_manager = WorkspaceManager::new(std::env::current_dir().unwrap());
        let workspace_result = workspace_manager.discover_and_load().await;
        
        match workspace_result {
            Ok(_) => {
                println!("✓ Workspace discovered successfully");
            }
            Err(e) => {
                println!("✗ Workspace discovery failed: {e}");
                return Err(format!("Workspace test failed: {e}"));
            }
        }

        // Test workspace configuration
        if self.verbose {
            println!("\nTest 2: Workspace configuration validation");
        }
        let workspace_config = workspace_manager.workspace_config();
        match workspace_config {
            Some(config) => {
                println!("✓ Workspace configuration found");
                if self.verbose {
                    println!("  Version: {}", config.version);
                    println!("  Git Remote: {}", config.git.remote);
                    println!("  Git Branch: {}", config.git.branch);
                    println!("  Cluster Context: {}", config.cluster.context);
                }
                
                // Validate configuration
                match config.validate() {
                    Ok(_) => {
                        println!("✓ Workspace configuration is valid");
                    }
                    Err(e) => {
                        println!("✗ Workspace configuration validation failed: {e}");
                        return Err(format!("Configuration validation failed: {e}"));
                    }
                }
            }
            None => {
                println!("⚠ Workspace configuration not found (workspace.json missing)");
            }
        }

        // Test apps loading
        if self.verbose {
            println!("\nTest 3: Apps loading");
        }
        let state = workspace_manager.get_state();
        println!("✓ Found {} app(s)", state.apps.len());
        if self.verbose && !state.apps.is_empty() {
            for (name, app) in &state.apps {
                println!("  - {} (v{})", name, app.metadata.version);
                
                // Validate app configuration
                match app.validate() {
                    Ok(_) => {
                        println!("    ✓ App configuration is valid");
                    }
                    Err(e) => {
                        println!("    ✗ App configuration validation failed: {e}");
                        return Err(format!("App '{}' validation failed: {e}", name));
                    }
                }
            }
        }

        // Test environments loading
        if self.verbose {
            println!("\nTest 4: Environments loading");
        }
        println!("✓ Found {} environment(s)", state.environments.len());
        if self.verbose && !state.environments.is_empty() {
            for env in &state.environments {
                println!("  - {}", env.name);
                
                // Validate environment configuration
                match env.validate() {
                    Ok(_) => {
                        println!("    ✓ Environment configuration is valid");
                    }
                    Err(e) => {
                        println!("    ✗ Environment configuration validation failed: {e}");
                        return Err(format!("Environment '{}' validation failed: {e}", env.name));
                    }
                }
            }
        }

        println!("\n✓ All tests passed!");
        Ok(())
    }
}

impl Default for TestCommand {
    fn default() -> Self {
        Self::new(false)
    }
}

