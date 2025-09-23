use crate::{JirunResult, jira::JiraApi};
use serde_json::{Value, json};

pub struct MockJiraApi;

impl JiraApi for MockJiraApi {
    fn fetch_parent_issue(&self, _key: &str) -> JirunResult<Value> {
        Ok(json!({
            "fields": {
                "summary": "Fake parent summary",
                "subtasks": [
                    {
                        "key": "FAKE-1",
                        "fields": {
                            "summary": "Existing Task A"
                        }
                    },
                    {
                        "key": "FAKE-2",
                        "fields": {
                            "summary": "Existing Task B"
                        }
                    }
                ]
            }
        }))
    }

    fn create_subtask(&self, _payload: &serde_json::Value) -> JirunResult<String> {
        Ok("FAKE-123".to_string())
    }
}
