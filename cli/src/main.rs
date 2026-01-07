// CLI Application Layer Entry Point
//
// This is a command-line interface for Roro Kube.
// It provides a thin controller layer that delegates to the Core layer.

use clap::Parser;
use roro_cli::{AddCommand, Command, DeleteCommand, StatusCommand, SyncCommand};
use roro_persistence::load_workstation_config;


/// Roro Kube - Docker Compose for Kubernetes
#[derive(Parser, Debug)]
#[command(name = "roro-kube")]
#[command(version)]
#[command(about = "A desktop application for managing Kubernetes deployments", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Add an app reference to the workstation configuration
    Add {
        /// The name of the app (unique identifier)
        name: String,
        /// The Git repository URL
        git_url: String,
        /// Local path where the repository should be synced
        #[arg(long)]
        local_path: Option<String>,
        /// Sync interval in milliseconds
        #[arg(long)]
        sync_interval: Option<u64>,
        /// Kubernetes context to use
        #[arg(long)]
        kubectl_context: Option<String>,
        /// Overwrite existing app reference if it already exists
        #[arg(long)]
        force: bool,
    },
    /// Delete an app reference from the workstation configuration
    Delete {
        /// The name of the app to delete
        name: String,
    },
    /// Show application status
    Status,
    /// Sync configurations from Git repositories
    Sync {
        /// The name of the app configuration to sync
        name: String,
    },
}

#[tokio::main]
async fn main() {
    // Load workstation configuration at startup
    // This will create an empty config file if it doesn't exist
    let workstation_config = match load_workstation_config().await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading workstation configuration: {e}");
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Add {
            name,
            git_url,
            local_path,
            sync_interval,
            kubectl_context,
            force,
        }) => {
            let cmd = AddCommand::new(name, git_url, local_path, sync_interval, kubectl_context, force);
            cmd.execute().await
        }
        Some(Commands::Delete { name }) => {
            let cmd = DeleteCommand::new(name);
            cmd.execute().await
        }
        Some(Commands::Status) => {
            let cmd = StatusCommand::new();
            cmd.execute().await
        }
        Some(Commands::Sync { name }) => {
            let cmd = SyncCommand::new(name, workstation_config);
            cmd.execute().await
        }
        None => {
            // No command provided, show help
            Cli::parse_from(vec!["roro-kube", "--help"]);
            return;
        }
    };

    // Handle command errors
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
