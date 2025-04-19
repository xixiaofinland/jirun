use serde_json::to_string_pretty;
use std::{
    error::Error,
    io::{self, Write},
};

use crate::{
    config::JiraConfig,
    jira::{build_jira_payload, send_subtask},
};

pub fn handle_subtask_command<F>(
    parent: String,
    assignee: Option<&str>,
    dry_run: bool,
    select_tasks: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&JiraConfig) -> Vec<String>,
{
    let token = dotenvy::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = JiraConfig::load_config()?;
    let tasks = select_tasks(&config);

    print_task_summary(&parent, &config, &tasks, assignee)?;

    if dry_run {
        print_dry_run_summary(&config, &parent, &tasks, assignee)?;
        return Ok(());
    }

    if !prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &tasks {
        if let Err(err) = send_subtask(&config, &token, &parent, summary, assignee) {
            eprintln!("{err}");
        }
    }

    Ok(())
}

fn print_dry_run_summary(
    config: &JiraConfig,
    parent: &str,
    tasks: &[String],
    assignee_override: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    println!("🌐 Server: {}", config.server.url);
    println!(
        "🔗 API: {}/rest/api/2/issue/",
        config.server.url.trim_end_matches('/')
    );
    println!();

    for summary in tasks {
        let body = build_jira_payload(config, parent, summary, assignee_override);
        println!(
            "📦 Dry-run: would send this payload for sub-task '{}':",
            summary
        );
        println!("{}", to_string_pretty(&body)?);
        println!();
    }

    println!("🚫 Dry-run: no requests were sent.");
    Ok(())
}

pub fn handle_init() -> Result<(), Box<dyn Error>> {
    if JiraConfig::init()? {
        println!("\n✨ Init complete.");
    } else {
        println!("\nℹ️ Nothing to do — config and .env already exist.");
    }
    Ok(())
}

fn print_task_summary(
    parent: &str,
    config: &JiraConfig,
    tasks: &[String],
    assignee: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("-----");
    println!("Parent: {parent}\n");

    println!("Prefill:");
    println!("-----");
    if let Some(labels) = &config.prefill.labels {
        for label in labels {
            println!("🏷️  Label: {label}");
        }
    }
    if let Some(assignee) = assignee.or(config.prefill.assignee.as_deref()) {
        println!("👤 Assignee: {assignee}");
    } else {
        println!("👤 Assignee: (none)");
    }

    println!("\nTasks:");
    println!("-----");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task);
    }
    println!();

    Ok(())
}

fn prompt_confirm() -> Result<bool, Box<dyn std::error::Error>> {
    print!("\n✅ Proceed with creating these sub-tasks? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
