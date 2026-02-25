use std::{error::Error, io::Write, process};

fn main() -> Result<(), Box<dyn Error>> {
    print!("RGrep: ");
    std::io::stdout().flush()?;
    let mut value = String::new();
    std::io::stdin().read_line(&mut value)?;

    let value: String = value.chars().filter(|char| !char.is_control()).collect();

    if value.trim().is_empty() {
        eprintln!("Intentionally crashing to prevent unnecessary results");
        process::exit(1);
    }

    let path = "./some.txt";
    let data = std::fs::read_to_string(path)?;
    if data.contains(&value) {
        println!("Found: {value} at {path}",)
    } else {
        eprintln!("Can not find {value} at {path}");
    }
    Ok(())
}
