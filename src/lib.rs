pub mod commands;
pub mod config;
pub mod env;
pub mod jira;
pub mod task_context;
pub mod utils;

#[cfg(test)]
pub mod common;

/// Result type used throughout the application
pub type JirunResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Main entry point for the CLI application
pub fn run() -> JirunResult<()> {
    commands::run()
}
