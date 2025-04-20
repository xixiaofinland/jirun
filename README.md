# 🛠️ Jirun

> Generate JIRA sub-tasks from pre-defined templates — fast, local, and customizable.

## ✨ Features

- 📁 Init config template files: `.jirun.toml` and `.env`
- 🔗 Create defined sub-tasks with prefilled field values
- 🧪 Run with dry-run mode
- 🌍 Support global or local config
- 🔐 Detect and exclude duplicate existing sub-task (simply by summary/title)

## 🚀 Usage

```sh
Usage: jirun <COMMAND>

Commands:
  init      Create .jirun.toml and .env (defaults to the local directory)
  new       Create sub-tasks from [sub_tasks.new_tasks] in .jirun.toml
  template  Create sub-tasks from [sub_tasks.template_tasks]
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

📘 Examples:
  1. jirun help init
     Help menu on initializing jirun's configuration files.

  2. jirun init --global
     Create config files in the global directory.

  3. jirun template --parent PROJ-123
     Use [sub_tasks.template_tasks] to create sub-tasks under PROJ-123

  4. jirun new --parent PROJ-123 --assignee alice
     Use [sub_tasks.new_tasks], overriding assignee with 'alice'

  5. jirun template -p PROJ-123 --dry-run
     Show request payloads without sending to JIRA
```

## 📦 Install

```sh
cargo install jirun
```

## 🧰 Configuration

Run `.jirun init --global` to generate the config template files.

`.jirun.toml`:

```toml
[server]
url = "https://yourcompany.atlassian.net"

[prefill]
labels = ["cli", "auto"]
assignee = "john.doe"

[sub_tasks]
template_tasks = """
Design API schema
Implement service logic
Write integration tests
"""

new_tasks = """
Fix login bug
Improve error messages
Document usage
"""
```

`.env`:

```env
JIRA_TOKEN=your-api-token-here
```

## 📄 License

MIT
