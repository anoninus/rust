mod command;
mod create;
mod serialize_json;

use std::{
    env::{self}, error::Error
};

// prototype code
fn main() -> std::result::Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    // if argument is less than 2 then print help
    if args.len() < 2 {
        crate::command::parse_command()?;
        return Ok(());
    }

    command::parse_command()?;
    
    Ok(())
}
