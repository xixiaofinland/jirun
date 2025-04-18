use crate::config::JiraConfig;
use std::io::{self, Write};

pub fn print_task_summary(
    parent: &str,
    config: &JiraConfig,
    tasks: &[String],
    assignee_override: Option<&str>,
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
    if let Some(assignee) = assignee_override.or(config.prefill.assignee.as_deref()) {
        println!("👤 Assignee: {assignee} (effective)");
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

pub fn prompt_confirm() -> Result<bool, Box<dyn std::error::Error>> {
    print!("\n✅ Proceed with creating these sub-tasks? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let answer = input.trim().to_lowercase();

    Ok(matches!(answer.as_str(), "y" | "yes"))
}
