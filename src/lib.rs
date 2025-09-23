pub mod commands;
pub mod config;
pub mod env;
pub mod jira;
pub mod task_context;
pub mod utils;

#[cfg(test)]
pub mod common;

pub type JirunResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run() -> JirunResult<()> {
    commands::run()
}
