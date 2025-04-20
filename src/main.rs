mod commands;
mod config;
mod env;
mod jira;

use std::error::Error;

use clap::{Parser, Subcommand};
use commands::{handle_init, handle_subtask_command};
use config::JiraConfig;

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "✨ Generate JIRA sub-tasks from a template with a specified parent ticket.",
    long_about = None,
    after_help = "\
📘 Examples:
  1. jirun help init
     Help menu on initializing jirun's configuration files.

  2. jirun init --global
     Create config files in the global directory.

  3. jirun template --parent PROJ-123
     Use [sub_tasks.template_tasks] to create sub-tasks under PROJ-123

  4. jirun new --parent PROJ-123 --assignee alice
     Use [sub_tasks.new_tasks], overriding assignee with 'alice'

  5. jirun template -p PROJ-123 --dry-run
     Show request payloads without sending to JIRA"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create .jirun.toml and .env (defaults to the local directory)
    Init {
        /// Write to global config directory (Linux: ~/.config/jirun/, macOS: ~/Library/Application Support/jirun/, Windows: %APPDATA%\jirun\)
        #[arg(long = "global")]
        global: bool,
    },

    /// Create sub-tasks from [sub_tasks.new_tasks] in .jirun.toml
    New {
        /// Parent JIRA issue key (e.g. PROJ-123)
        #[arg(short = 'p', long = "parent")]
        parent: String,

        /// Override default assignee
        #[arg(short = 'a', long = "assignee")]
        assignee: Option<String>,

        /// Prevent actual sub-task creation (dry-run mode)
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
    },

    /// Create sub-tasks from [sub_tasks.template_tasks]
    Template {
        /// Parent JIRA issue key (e.g. PROJ-123)
        #[arg(short = 'p', long = "parent")]
        parent: String,

        /// Override default assignee
        #[arg(short = 'a', long = "assignee")]
        assignee: Option<String>,

        /// Prevent actual sub-task creation (dry-run mode)
        #[arg(short = 'd', long = "dry-run")]
        dry_run: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    env::try_load_dotenv();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { global } => handle_init(global),
        Commands::Template {
            parent,
            assignee,
            dry_run,
        } => handle_subtask_command(
            parent,
            assignee.as_deref(),
            dry_run,
            JiraConfig::template_task_list,
        )?,
        Commands::New {
            parent,
            assignee,
            dry_run,
        } => handle_subtask_command(
            parent,
            assignee.as_deref(),
            dry_run,
            JiraConfig::new_task_list,
        )?,
    }

    Ok(())
}
