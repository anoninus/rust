use std::io::Write;

use ::serde::{Deserialize, Serialize};
use chrono::Datelike;

#[derive(Serialize, Deserialize)]
struct FileData {
    timestamp: String,
    data: String,
}

pub fn initialization(data: String) -> Result<(), Box<dyn std::error::Error>> {
    let json_struct = FileData {
        timestamp: chrono::Local::now().to_rfc3339(),
        data,
    };

    let path = format!(
        "./ledgerly/{}-{}-{}.json",
        chrono::Local::now().day(),
        chrono::Local::now().format("%b"),
        chrono::Local::now().format("%y"),
    );

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)?;
    serde_json::to_writer_pretty(&mut file, &json_struct)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
