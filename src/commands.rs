use crate::{config, jira, utils};
use std::env;

pub fn handle_template(
    parent: String,
    assignee_override: Option<&str>,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = config::load_config()?;
    let tasks = config.template_task_list();

    utils::print_task_summary(&parent, &config, &tasks, assignee_override)?;
    if !dry_run && !utils::prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    let mut dry_run_header_printed = false;
    for summary in &tasks {
        if let Err(err) = jira::send_subtask(
            &config,
            &token,
            &parent,
            summary,
            assignee_override,
            dry_run,
            &mut dry_run_header_printed,
        ) {
            eprintln!("{err}");
        }
    }

    if dry_run {
        println!("🚫 Dry-run: no requests were sent.");
    }

    Ok(())
}

pub fn handle_new(
    parent: String,
    assignee_override: Option<&str>,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = config::load_config()?;
    let tasks = config.new_task_list();

    utils::print_task_summary(&parent, &config, &tasks, assignee_override)?;
    if !dry_run || !utils::prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    let mut dry_run_header_printed = false;
    for summary in &tasks {
        if let Err(err) = jira::send_subtask(
            &config,
            &token,
            &parent,
            summary,
            assignee_override,
            dry_run,
            &mut dry_run_header_printed,
        ) {
            eprintln!("{err}");
        }
    }

    if dry_run {
        println!("🚫 Dry-run: no requests were sent.");
    }

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
