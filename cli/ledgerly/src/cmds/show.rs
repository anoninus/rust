use crate::command::{ShowCommand};
use crate::models::FileData;
use chrono::{Datelike};
use std::io::BufReader;
use chrono::{Duration, Local};

pub fn logic_show(mode: ShowCommand) -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::path::Path::new("./ledgerly");

    match mode {
        ShowCommand::Today => {
            let today = Local::now().date_naive();

            let path = dir.join(format!(
                "{}-{}-{}.json",
                today.day(),
                today.format("%b"),
                today.format("%y"),
            ));

            let file = std::fs::File::open(path)?;
            let reader = BufReader::new(file);
            let parsed: FileData = serde_json::from_reader(reader)?;

            println!("\nEntry For Today\n");
            println!("{}", parsed.data);
        }

        ShowCommand::Yesterday => {
            let yesterday = (Local::now() - Duration::days(1)).date_naive();

            let path = dir.join(format!(
                "{}-{}-{}.json",
                yesterday.day(),
                yesterday.format("%b"),
                yesterday.format("%y"),
            ));

            let file = std::fs::File::open(path)?;
            let reader = BufReader::new(file);
            let parsed: FileData = serde_json::from_reader(reader)?;

            println!("\nEntry For Yesterday\n");
            println!("{}", parsed.data);
        }
    }

    Ok(())
}
