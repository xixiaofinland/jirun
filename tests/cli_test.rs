use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

fn write_sample_config_and_env(dir: &std::path::Path) {
    use std::fs;

    let config_content = r#"[server]
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
"#;

    fs::write(dir.join(".jirun.toml"), config_content).unwrap();
    fs::write(dir.join(".env"), "JIRA_TOKEN=dummy-token\n").unwrap();
}

#[test]
#[ignore]
fn template_command_prints_tasks_and_aborts_on_no() {
    let dir = tempdir().unwrap();
    write_sample_config_and_env(dir.path());

    Command::cargo_bin("jirun")
        .unwrap()
        .current_dir(&dir)
        .args(["template", "--parent", "PROJ-123"])
        .write_stdin("n\n")
        .assert()
        .success()
        .stdout(contains("PROJ-123"))
        .stdout(contains("Aborted"));
}

// TODO: how to CLI test for mocking api?
#[test]
#[ignore]
fn template_command_accepts_confirmation_and_proceeds() {
    let dir = tempfile::tempdir().unwrap();
    write_sample_config_and_env(dir.path());

    Command::cargo_bin("jirun")
        .unwrap()
        .current_dir(&dir)
        .args(["template", "--parent", "PROJ-123"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(contains("PROJ-123"))
        .stdout(contains("Task A"))
        .stdout(contains("Task B"))
        .stdout(contains("john.doe"));
}

#[test]
#[ignore]
fn template_command_dry_run_skips_confirmation_and_prints_payload() {
    let dir = tempdir().unwrap();
    write_sample_config_and_env(dir.path());

    Command::cargo_bin("jirun")
        .unwrap()
        .current_dir(&dir)
        .args(["template", "--parent", "PROJ-123", "--dry-run"])
        .assert()
        .success()
        .stdout(contains(
            "Dry-run: would send this payload for sub-task #1: 'Task A'",
        ))
        .stdout(contains("https://yourcompany.atlassian.net"))
        .stdout(contains("PROJ-123"))
        .stdout(contains("john.doe"))
        .stdout(contains("Task A"))
        .stdout(contains("Task B"))
        .stdout(contains("Dry-run: no requests were sent."));
}

#[test]
#[ignore]
fn new_command_simple_flag_creates_single_task() {
    let dir = tempdir().unwrap();
    write_sample_config_and_env(dir.path());

    Command::cargo_bin("jirun")
        .unwrap()
        .current_dir(&dir)
        .args(["new", "--parent", "PROJ-123", "--simple", "Fix login bug"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(contains("PROJ-123"))
        .stdout(contains("Fix login bug"));
}

