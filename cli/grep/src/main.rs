use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let value = input_handling()?;
        file_search(&value)?;
        if value.trim() == "exit" {
            std::process::exit(1);
        }
    }
}
fn input_handling() -> Result<String, Box<dyn Error>> {
    print!("RGrep: ");
    std::io::stdout().flush()?;
    let mut value = String::new();
    std::io::stdin().read_line(&mut value)?;

    let value: String = value.chars().filter(|char| !char.is_control()).collect();

    if value.trim().is_empty() {
        eprintln!("Intentionally crashing to prevent unnecessary results");
        return Err("Empty input".into());
    }
    Ok(value)
}

fn file_search(value: &str) -> Result<(), Box<dyn Error>> {
    let cwd = "./";
    for entry in std::fs::read_dir(cwd)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let line_number = line_num + 1;
            if line.contains(value) {
                println!(">{path:?}, at Line {line_number}, Found ===> {line}");
            }
        }
    }
    Ok(())
}
