use crate::{jira::api::JiraApi, JirunResult};
use serde_json::Value;

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
    fn fetch_parent_issue(&self, parent_key: &str) -> JirunResult<Value> {
        let url = format!("{}/{}", self.url, parent_key);
        let res = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .send()?;

        let status = res.status();
        if !status.is_success() {
            let body = res
                .text()
                .unwrap_or_else(|_| "<failed to read body>".into());
            return Err(format!(
                "❌ Failed to fetch issue {}: HTTP {} – {}",
                parent_key, status, body
            )
            .into());
        }

        let json: Value = res.json()?;
        Ok(json)
    }

    fn create_subtask(&self, payload: &Value) -> JirunResult<String> {
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
