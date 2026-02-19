use crate::serialize_json;
use clap::{Parser, Subcommand};
use std::{error::Error};

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
    Greet {
        name: Option<String>,
    },
    /// A add command
    Add {
        text: Vec<String>,
    },
    Create,
}

pub fn parse_command() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Greet { name } => match name {
            Some(n) => println!("Ledgerly greets {n}"),
            None => println!("Hello, from Ledgerly!\n\nUsage:\nledgerly help"),
        },

        Commands::Add { text } => {
            if text.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Hint: Input something after add argument.",
                )));
            }

            let log = text.join(" ");
            serialize_json::initialization(log)?;
        }
        Commands::Create => {
            let path = format!("leaderly/{}-{}",chrono::Local::now().format("%B"), chrono::Local::now().format("%y"));
            std::fs::create_dir_all(path)?;
        }
    }
    Ok(())
}
