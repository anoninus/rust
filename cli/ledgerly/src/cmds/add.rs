use std::error::Error;

use crate::serialize_json;
pub fn logic_add(text: Vec<String>)-> Result<(), Box<dyn Error>>{
    if text.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Hint: Input something after add argument.",
        )));
    }

    let log = text.join(" ");
    serialize_json::initialization(log)?;

    Ok(())
}
