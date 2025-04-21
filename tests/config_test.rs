use jirun::common::test_helper::load_test_config;

#[test]
fn test_config_parses_correctly() {
    let config = load_test_config();

    assert_eq!(config.server.url, "https://yourcompany.atlassian.net");
    assert_eq!(
        config.prefill.labels.as_deref(),
        Some(&["cli".to_string(), "auto".to_string()][..])
    );
    assert_eq!(config.prefill.assignee.as_deref(), Some("john.doe"));
    assert_eq!(config.template_task_list(), vec!["Task A", "Task B"]);
    assert_eq!(config.new_task_list(), vec!["Task C", "Task D"]);
}

#[test]
fn test_missing_server_section_fails() {
    let toml = r#"
        [prefill]
        labels = ["cli"]
        assignee = "john.doe"

        [sub_tasks]
        template_tasks = """task 1"""
        new_tasks = """task 2"""
    "#;

    let result = toml::from_str::<jirun::config::JiraConfig>(toml);
    assert!(result.is_err(), "Expected error due to missing [server]");
}

#[test]
fn test_missing_sub_tasks_section_fails() {
    let toml = r#"
        [server]
        url = "https://yourcompany.atlassian.net"

        [prefill]
        labels = ["cli"]
        assignee = "john.doe"
    "#;

    let result = toml::from_str::<jirun::config::JiraConfig>(toml);
    assert!(result.is_err(), "Expected error due to missing [sub_tasks]");
}

#[test]
fn test_invalid_labels_type_fails() {
    let toml = r#"
        [server]
        url = "https://yourcompany.atlassian.net"

        [prefill]
        labels = "cli"  # should be a list!
        assignee = "john.doe"

        [sub_tasks]
        template_tasks = """task 1"""
        new_tasks = """task 2"""
    "#;

    let result = toml::from_str::<jirun::config::JiraConfig>(toml);
    assert!(result.is_err(), "Expected error due to invalid labels type");
}
