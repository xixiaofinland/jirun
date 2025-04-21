mod common;

use common::mock_api::MockJiraApi;
use jirun::jira::api::JiraApi;

#[test]
fn test_fetch_parent_issue() {
    let api = MockJiraApi;
    let issue = api.fetch_parent_issue("JIRA-123").unwrap();

    let summary = issue["fields"]["summary"]
        .as_str()
        .unwrap_or("<unknown summary>");
    assert_eq!(summary, "Fake parent summary");

    let subtasks = issue["fields"]["subtasks"].as_array().unwrap();
    assert_eq!(subtasks.len(), 2);
}
