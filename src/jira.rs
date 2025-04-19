use crate::config::JiraConfig;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Map, Value};

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

    let client = Client::new();
    let res = client
        .post(&url)
        .bearer_auth(token)
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()?;

    let status = res.status();
    if status.is_success() {
        let json: Value = res.json()?;
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
    let mut fields = Map::new();

    fields.insert("summary".into(), json!(summary));
    fields.insert(
        "project".into(),
        json!({ "key": extract_project_key(parent_key) }),
    );
    fields.insert("parent".into(), json!({ "key": parent_key }));
    fields.insert("issuetype".into(), json!({ "name": "Sub-task" }));

    if let Some(labels) = &config.prefill.labels {
        fields.insert("labels".into(), json!(labels));
    }

    if let Some(assignee) = assignee_override.or(config.prefill.assignee.as_deref()) {
        fields.insert("assignee".into(), json!({ "name": assignee }));
    }

    json!({ "fields": Value::Object(fields) })
}

pub fn extract_project_key(parent_key: &str) -> &str {
    parent_key
        .split('-')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("UNKNOWN")
}
