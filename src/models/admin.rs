use serde::Deserialize;

// Struct
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct AdminToken {
    pub token: String
}
