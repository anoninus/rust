use chrono::Datelike;
use std::io::BufReader;
use crate::models::FileData;
use crate::command::ShowArgs;
pub fn logic_show(args: ShowArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.today {
        let dir = std::path::Path::new("./ledgerly");

        let path = dir.join(format!(
            "{}-{}-{}.json",
            chrono::Local::now().day(),
            chrono::Local::now().format("%b"),
            chrono::Local::now().format("%y"),
        ));
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let parsed: FileData = serde_json::from_reader(reader)?;
        println!("Entry For Today\n");
        println!("{}", parsed.data);
    }
    Ok(())
}
