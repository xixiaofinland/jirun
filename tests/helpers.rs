use jirun::config::JiraConfig;

pub fn sample_config_toml() -> &'static str {
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

pub fn load_sample_config() -> JiraConfig {
    toml::from_str(sample_config_toml()).expect("Failed to parse sample config")
}
