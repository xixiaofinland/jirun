use std::env;

use reqwest::blocking::Client;
use serde_json::{json, to_string_pretty};

use crate::config;

pub fn send_subtask(
    config: &config::JiraConfig,
    token: &str,
    parent_key: &str,
    summary: &str,
    diagnose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_key = parent_key
        .split('-')
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("❌ Invalid parent issue key: '{parent_key}'"))?;

    let mut fields = json!({
        "project": { "key": project_key },
        "parent": { "key": parent_key },
        "summary": summary,
        "issuetype": { "name": "Sub-task" }
    });

    if let Some(labels) = &config.prefill.labels {
        fields["labels"] = serde_json::Value::Array(
            labels
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect(),
        );
    }

    if let Some(assignee) = &config.prefill.assignee {
        fields["assignee"] = json!({ "name": assignee });
    }

    let body = json!({ "fields": fields });

    let url = format!(
        "{}/rest/api/2/issue/",
        config.server.url.trim_end_matches('/')
    );

    if diagnose {
        println!();
        match env::var("JIRA_TOKEN") {
            Ok(_) => println!("🔐 Found JIRA_TOKEN in environment"),
            Err(_) => println!("⚠️  JIRA_TOKEN not set"),
        }

        println!("\n🌐 JIRA endpoint: {url}");
        println!(
            "📦 Constructed JIRA JSON payload:\n{}",
            to_string_pretty(&body)?
        );
    }

    // let res = client
    //     .post(&url)
    //     .bearer_auth(token)
    //     .header("Content-Type", "application/json")
    //     .json(&body)
    //     .send()?;
    //
    // let status = res.status();
    // if status.is_success() {
    //     let json: serde_json::Value = res.json()?;
    // let key = json["key"]
    //     .as_str()
    //     .ok_or_else(|| format!("❌ JIRA response missing 'key':\n{}", json))?;
    //
    //     println!("✅ Created sub-task: {key}");
    //     Ok(())
    // } else {
    //     let status = res.status();
    //     let err_text = res.text().unwrap_or_default();
    //     Err(format!("❌ Failed to create sub-task: {status}\n{err_text}").into())
    // }
    Ok(())
}
