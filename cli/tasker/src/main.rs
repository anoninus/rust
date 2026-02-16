mod tmp;

use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

type AppResult<T> = Result<T, io::Error>;
fn main() -> AppResult<()> {
    let mut file = file_creation()?;
    editor(&mut file)?;
    Ok(())
}

fn validiate_name(f_name: &str) -> AppResult<()> {
    if f_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name is empty!",
        ));
    }
    if f_name.contains(".") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name either contains an extension or dot!",
        ));
    }
    Ok(())
}

fn open_or_create(filename: &str) -> AppResult<File> {
    let path = Path::new(&filename);
    OpenOptions::new().append(true).create(true).open(&path)
}

fn file_creation() -> AppResult<File> {
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter file name: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if let Err(e) = validiate_name(trimmed) {
            println!("Error : {e}");
            continue;
        }

        let file = open_or_create(trimmed)?;
        println!("Editing {}.txt", trimmed);
        return Ok(file);
    }
}

fn editor(file: &mut File) -> AppResult<()> {
    loop {
        let mut lines = String::new();
        let bytes = io::stdin().read_line(&mut lines)?;
        if bytes == 0 {
            break;
        }

        file.write_all(lines.as_bytes())?;
    }
    Ok(())
}
