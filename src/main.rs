use std::error::Error;

use clap::{Parser, Subcommand};
use jirun::{env, handle_init, handle_new_command, handle_template_command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    jirun::run()
}

// fn main() -> Result<(), Box<dyn Error>> {
//     env::try_load_dotenv();
//     let cli = Cli::parse();
//
//     match cli.command {
//         Commands::Init { global } => handle_init(global),
//         Commands::Template {
//             parent,
//             assignee,
//             dry_run,
//         } => handle_template_command(parent, assignee.as_deref(), dry_run)?,
//         Commands::New {
//             parent,
//             assignee,
//             dry_run,
//         } => handle_new_command(parent, assignee.as_deref(), dry_run)?,
//     }
//
//     Ok(())
// }
