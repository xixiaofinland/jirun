use jirun::{
    jira::{build_jira_payload, extract_project_key},
    test_support::load_sample_config,
};
use serde_json::json;

#[test]
fn test_build_jira_payload_fields_are_correct() {
    let config = load_sample_config();
    let payload = build_jira_payload(&config, "PROJ-123", "Test summary", None);
    let fields = &payload["fields"];

    assert_eq!(fields["project"]["key"], "PROJ");
    assert_eq!(fields["parent"]["key"], "PROJ-123");
    assert_eq!(fields["summary"], "Test summary");
    assert_eq!(fields["issuetype"]["name"], "Sub-task");
    assert_eq!(fields["labels"], json!(["cli", "auto"]));
    assert_eq!(fields["assignee"]["name"], "john.doe");

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
