use crate::config::JiraConfig;
use std::io::{self, Write};

pub fn print_task_summary(
    parent: &str,
    config: &JiraConfig,
    tasks: &[String],
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
    if let Some(assignee) = &config.prefill.assignee {
        println!("👤 Assignee: {assignee}");
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

