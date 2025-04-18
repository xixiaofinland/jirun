mod config;

use clap::{Parser, Subcommand};
use config::{init_config, load_config};
use std::env;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate JIRA sub-tasks from a template with a specified parent")]
struct Cli {
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if init_config()? {
                println!("✅ Created .jist.toml in current directory.");
            }
            return Ok(());
        }

        Commands::Template { parent } => {
            let _token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
            let config = load_config()?;
            let tasks = config.template_task_list();

            print_task_summary(&parent, &config, &tasks, "📄 Template Tasks")?;

            if !prompt_confirm()? {
                println!("❌ Aborted.");
                return Ok(());
            }

            // TODO: send JIRA API requests here
        }

        Commands::New { parent } => {
            let _token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
            let config = load_config()?;
            let tasks = config.new_task_list();

            print_task_summary(&parent, &config, &tasks, "🆕 New Tasks")?;

            if !prompt_confirm()? {
                println!("❌ Aborted.");
                return Ok(());
            }

            // TODO: send JIRA API requests here
        }
    }

    Ok(())
}

fn print_task_summary(
    parent: &str,
    config: &config::JiraConfig,
    tasks: &[String],
    title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧾 Sub-task preview");
    println!("🔗 Parent ticket: {parent}");

    if let Some(labels) = &config.prefill.labels {
        println!("🏷️  Labels: {:?}", labels);
    }

    if let Some(assignee) = &config.prefill.assignee {
        println!("👤 Assignee: {}", assignee);
    }

    println!("\n{title}:");
    for (i, task) in tasks.iter().enumerate() {
        println!("  {}. {}", i + 1, task);
    }

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

