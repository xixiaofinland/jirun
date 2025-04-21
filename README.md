# 🛠️ Jirun

>A CLI tool that generates JIRA sub-task(s) with pre-populated field values.

## Table of Contents
- [💡 What Problem Does It Solve?](#-what-problem-does-it-solve)
- [🚀 Quick Start](#-quick-start)
- [✨ Other Features](#-other-features)
- [🚀 Usage](#-usage)
- [📥 Installation](#-installation)
  - [1. Script Install](#1-script-install)
    - [For Linux/MacOS](#for-linuxmacos)
    - [For Windows (PowerShell)](#for-windows-powershell)
  - [2. Cargo Install](#2-cargo-install)
  - [3. Manual Download](#3-manual-download)
- [🧰 Configuration](#-configuration)
- [❓ FAQ](#-faq)
- [📄 License](#-license)

<br>

## 💡 What Problem Does It Solve?

Tired of the repetitive JIRA sub-task creating dance? You know — clicking "new",
selecting "Sub-task", give a descriptive name, assigning yourself, adding the
"teamX" label, clicking "create"... and 10+ sub tasks in the queue...

`jirun` eliminates this tedium by generating consistent sub-tasks from templates
with pre-filled fields—saving you time and keeping your workflow smooth.

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

## 📥 Installation

### 1. Script Install

#### For Linux/MacOS

```bash
curl -sL https://raw.githubusercontent.com/xixiaofinland/jirun/main/scripts/install-jirun.sh | bash
```

#### For Windows (PowerShell)

```ps1
iwr -useb https://raw.githubusercontent.com/xixiaofinland/jirun/main/scripts/install-jirun.ps1 | iex
```

> [!NOTE]
> If you see an error like "This script contains malicious content and has been
> blocked by your antivirus software", it means Microsoft Defender flagged it
> for downloading and executing content from the internet. To proceed, either
> lower Defender’s protection or break the script into smaller steps:

```ps1
# Step 1: Review the script manually
Invoke-WebRequest -Uri https://raw.githubusercontent.com/xixiaofinland/jirun/main/scripts/install-jirun.ps1 -OutFile install-jirun.ps1
notepad install-jirun.ps1  # Inspect the content

# Step 2: Run after trust
powershell -ExecutionPolicy Bypass -File install-jirun.ps1
```

<br>

### 2. Cargo Install

`jirun` is published in creates.io [here](https://crates.io/crates/jirun).
Run cmd below if you have the `Cargo` tool.

```bash
cargo install jirun
```

<br>

### 3. Manual Download

Visit the [release page](https://github.com/xixiaofinland/jirun/releases/latest) and download the appropriate binary for your operating system (Linux, macOS, or Windows).

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

> What's the difference between putting sub-tasks under template_tasks or
> new_tasks section in .jirun.toml?

**template_tasks:**

- Executed by `jirun template` command
- Store sub-tasks you repeat frequently across different tickets
- Reusable workflow patterns

**new_tasks:**

- Executed by `jirun new` command
- For ad-hoc, ticket-specific sub-tasks
- One-off task sequences

Both commands share the same underlying logic - the separation simply helps
organize your brain between standard templates and custom task lists.

## 📄 License

MIT
