mod common;

use common::mock_api::MockJiraApi;
use jirun::{jira::api::JiraApi, task_context::TaskContext};

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
    let mock_api = Box::new(MockJiraApi);
    let context = TaskContext::new(mock_api, "JIRA-123", None, false).unwrap();

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
