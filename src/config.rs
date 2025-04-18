use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"
[server]
url = "https://yourcompany.atlassian.net"

[prefill]
labels = ["cli"]
assignee = "john.doe"

[sub_tasks]
template_tasks = """
Set up database schema
Implement service logic
Write integration tests
"""

new_tasks = """
Fix login bug
Refactor error handling
Document API usage
"""
"#;

pub fn init_config() -> Result<bool, Box<dyn std::error::Error>> {
    let config_path = Path::new(".jirun.toml");
    let env_path = Path::new(".env");

    let mut created_any = false;

    if config_path.exists() {
        println!("⚠️  .jirun.toml already exists. Not overwriting.");
    } else {
        fs::write(config_path, DEFAULT_CONFIG)?;
        println!("✅ Created .jirun.toml");
        created_any = true;
    }

    if env_path.exists() {
        println!("⚠️  .env already exists. Not overwriting.");
    } else {
        fs::write(
            env_path,
            "# JIRA API token (used by jirun)\nJIRA_TOKEN=your-api-token-here\n",
        )?;
        println!("✅ Created .env (with placeholder JIRA_TOKEN)");
        created_any = true;
    }

    Ok(created_any)
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Prefill {
    pub labels: Option<Vec<String>>,
    pub assignee: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubTasks {
    pub template_tasks: String,
    pub new_tasks: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraConfig {
    pub server: Server,
    pub prefill: Prefill,
    pub sub_tasks: SubTasks,
}

impl JiraConfig {
    pub fn template_task_list(&self) -> Vec<String> {
        self.sub_tasks
            .template_tasks
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect()
    }

    pub fn new_task_list(&self) -> Vec<String> {
        self.sub_tasks
            .new_tasks
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect()
    }
}

pub fn load_config() -> Result<JiraConfig, Box<dyn std::error::Error>> {
    let config_path = std::env::current_dir()?.join(".jirun.toml");
    if !config_path.exists() {
        return Err(format!(
            "❌ Config file not found at {:?}. Run `jirun init` to create one.",
            config_path
        )
        .into());
    }
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("❌ Failed to read config file at {:?}: {}", config_path, e))?;
    let config: JiraConfig = toml::from_str(&content).map_err(|e| {
        format!(
            "❌ Failed to parse TOML config: {}\nContent:\n{}",
            e, content
        )
    })?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_config() {
        let toml_str = r#"
            [server]
            url = "https://yourcompany.atlassian.net"

            [prefill]
            labels = ["cli"]
            assignee = "john.doe"

            [sub_tasks]
            template_tasks = """
            Set up database schema
            Implement service logic
            Write integration tests
            """
            new_tasks = """
            Fix login bug
            Refactor error handling
            Document API usage
            """
        "#;

        let config: JiraConfig = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.server.url, "https://yourcompany.atlassian.net");
        assert_eq!(config.prefill.labels, Some(vec!["cli".to_string()]));
        assert_eq!(config.prefill.assignee, Some("john.doe".to_string()));
        assert_eq!(
            config.template_task_list(),
            vec![
                "Set up database schema".to_string(),
                "Implement service logic".to_string(),
                "Write integration tests".to_string()
            ]
        );
        assert_eq!(
            config.new_task_list(),
            vec![
                "Fix login bug".to_string(),
                "Refactor error handling".to_string(),
                "Document API usage".to_string()
            ]
        );
    }
}

