use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(name = "jira-cli")]
#[command(about = "A CLI tool to create JIRA sub-tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a JIRA sub-task
    Create {
        /// Parent issue key (e.g., PROJ-123)
        #[arg(short, long)]
        parent: String,

        /// Summary for the sub-task
        #[arg(short, long)]
        summary: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load JIRA token from environment
    let token = env::var("JIRA_TOKEN")
        .expect("JIRA_TOKEN environment variable must be set");

    match cli.command {
        Commands::Create { parent, summary } => {
            println!("Creating sub-task under {parent} with summary '{summary}'");
            // You’ll later pass `token` here to the HTTP client
        }
    }

    Ok(())
}

