mod helpers;
use helpers::*;

#[test]
fn test_config_parses_correctly() {
    let config = load_sample_config();

    assert_eq!(config.server.url, "https://yourcompany.atlassian.net");
    assert_eq!(
        config.prefill.labels.as_deref(),
        Some(&["cli".to_string(), "auto".to_string()][..])
    );
    assert_eq!(config.prefill.assignee.as_deref(), Some("john.doe"));
    assert_eq!(config.template_task_list(), vec!["Task A", "Task B"]);
    assert_eq!(config.new_task_list(), vec!["Task C", "Task D"]);
}
