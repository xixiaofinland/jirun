mod commands;
mod config;
mod jira;
mod utils;

use clap::{Parser, Subcommand};
use commands::{handle_init, handle_subtask_command};
use config::JiraConfig;

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate JIRA sub-tasks from a template with a specified parent")]
struct Cli {
    /// Prevent actual sub-task creation (dry-run mode)
    #[arg(short = 'd', long = "dry-run", global = true)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Template {
        #[arg(short = 'p', long = "parent")]
        parent: String,

        #[arg(short = 'a', long = "assignee")]
        assignee: Option<String>,
    },
    New {
        #[arg(short = 'p', long = "parent")]
        parent: String,

        #[arg(short = 'a', long = "assignee")]
        assignee: Option<String>,
    },
    Init,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => handle_init()?,
        Commands::Template {
            parent,
            assignee: assignee_override,
        } => handle_subtask_command(
            parent,
            assignee_override.as_deref(),
            cli.dry_run,
            JiraConfig::template_task_list,
        )?,
        Commands::New {
            parent,
            assignee: assignee_override,
        } => handle_subtask_command(
            parent,
            assignee_override.as_deref(),
            cli.dry_run,
            JiraConfig::new_task_list,
        )?,
    }

    Ok(())
}
