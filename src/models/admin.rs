use serde::Deserialize;

// Struct
#[derive(Deserialize)]
pub struct AdminToken {
    pub token: String
}