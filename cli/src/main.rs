// CLI Application Layer Entry Point
//
// This is a command-line interface for Roro Kube.
// It provides a thin controller layer that delegates to the Core layer.

mod commands;

use clap::Parser;
use commands::{Command, StatusCommand};

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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Status) => {
            let cmd = StatusCommand::new();
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
