use crate::{config, jira, utils};
use std::env;

pub fn handle_template(parent: String, diagnose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = config::load_config()?;
    let tasks = config.template_task_list();

    utils::print_task_summary(&parent, &config, &tasks)?;
    if !utils::prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &tasks {
        if let Err(err) = jira::send_subtask(&config, &token, &parent, summary, diagnose) {
            eprintln!("{err}");
        }
    }

    Ok(())
}

pub fn handle_new(parent: String, diagnose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN environment variable must be set");
    let config = config::load_config()?;
    let tasks = config.new_task_list();

    utils::print_task_summary(&parent, &config, &tasks)?;
    if !utils::prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &tasks {
        if let Err(err) = jira::send_subtask(&config, &token, &parent, summary, diagnose) {
            eprintln!("{err}");
        }
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
