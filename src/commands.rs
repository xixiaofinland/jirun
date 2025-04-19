use crate::{
    config::JiraConfig,
    jira::{build_jira_payload, send_subtask},
};
use reqwest::blocking::Client;
use serde_json::{to_string_pretty, Value};
use std::{
    error::Error,
    io::{self, Write},
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
    let config = JiraConfig::load()?;
    let tasks = select_tasks(&config);

    print_task_summary(&parent, &config, &tasks, assignee, &token)?;

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
    println!("🔗 API: {}\n", config.get_api_url());

    for (i, summary) in tasks.iter().enumerate() {
        let display_summary = truncate_with_ellipsis(summary, 20);
        println!(
            "📦 Dry-run: would send this payload for sub-task #{}: '{}'",
            i + 1,
            display_summary
        );

        let body = build_jira_payload(config, parent, summary, assignee_override);
        println!("{}\n", to_string_pretty(&body)?);
    }

    println!("🚫 Dry-run: no requests were sent.");
    Ok(())
}

pub fn handle_init(global: bool) {
    if global {
        JiraConfig::init_global()
    } else {
        JiraConfig::init_local()
    }
}

fn print_task_summary(
    parent: &str,
    config: &JiraConfig,
    tasks: &[String],
    assignee: Option<&str>,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let parent_url = format!("{}/{}", config.get_api_url(), parent);

    let res = client
        .get(&parent_url)
        .bearer_auth(token)
        .header("Accept", "application/json")
        .send()?;

    let json: Value = res.json()?;
    let parent_summary = json["fields"]["summary"]
        .as_str()
        .unwrap_or("<unknown summary>");
    let parent_summary = truncate_with_ellipsis(parent_summary, 50);

    println!("\n{}", bold_yellow("Parent:"));
    println!("-----");
    println!("🔗 {} — '{}'", parent, bold_cyan(&parent_summary));

    println!("\n{}", bold_yellow("Tasks:"));
    println!("-----");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, task);
    }

    println!("\n{}", bold_yellow("Prefill:"));
    println!("-----");

    if let Some(labels) = &config.prefill.labels {
        let joined = labels.join(", ");
        println!("🏷️  Labels: {joined}");
    }

    if let Some(name) = assignee.or(config.prefill.assignee.as_deref()) {
        println!("👤 Assignee: {name}");
    } else {
        println!("👤 Assignee: (none)");
    }
    println!();

    Ok(())
}

fn prompt_confirm() -> Result<bool, Box<dyn Error>> {
    print!("\n✅ Proceed with creating these sub-tasks? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}

fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{}...", truncated)
    } else {
        truncated
    }
}

fn bold_yellow(text: &str) -> String {
    format!("\x1b[1;33m{}\x1b[0m", text)
}

fn bold_cyan(text: &str) -> String {
    format!("\x1b[1;36m{}\x1b[0m", text)
}
