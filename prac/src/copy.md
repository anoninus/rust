``` rust
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::Local;

// --- DATA MODELS ---

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Commit {
    id: usize,
    message: String,
    timestamp: String,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Repository {
    staged_files: Vec<String>,
    commits: Vec<Commit>,
}

// --- CLI STRUCTURE ---

#[derive(Parser)]
#[command(name = "mgit", about = "Mini-Git Prototype", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add files to the staging area
    Add { files: Vec<String> },
    /// Create a new commit from staged files
    Commit { 
        #[arg(short, long)]
        message: String 
    },
    /// Push commits to the JSON "database"
    Push,
}

// --- LOGIC ---

const DB_PATH: &str = "remote_server.json";
const LOCAL_STATE: &str = ".mgit_state.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load current local state or create new if it doesn't exist
    let mut repo: Repository = if Path::new(LOCAL_STATE).exists() {
        let data = fs::read_to_string(LOCAL_STATE)?;
        serde_json::from_str(&data)?
    } else {
        Repository::default()
    };

    match cli.command {
        Commands::Add { files } => {
            repo.staged_files.extend(files);
            println!("Staged {} files.", repo.staged_files.len());
        }
        Commands::Commit { message } => {
            if repo.staged_files.is_empty() {
                println!("Nothing to commit (staging area empty).");
            } else {
                let new_commit = Commit {
                    id: repo.commits.len() + 1,
                    message,
                    timestamp: Local::now().to_rfc3339(),
                    files: repo.staged_files.drain(..).collect(),
                };
                println!("Created commit: {}", new_commit.id);
                repo.commits.push(new_commit);
            }
        }
        Commands::Push => {
            let json = serde_json::to_string_pretty(&repo.commits)?;
            fs::write(DB_PATH, json)?;
            println!("Successfully pushed {} commits to {}", repo.commits.len(), DB_PATH);
        }
    }

    // Save state back to local file
    let state_json = serde_json::to_string(&repo)?;
    fs::write(LOCAL_STATE, state_json)?;

    Ok(())
}
``` 
