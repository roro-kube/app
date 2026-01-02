// CLI Application Layer Entry Point
//
// This is a command-line interface for Roro Kube.
// It provides a thin controller layer that delegates to the Core layer.

mod commands;

use clap::Parser;
use commands::{Command, StatusCommand, SyncCommand};

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
    /// Show application status
    Status,
    /// Sync workspace configurations from Git repositories
    Sync {
        /// The remote repository URL (SSH or HTTPS)
        #[arg(long)]
        url: String,
        /// The local path where the repository should be synced
        #[arg(long)]
        path: std::path::PathBuf,
        /// Username for authentication (optional)
        #[arg(long)]
        username: Option<String>,
        /// Password or token for authentication (optional)
        #[arg(long)]
        password: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Status) => {
            let cmd = StatusCommand::new();
            cmd.execute().await
        }
        Some(Commands::Sync {
            url,
            path,
            username,
            password,
        }) => {
            let cmd = SyncCommand::new(url, path, username, password);
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
