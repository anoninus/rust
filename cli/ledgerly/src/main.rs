mod command;
mod create;

use std::{
    env::{self},
    io::Result,
};

// prototype code
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // if argument is less than 2 then print help
    if args.len() < 2 {
        crate::command::help();
        return Ok(());
    }

    // argument 2 is always sub command
    let sub_command = &args[1];

    // used as_string as we can't match directly on String/&String
    match sub_command.as_str() {
        "greet" => crate::command::greet(&args),
        "add" => crate::command::add(&args)?,
        "help" => crate::command::help(),
        _ => println!("Diffrent arg")
    }
    
    
    
    Ok(())
}
