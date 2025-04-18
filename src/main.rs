mod config;
mod jira;
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use commands::{handle_init, handle_new, handle_template};

#[derive(Parser)]
#[command(name = "jirun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate JIRA sub-tasks from a template with a specified parent")]
struct Cli {
    #[arg(short = 'd', long = "diagnose", global = true)]
    diagnose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Template {
        #[arg(short = 'p', long = "parent")]
        parent: String,
    },
    New {
        #[arg(short = 'p', long = "parent")]
        parent: String,
    },
    Init,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => handle_init()?,
        Commands::Template { parent } => handle_template(parent, cli.diagnose)?,
        Commands::New { parent } => handle_new(parent, cli.diagnose)?,
    }

    Ok(())
}
