use serde_json::Value;

use crate::JirunResult;

/// Trait for JIRA API behavior (for real or mocked clients)
#[allow(dead_code)]
pub trait JiraApi {
    fn fetch_parent_issue(&self, key: &str) -> JirunResult<Value>;
    fn create_subtask(&self, payload: &Value) -> JirunResult<String>;
}
