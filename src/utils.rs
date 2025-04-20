use crate::config::JiraConfig;
use serde_json::{json, Map, Value};

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

pub fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();

    if chars.next().is_some() {
        format!("{}...", truncated)
    } else {
        truncated
    }
}

pub fn bold_yellow(text: &str) -> String {
    format!("\x1b[1;33m{}\x1b[0m", text)
}

pub fn bold_cyan(text: &str) -> String {
    format!("\x1b[1;36m{}\x1b[0m", text)
}

pub fn bold_white(text: &str) -> String {
    format!("\x1b[1;97m{}\x1b[0m", text)
}

pub fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

pub fn print_line_separator() {
    println!("-----");
}
