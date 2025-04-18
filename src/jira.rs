// use reqwest::blocking::Client;
use serde_json::{json, Value};

use crate::config::JiraConfig;

pub fn send_subtask(
    config: &JiraConfig,
    token: &str,
    parent_key: &str,
    summary: &str,
    assignee_override: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = build_jira_payload(config, parent_key, summary, assignee_override);

    let url = format!(
        "{}/rest/api/2/issue/",
        config.server.url.trim_end_matches('/')
    );
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(&url)
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    let status = res.status();
    if status.is_success() {
        let json: serde_json::Value = res.json()?;
        let key = json["key"]
            .as_str()
            .ok_or_else(|| format!("❌ JIRA response missing 'key':\n{}", json))?;
        println!("✅ Created sub-task: {key}");
        Ok(())
    } else {
        let err_text = res.text().unwrap_or_default();
        Err(format!("❌ Failed to create sub-task: {status}\n{err_text}").into())
    }
}

pub fn build_jira_payload(
    config: &JiraConfig,
    parent_key: &str,
    summary: &str,
    assignee_override: Option<&str>,
) -> Value {
    let project_key = extract_project_key(parent_key);

    let mut fields = json!({
        "project": { "key": project_key },
        "parent": { "key": parent_key },
        "summary": summary,
        "issuetype": { "name": "Sub-task" }
    });

    if let Some(labels) = &config.prefill.labels {
        fields["labels"] = json!(labels);
    }

    if let Some(assignee) = assignee_override.or(config.prefill.assignee.as_deref()) {
        fields["assignee"] = json!({ "name": assignee });
    }

    json!({ "fields": fields })
}

pub fn extract_project_key(parent_key: &str) -> &str {
    parent_key
        .split('-')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("UNKNOWN")
}
