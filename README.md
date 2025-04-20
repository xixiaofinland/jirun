# 🛠️ Jirun

>A CLI tool that generates JIRA sub-task(s) with pre-populated field values.
<br>

## ❓ What Problem Does It Solve?

Manually creating repetitive JIRA sub-tasks can be time-consuming and
error-prone. Jirun streamlines this by allowing you to generate sub-tasks with
template, saving you time and ensuring consistency.

## 🚀 Quick Start
1. **Initialize Configuration:**

```bash
jirun init --global
```

This creates `.jirun.toml` and `.env` files in OS preferred directory.

2. **Edit Configuration:**

- In `.env` set your JIRA user bearer token: the authentication to create sub-tasks.
- In `.jirun.toml` define server url, templated subtask field and value(s), and
  subtasks to be created when running `jirun template` and `jirn new` commands.

3. **Dry Run**

```bash
jirun template --parent PROJ-123 --dry-run # then without `--dry-run` for the real action.
```

When you are not sure, this command prints out the raw payloads without sending
the real request.

4. **Create new Sub-tasks**

```bash
jirun new --parent PROJ-123

```

This command creates JIRA sub-tasks defined in `[sub_tasks.new_tasks]`. Each
subtask per line.

## ✨ Other Features

- 🌍 Support generating global (`jirun init --global`) or local (`jirun init`) config files.
- 🔐 Detect and skip subtasks that are already existing.

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

## ❓ FAQ

- "difference between template_tasks for `jirun template` and new_tasks for `jirun new`"?

template_tasks is for storing reusable tasks you create repeatedly. new_tasks is
for ad-hoc subtasks you update frequently. Functionally, `jirun template` and
`jirun new` have the same underlying logic.

## 📄 License

MIT
