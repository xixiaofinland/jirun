use std::{
    error::Error,
    io::{self, Write},
};

use clap::{Parser, Subcommand};

use crate::{
    env,
    task_context::TaskContext,
    utils::{build_jira_payload, print_line_separator},
};

use crate::{config::JiraConfig, jira::RealJiraApi};

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = concat!("✨ generates JIRA sub-task(s) with template (v.", env!("CARGO_PKG_VERSION"), ")"),
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Load .env once for everyone
    env::try_load_dotenv();

    // 2) Parse CLI
    let cli = crate::commands::Cli::parse();

    // 3) Dispatch
    match cli.command {
        Commands::Init { global } => handle_init(global),
        Commands::Template {
            parent,
            assignee,
            dry_run,
        } => handle_template_command(parent, assignee.as_deref(), dry_run)?,
        crate::commands::Commands::New {
            parent,
            assignee,
            dry_run,
        } => handle_new_command(parent, assignee.as_deref(), dry_run)?,
    }

    Ok(())
}

pub fn handle_init(global: bool) {
    if global {
        JiraConfig::init_global()
    } else {
        JiraConfig::init_local()
    }
}

pub fn handle_template_command(
    parent: String,
    assignee: Option<&str>,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    handle_subtask_command(parent, assignee, dry_run, JiraConfig::template_task_list)
}

pub fn handle_new_command(
    parent: String,
    assignee: Option<&str>,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    handle_subtask_command(parent, assignee, dry_run, JiraConfig::new_task_list)
}

fn handle_subtask_command<F>(
    parent: String,
    assignee: Option<&str>,
    dry_run: bool,
    select_tasks: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&JiraConfig) -> Vec<String>,
{
    let token = dotenvy::var("JIRA_TOKEN")?;
    let config = JiraConfig::load()?;
    let tasks = select_tasks(&config);
    let api = RealJiraApi::new(config.api_url(), token);
    let ctx = TaskContext::new(
        config,
        Box::new(api),
        &parent,
        assignee.map(str::to_string),
        dry_run,
    )?;

    let (to_create, duplicates) = ctx.filter_new_tasks(&tasks);
    ctx.print_task_summary(&tasks, &duplicates)?;

    if dry_run {
        ctx.print_dry_run_summary(&to_create)?;
        return Ok(());
    }

    if to_create.is_empty() {
        print_line_separator();
        println!("⚠️  No new tasks to create. Terminating...");
        return Ok(());
    }

    if !prompt_confirm(to_create.len())? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &to_create {
        let payload = build_jira_payload(
            &ctx.config,
            &ctx.parent_key,
            summary,
            ctx.assignee.as_deref(),
        );
        let key = ctx.api.create_subtask(&payload)?;
        println!("✅ Created sub-task: {key}");
    }

    Ok(())
}

fn prompt_confirm(size: usize) -> Result<bool, Box<dyn Error>> {
    print!("\n✅ {} sub-task(s) to create, proceed? [y/N]: ", size);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
