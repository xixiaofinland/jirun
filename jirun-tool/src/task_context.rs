use crate::{JirunResult, config::JiraConfig, jira::JiraApi, utils::*};
use serde_json::to_string_pretty;
use std::collections::HashMap;

pub struct TaskContext {
    pub config: JiraConfig,
    pub api: Box<dyn JiraApi>,
    pub parent_key: String,
    pub parent_summary: String,
    pub existing_subtask_summaries: HashMap<String, String>,
    pub assignee: Option<String>,
}

impl TaskContext {
    pub fn new(
        config: JiraConfig,
        api: Box<dyn JiraApi>,
        parent_key: &str,
        assignee: Option<String>,
    ) -> JirunResult<Self> {
        let json = api.fetch_parent_issue(parent_key)?;

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
            api,
            parent_key: parent_key.to_string(),
            parent_summary,
            existing_subtask_summaries,
            assignee,
        })
    }

    pub fn print_task_summary(
        &self,
        original: &[String],
        duplicates: &[(String, String)],
    ) -> JirunResult<()> {
        let duplicates: HashMap<_, _> = duplicates.iter().cloned().collect();

        println!("{}", bold_yellow("Parent:"));
        print_line_separator();
        println!(
            "{} — '{}'",
            self.issue_link(&self.parent_key),
            bold_cyan(&self.parent_summary)
        );

        println!("\n{}", bold_yellow("Prefill:"));
        print_line_separator();

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

        println!("\n{}", bold_yellow("Sub-tasks to create:"));
        print_line_separator();

        for (i, task) in original.iter().enumerate() {
            if let Some(existing_key) = duplicates.get(task) {
                println!(
                    "{}. '{}' — {}",
                    i + 1,
                    bold_white(task),
                    red(&format!(
                        "skipped (identical title found in {})",
                        self.issue_link(existing_key)
                    ))
                );
            } else {
                println!("{}. '{}'", i + 1, bold_white(task));
            }
        }

        Ok(())
    }

    pub fn print_dry_run_summary(&self, to_create: &[String]) -> JirunResult<()> {
        println!("🔗 API: {}\n", self.config.api_url());

        for (i, summary) in to_create.iter().enumerate() {
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

    pub fn filter_new_tasks(&self, tasks: &[String]) -> (Vec<String>, Vec<(String, String)>) {
        let mut to_create = Vec::new();
        let mut duplicates = Vec::new();

        for task in tasks {
            let task_lower = task.to_lowercase();

            let maybe_duplicate = self
                .existing_subtask_summaries
                .iter()
                .find(|(summary, _)| summary.to_lowercase() == task_lower);

            if let Some((_, key)) = maybe_duplicate {
                duplicates.push((task.clone(), key.clone()));
            } else {
                to_create.push(task.clone());
            }
        }
        (to_create, duplicates)
    }

    pub fn issue_link(&self, issue_key: &str) -> String {
        let url = self.issue_url(issue_key);
        let link = hyperlink(issue_key, &url);
        format!("🔗 {}", link)
    }

    fn issue_url(&self, issue_key: &str) -> String {
        format!(
            "{}/browse/{}",
            self.config.server.url.trim_end_matches('/'),
            issue_key
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{mock_api::MockJiraApi, test_helper::load_test_config};

    #[test]
    fn test_fetch_parent_summary_and_subtasks() {
        let api = MockJiraApi;
        let issue = api.fetch_parent_issue("JIRA-123").unwrap();

        let summary = issue["fields"]["summary"]
            .as_str()
            .unwrap_or("<unknown summary>");
        assert_eq!(summary, "Fake parent summary");

        let subtasks = issue["fields"]["subtasks"].as_array().unwrap();
        assert_eq!(subtasks.len(), 2);

        let summaries: Vec<_> = subtasks
            .iter()
            .map(|s| s["fields"]["summary"].as_str().unwrap_or(""))
            .collect();

        assert_eq!(summaries, vec!["Existing Task A", "Existing Task B"]);
    }

    #[test]
    fn test_filter_new_tasks_excludes_duplicates() {
        let config = load_test_config();
        let mock_api = Box::new(MockJiraApi);
        let context = TaskContext::new(config, mock_api, "JIRA-123", None).unwrap();

        // Input tasks: one duplicate, one unique
        let input = vec![
            "Existing Task A".to_string(),
            "Totally New Task".to_string(),
        ];

        let (to_create, duplicates) = context.filter_new_tasks(&input);

        assert_eq!(to_create, vec!["Totally New Task"]);

        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].0, "Existing Task A");
        assert_eq!(duplicates[0].1, "FAKE-1");
    }
}
