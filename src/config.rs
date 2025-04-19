use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"[server]
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
    pub fn init_local() {
        let config_path = Path::new(".jirun.toml").to_path_buf();
        let env_path = Path::new(".env").to_path_buf();
        Self::init_at(config_path, env_path, "./");
    }

    pub fn init_global() {
        let base = dirs::config_dir()
            .expect("❌ Could not determine config directory (XDG_CONFIG_HOME)")
            .join("jirun");
        fs::create_dir_all(&base).expect("❌ Failed to create config directory");

        println!("📁 Global config path: {}\n", base.display());

        let config_path = base.join(".jirun.toml");
        let env_path = base.join(".env");

        Self::init_at(config_path, env_path, "");
    }

    fn init_at(config_path: PathBuf, env_path: PathBuf, local_prefix: &str) {
        if config_path.exists() {
            println!(
                "⚠️  Config file already exists: {}{}",
                local_prefix,
                config_path.display()
            );
        } else {
            fs::write(&config_path, DEFAULT_CONFIG).expect("❌ Failed to write config file");
            println!(
                "✅ Created config file: {}{}",
                local_prefix,
                config_path.display()
            );
        }

        if env_path.exists() {
            println!(
                "⚠️  .env file already exists: {}{}",
                local_prefix,
                env_path.display()
            );
        } else {
            fs::write(
                &env_path,
                "# JIRA API token (used by jirun)\nJIRA_TOKEN=your-api-token-here\n",
            )
            .expect("❌ Failed to write .env file");
            println!("✅ Created .env: {}{}", local_prefix, env_path.display());
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::config_locations()
            .into_iter()
            .find(|p| p.exists())
            .ok_or("❌ No config file found. Run `jirun init (--global)`.")?;

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

    fn config_locations() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        if let Ok(cwd) = std::env::current_dir() {
            paths.push(cwd.join(".jirun.toml"));
        }
        if let Some(home_config) = dirs::config_dir() {
            paths.push(home_config.join("jirun").join("config.toml"));
        }
        paths
    }
}
