use crate::config::JiraConfig;
use std::fs;

pub fn write_sample_config_and_env(dir: &std::path::Path) {
    fs::write(dir.join(".jirun.toml"), test_config_toml()).unwrap();
    fs::write(dir.join(".env"), "JIRA_TOKEN=dummy-token\n").unwrap();
}

pub fn load_test_config() -> JiraConfig {
    toml::from_str(test_config_toml()).expect("Failed to parse test config")
}

fn test_config_toml() -> &'static str {
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
