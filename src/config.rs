use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct JiraConfig {
    pub jira: JiraDefaults,
    pub subtasks: Vec<SubtaskEntry>,
}

#[derive(Debug, Deserialize)]
pub struct JiraDefaults {
    pub url: String,
    pub project_key: String,
    pub issue_type: String,
    pub labels: Option<Vec<String>>,
    pub assignee: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubtaskEntry {
    pub summary: String,
    pub description: String,
}

pub fn load_config() -> Result<JiraConfig, Box<dyn std::error::Error>> {
    let config_path: PathBuf = dirs::home_dir()
        .ok_or("❌ Could not determine home directory")?
        .join(".jirar.toml");

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("❌ Failed to read config file at {:?}: {}", config_path, e))?;

    let config: JiraConfig = toml::from_str(&content)
        .map_err(|e| format!("❌ Failed to parse TOML config: {}", e))?;

    Ok(config)
}
