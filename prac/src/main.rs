use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Commit {
    id: u32,
    message: String,
    timestamp: String,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Reporistory {
    staged_files: Vec<String>,
    commit: Vec<Commit>,
}

#[derive(Parser)]
#[command(name = "mgit", version = "1.0", about = "rust implementation of git")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        files: Vec<String>,
    },
    Commit {
        #[arg(short, long)]
        msg: String,
    },
    Push,
}
const DB_PATH: &str = "remote_server.json";
const LOCAL_STATE: &str = "local_state.json";
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { files } => {
           println!("Working") 
        }
    }
}
