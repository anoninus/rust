use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileData {
    pub timestamp: String,
    pub data: String,
}
