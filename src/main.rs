mod config;

use clap::{Parser, Subcommand};
use config::{init_config, load_config};
use std::env;

#[derive(Parser)]
#[command(name = "jist")]
#[command(about = "Generate JIRA sub-tasks from a template with a specified parent")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create sub-tasks from template_tasks
    Template {
        #[arg(short, long)]
        parent: String,
    },

    /// Create sub-tasks from new_tasks
    New {
        #[arg(short, long)]
        parent: String,
    },

    /// Generate a default .jist.toml config file
    Init,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = load_config()?;
    match cli.command {
        Commands::Init => {
            init_config()?;
            println!("✅ Created .jist.toml in current directory.");
        }
        Commands::Template { parent } => {
            println!("🔗 Parent issue: {parent}");
            println!("🧩 Server: {}", config.server.url);
            println!("⚙️  Prefill: {:?}", config.prefill);
            println!("\n📄 Template Tasks:");
            for (i, task) in config.template_task_list().iter().enumerate() {
                println!("{}. {}", i + 1, task);
            }
        }
        Commands::New { parent } => {
            println!("🔗 Parent issue: {parent}");
            println!("🧩 Server: {}", config.server.url);
            println!("⚙️  Prefill: {:?}", config.prefill);
            println!("\n🆕 New Tasks:");
            for (i, task) in config.new_task_list().iter().enumerate() {
                println!("{}. {}", i + 1, task);
            }
        }
    }
    Ok(())
}

