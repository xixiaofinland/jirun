use crate::config::JiraConfig;
use std::fs;

#[allow(dead_code)]
pub fn write_sample_config_and_env(dir: &std::path::Path) {
    fs::write(dir.join(".jirun.toml"), sample_config_toml()).unwrap();
    fs::write(dir.join(".env"), "JIRA_TOKEN=dummy-token\n").unwrap();
}

#[allow(dead_code)]
pub fn load_sample_config() -> JiraConfig {
    toml::from_str(sample_config_toml()).expect("Failed to parse sample config")
}

fn sample_config_toml() -> &'static str {
    r#"[server]
        url = "https://yourcompany.atlassian.net"

        [prefill]
        labels = ["cli", "auto"]
        assignee = "john.doe"

        [sub_tasks]
        template_tasks = """
        Task A
        Task B
        """

        new_tasks = """
        Task C
        Task D
        """
    "#
}
