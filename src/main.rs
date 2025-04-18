mod config;
mod jira;

use clap::{Parser, Subcommand};
use config::{init_config, load_config};
use jira::send_subtask;
use std::env;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate JIRA sub-tasks from a template with a specified parent")]
struct Cli {
    /// Print outgoing JIRA JSON payloads for debugging
    #[arg(short = 'd', long = "diagnose", global = true)]
    diagnose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create sub-tasks from the [sub_tasks.template_tasks] section of the config
    Template {
        #[arg(short = 'p', long = "parent")]
        parent: String,
    },

    /// Create sub-tasks from the [sub_tasks.new_tasks] section of the config
    New {
        #[arg(short = 'p', long = "parent")]
        parent: String,
    },

    /// Generate a default .jist.toml config file
    Init,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if init_config()? {
                println!("✅ Created .jirun.toml in current directory.");
            }
            return Ok(());
        }

        Commands::Template { parent } => {
            let _token =
                env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");

            let config = load_config()?;
            let tasks = config.template_task_list();

            print_task_summary(&parent, &config, &tasks)?;
            if !prompt_confirm()? {
                println!("❌ Aborted.");
                return Ok(());
            }

            for summary in &tasks {
                if let Err(err) = send_subtask(&config, &_token, &parent, summary, cli.diagnose) {
                    eprintln!("{err}");
                }
            }
        }

        Commands::New { parent } => {
            let _token =
                env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");

            let config = load_config()?;
            let tasks = config.new_task_list();

            print_task_summary(&parent, &config, &tasks)?;
            if !prompt_confirm()? {
                println!("❌ Aborted.");
                return Ok(());
            }

            for summary in &tasks {
                if let Err(err) = send_subtask(&config, &_token, &parent, summary, cli.diagnose) {
                    eprintln!("{err}");
                }
            }
        }
    }

    Ok(())
}

fn print_task_summary(
    parent: &str,
    config: &config::JiraConfig,
    tasks: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("-----");
    println!("Parent: {parent}");
    println!();

    println!("Prefill:");
    println!("-----");
    if let Some(labels) = &config.prefill.labels {
        for label in labels {
            println!("🏷️  Label: {label}");
        }
    }
    if let Some(assignee) = &config.prefill.assignee {
        println!("👤 Assignee: {assignee}");
    }
    println!();

    println!("Tasks:");
    println!("-----");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task);
    }
    println!();

    Ok(())
}

fn prompt_confirm() -> Result<bool, Box<dyn std::error::Error>> {
    print!("\n✅ Proceed with creating these sub-tasks? [y/N]: ");
    io::stdout().flush()?; // flush prompt before reading

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
