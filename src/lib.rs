pub mod commands;
pub mod config;
pub mod env;

pub use commands::{handle_init, handle_new_command, handle_template_command};

mod jira;
mod task_context;
mod utils;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    commands::run()
}
