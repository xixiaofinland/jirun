use crate::jira::api::JiraApi;
use serde_json::Value;
use std::error::Error;

pub struct RealJiraApi {
    pub url: String,
    pub token: String,
    client: reqwest::blocking::Client,
}

impl RealJiraApi {
    #[allow(dead_code)]
    pub fn new(url: String, token: String) -> Self {
        Self {
            url,
            token,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl JiraApi for RealJiraApi {
    fn fetch_parent_issue(&self, parent_key: &str) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/{}", self.url, parent_key);
        let res = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .send()?;
        Ok(res.json()?)
    }

    fn create_subtask(&self, payload: &Value) -> Result<String, Box<dyn Error>> {
        let res = self
            .client
            .post(&self.url)
            .bearer_auth(&self.token)
            .header("Content-Type", "application/json")
            .json(payload)
            .send()?;

        let status = res.status();
        if status.is_success() {
            let json: Value = res.json()?;
            let key = json["key"]
                .as_str()
                .ok_or("Missing 'key' in JIRA response")?
                .to_string();
            Ok(key)
        } else {
            Err(format!("JIRA error: {} - {}", status, res.text()?).into())
        }
    }
}
