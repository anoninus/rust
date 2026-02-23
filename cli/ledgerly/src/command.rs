use crate::cmds::{add, greet};
use crate::cmds::{show, createdir};
use std::{error::Error, path::PathBuf};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ledgerly",
    about = "the ledger taking app",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Used for greeting someone.
    Greet { name: Option<String> },
    /// A add command
    Add { text: Vec<String> },
    /// Create dir
    #[command(alias = "c")]
    Create(CreateArgs),

    #[command(alias = "sh")]
    Show(ShowArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[arg(short, long)]
    pub at: Option<PathBuf>,

    pub keyword: Option<String>,
    pub path: Option<PathBuf>,
}

#[derive(Args)]
pub struct ShowArgs {
    #[arg(short, long)]
    pub today: bool,
}

pub fn parse_command() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Greet { name } => {
            greet::logic_greet(name);
        }

        Commands::Add { text } => {
            add::logic_add(text)?;
        }
        Commands::Create(args) => {
            createdir::logic_create(args)?;
        }
        Commands::Show(args) => {
            show::logic_show(args)?;
        }
    }
    Ok(())
}

