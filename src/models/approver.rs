use serde::Deserialize;

// Structs
#[derive(Deserialize)]
pub struct ApproverCode {
    pub new_code: String,
    pub days: u32,
}