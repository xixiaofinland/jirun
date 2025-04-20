use serde_json::Value;
use std::error::Error;

/// Trait for JIRA API behavior (for real or mocked clients)
#[allow(dead_code)]
pub trait JiraApi {
    fn fetch_parent_issue(&self, key: &str) -> Result<Value, Box<dyn Error>>;
    fn create_subtask(&self, payload: &Value) -> Result<String, Box<dyn Error>>;
}
