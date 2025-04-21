use crate::jira::JiraApi;
use serde_json::json;
use std::error::Error;

pub struct MockJiraApi;

impl JiraApi for MockJiraApi {
    fn fetch_parent_issue(&self, _key: &str) -> Result<serde_json::Value, Box<dyn Error>> {
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

    fn create_subtask(&self, _payload: &serde_json::Value) -> Result<String, Box<dyn Error>> {
        Ok("FAKE-123".to_string())
    }
}
