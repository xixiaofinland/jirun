mod config;

use clap::{Parser, Subcommand};
use config::load_config;
use std::env;

#[derive(Parser)]
#[command(name = "Jirar")]
#[command(about = "A CLI tool to handle JIRA sub-tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create sub-tasks as defined in config
    Create {
        /// Parent issue key (e.g., PROJ-123)
        #[arg(short, long)]
        parent: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load JIRA token from environment
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");

    let config = load_config()?;

    match cli.command {
        Commands::Create { parent } => {
            println!("✅ JIRA_TOKEN loaded");
            println!("🔗 Parent issue: {parent}");
            println!("📄 Parsed config:\n{:#?}", config);

            println!("\n🔧 Defaults:");
            println!("- Project: {}", config.jira.project_key);
            println!("- Issue Type: {}", config.jira.issue_type);
            if let Some(labels) = &config.jira.labels {
                println!("- Labels: {:?}", labels);
            }

            println!("\n🧾 Sub-tasks:");
            for (i, subtask) in config.subtasks.iter().enumerate() {
                println!("{}. {}", i + 1, subtask.summary);
                println!("   {}", subtask.description);
            }
        }
    }

    Ok(())
}
