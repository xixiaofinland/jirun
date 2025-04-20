use crate::{config::JiraConfig, jira::send_subtask};
use jirun::{task_context::TaskContext, utils::print_line_separator};
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

fn prompt_confirm(size: usize) -> Result<bool, Box<dyn Error>> {
    print!("\n✅ {} sub-task(s) to create, proceed? [y/N]: ", size);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
