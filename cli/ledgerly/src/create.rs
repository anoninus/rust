use std::io::Write;

pub fn create_dir_() -> std::io::Result<()> {
    println!("creating dir...");
    std::fs::create_dir_all("./ledgerly/")?;
    Ok(())
}

fn get_input(prompt: &str) -> std::io::Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn file_name_check() -> std::io::Result<String> {
    let input_name = get_input("Enter file's name: ")?;
    let path = format!("./ledgerly/{}.txt", input_name);

    if input_name.is_empty() {
        println!("Input name can not be empty! Retry...");
    }

    if input_name.contains(".") {
        println!("Not allowed to add extension or dots in file name")
    }

    if std::path::Path::new(&path).exists() {
        println!("Path already exists!");
    }
    Ok(path)
}

pub fn file_exists_in_path(path: &str, prompt: &str) -> std::io::Result<()> {
    loop {
        let ans = get_input("File exists! Overwrite? (y/n): ")?;
        match ans.as_str() {
            "y" => {
                std::fs::write(path, prompt)?;
                break;
            }

            "n" => {
                break;
            }

            _ => {
                println!("Please enter `y` or `n`");
                continue;
            }
        }
    }
    Ok(())
}
