use crate::{config::JiraConfig, jira::send_subtask};
use jirun::{
    task_context::TaskContext,
    utils::{bold_cyan, bold_white},
};
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
    let config = JiraConfig::load()?;
    let tasks = select_tasks(&config);
    let ctx = TaskContext::new(&parent, assignee.map(str::to_string), dry_run)?;

    let (to_create, duplicates) = ctx.filter_new_tasks(&tasks);

    if !duplicates.is_empty() {
        println!("⚠️  Skipped {} duplicate task(s):", duplicates.len());
        for (summary, key) in duplicates {
            println!("• {} ({})", bold_white(summary), bold_cyan(&key));
        }
        println!();
    }

    ctx.print_task_summary(&to_create)?;

    if dry_run {
        ctx.print_dry_run_summary(&tasks)?;
        return Ok(());
    }

    if !prompt_confirm()? {
        println!("❌ Aborted.");
        return Ok(());
    }

    for summary in &tasks {
        if let Err(err) = send_subtask(&config, &ctx.token, &parent, summary, assignee) {
            eprintln!("{err}");
        }
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

fn prompt_confirm() -> Result<bool, Box<dyn Error>> {
    print!("\n✅ Proceed with creating these sub-tasks? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
