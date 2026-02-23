use crate::command::CreateArgs;
use std::path::PathBuf;
pub fn logic_create(args: CreateArgs) -> Result<(), Box<dyn std::error::Error>> {
    // equate base path with
    // match in args.at
    // if found some value then
    // return the value
    // else convert args.keyword from Option<String>
    // to Option<&str> and equate with Some("at")
    let base_path = if let Some(flag_path) = args.at {
        flag_path
    } else if args.keyword.as_deref() == Some("at") {
        // contains an inline non argument taking clouser
        // || means the inline clouser takes no arg.
        args.path.unwrap_or_else(|| {
            // clouser's body
            // print error message to stderr
            eprintln!("Expected path after 'at'");
            // terminate the program
            std::process::exit(1);
        })
    } else {
        // convert string to path for the buffer
        PathBuf::from("leaderly")
    };

    // base_path ia path string of fs
    // we joined the formatted Month-YY
    // it becomes base_path/Month-YY or base_path\Month-YY
    // as per the system.
    let final_path = base_path.join(format!(
        "{}-{}",
        chrono::Local::now().format("%B"),
        chrono::Local::now().format("%y")
    ));
    std::fs::create_dir_all(final_path)?;
    Ok(())
}

