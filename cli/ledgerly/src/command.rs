use crate::create;

enum Command {
    Greet(String)
}

pub fn greet(arg_in: &[String]) {
    let arg_2 = arg_in.get(2);
    let tip = "\n   Usage:      \nledgerly help";
    if arg_2.is_none() {
        println!("Hello, from Ledgerly!{tip}");
    } else {
        println!("Ledgerly greets {}", arg_in[2]);
    }
}

pub fn add(arg_in: &[String]) -> std::io::Result<()> {
    let arg_2 = arg_in.get(2);
    if arg_2.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Hint : Input something after add argument.",
        ));
    } else {
        let to_add = arg_in[2..].join(" ");
        let prompt: &str = &to_add;
        create::create_dir_()?;
        let path = create::file_name_check()?;
        let path_str: &str = &path;

        if std::path::Path::new(&path).exists() {
            create::file_exists_in_path(path_str, prompt)?;
        } else {
            std::fs::write(&path, prompt)?;
        }
    }

    Ok(())
}

pub fn help() {
    println!(
        r#"
        Usage : ledgerly <cmd>
        Where cmd is :
        
        1. add               --> ro add a value
        2. sum               --> for summarize.
        3. help              --> for help.
        4. greet             --> for greeting.
        5. since DD-MM-YY    --> to see from a time.
        6. today             --> to see today's ledger.
        7. yesterday         --> to see yesterday ledger.
        8. week              --> to see past 7 day ledger.
    "#
    )
}
