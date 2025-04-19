use serde_json::to_string_pretty;
use std::{collections::HashMap, error::Error};

use crate::{
    config::JiraConfig,
    jira::build_jira_payload,
    utils::{bold_cyan, bold_white, bold_yellow, truncate_with_ellipsis},
};

pub struct TaskContext {
    pub config: JiraConfig,
    pub token: String,
    pub parent_key: String,
    pub parent_summary: String,
    pub existing_subtask_summaries: HashMap<String, String>,
    pub assignee: Option<String>,
    pub dry_run: bool,
}

impl TaskContext {
    pub fn new(
        parent_key: &str,
        assignee: Option<String>,
        dry_run: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let token = dotenvy::var("JIRA_TOKEN")?;
        let config = JiraConfig::load()?;
        let url = format!("{}/{}", config.api_url(), parent_key);

        let client = reqwest::blocking::Client::new();
        let res = client
            .get(&url)
            .bearer_auth(&token)
            .header("Accept", "application/json")
            .send()?;

        let json: serde_json::Value = res.json()?;

        let parent_summary = json["fields"]["summary"]
            .as_str()
            .unwrap_or("<unknown summary>")
            .to_string();

        let existing_subtask_summaries = json["fields"]["subtasks"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|s| {
                let summary = s["fields"]["summary"].as_str()?;
                let key = s["key"].as_str()?;
                Some((summary.to_string(), key.to_string()))
            })
            .collect();

        Ok(Self {
            config,
            token,
            parent_key: parent_key.to_string(),
            parent_summary,
            existing_subtask_summaries,
            assignee,
            dry_run,
        })
    }

    pub fn print_task_summary(&self, tasks: &[&String]) -> Result<(), Box<dyn Error>> {
        println!("\n{}", bold_yellow("Parent:"));
        println!("-----");
        println!(
            "🔗 {} — '{}'",
            self.parent_key,
            bold_cyan(&self.parent_summary)
        );

        println!("\n{}", bold_yellow("Tasks:"));
        println!("-----");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, bold_white(task));
        }

        println!("\n{}", bold_yellow("Prefill:"));
        println!("-----");

        if let Some(labels) = &self.config.prefill.labels {
            let joined = labels.join(", ");
            println!("🏷️  Labels: {joined}");
        }

        if let Some(name) = self
            .assignee
            .as_deref()
            .or(self.config.prefill.assignee.as_deref())
        {
            println!("👤 Assignee: {name}");
        } else {
            println!("👤 Assignee: (none)");
        }
        println!();

        Ok(())
    }

    pub fn print_dry_run_summary(&self, tasks: &[String]) -> Result<(), Box<dyn Error>> {
        println!("🔗 API: {}\n", self.config.api_url());

        for (i, summary) in tasks.iter().enumerate() {
            let display_summary = truncate_with_ellipsis(summary, 20);
            println!(
                "📦 Dry-run: would send this payload for sub-task #{}: '{}'",
                i + 1,
                display_summary
            );

            let body = build_jira_payload(
                &self.config,
                &self.parent_key,
                summary,
                self.assignee.as_deref(),
            );
            println!("{}\n", to_string_pretty(&body)?);
        }

        println!("🚫 Dry-run: no requests were sent.");
        Ok(())
    }

    pub fn filter_new_tasks<'a>(
        &self,
        tasks: &'a [String],
    ) -> (Vec<&'a String>, Vec<(&'a String, String)>) {
        let mut to_create = Vec::new();
        let mut duplicates: Vec<(&String, String)> = Vec::new();

        for task in tasks {
            if let Some(key) = self.existing_subtask_summaries.get(task) {
                duplicates.push((task, key.clone()));
            } else {
                to_create.push(task);
            }
        }

        (to_create, duplicates)
    }
}
