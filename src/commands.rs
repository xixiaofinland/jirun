use crate::{
    config::{self, JiraConfig},
    jira::{self, build_jira_payload},
    utils,
};
use std::env;

pub fn handle_subtask_command<F>(
    parent: String,
    assignee_override: Option<&str>,
    dry_run: bool,
    select_tasks: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(&JiraConfig) -> Vec<String>,
{
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = config::load_config()?;
    let tasks = select_tasks(&config);

    utils::print_task_summary(&parent, &config, &tasks, assignee_override)?;

    if dry_run {
        print_dry_run_summary(&config, &parent, &tasks, assignee_override)?;
        return Ok(());
    }

    if !utils::prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &tasks {
        if let Err(err) = jira::send_subtask(&config, &token, &parent, summary, assignee_override) {
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
) -> Result<(), Box<dyn std::error::Error>> {
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
        println!("{}", serde_json::to_string_pretty(&body)?);
        println!();
    }

    println!("🚫 Dry-run: no requests were sent.");
    Ok(())
}

pub fn handle_init() -> Result<(), Box<dyn std::error::Error>> {
    if config::init_config()? {
        println!("\n✨ Init complete.");
    } else {
        println!("\nℹ️ Nothing to do — config and .env already exist.");
    }
    Ok(())
}
