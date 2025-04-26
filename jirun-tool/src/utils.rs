use crate::config::JiraConfig;
use serde_json::{Map, Value, json};

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

/// Wraps `text` in an OSC 8 hyperlink to `url`.
/// Most modern terminals will render it clickable; older ones will show the raw escape codes.
pub fn hyperlink(text: &str, url: &str) -> String {
    // OSC 8 ; ; URL BEL text OSC 8 ; ; BEL
    format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, text)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_helper::load_test_config;
    use serde_json::json;

    #[test]
    fn test_build_jira_payload_fields_are_correct() {
        let config = load_test_config();
        let payload = build_jira_payload(&config, "PROJ-123", "Test summary", None);
        let fields = &payload["fields"];

        assert_eq!(fields["project"]["key"], "PROJ");
        assert_eq!(fields["parent"]["key"], "PROJ-123");
        assert_eq!(fields["summary"], "Test summary");
        assert_eq!(fields["issuetype"]["name"], "Sub-task");
        assert_eq!(fields["labels"], json!(["cli", "auto"]));
        assert_eq!(fields["assignee"]["name"], "john.doe");

        // override assignee
        let payload = build_jira_payload(&config, "PROJ-123", "summary", Some("override_user"));
        assert_eq!(payload["fields"]["assignee"]["name"], "override_user");
    }

    #[test]
    fn test_extract_project_key() {
        assert_eq!(extract_project_key("PROJ-123"), "PROJ");
        assert_eq!(extract_project_key("ABC-9999"), "ABC");
        assert_eq!(extract_project_key("X-1"), "X");
        assert_eq!(extract_project_key("INVALID"), "INVALID");
        assert_eq!(extract_project_key(""), "UNKNOWN");
    }
}
