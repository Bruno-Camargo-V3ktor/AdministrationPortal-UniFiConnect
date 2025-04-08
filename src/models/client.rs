use std::collections::HashMap;

use serde::{Deserialize, Deserializer};


// Enums
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ClientStatus {
    Approved,
    Pending,
    Reject,
    Expired,
}


// Structs
#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Client {
    #[serde(rename = "_id", deserialize_with = "mongodb_id")]
    pub id: String,

    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub fields: HashMap<String, String>,

    pub mac: String,
    pub site: String,
    pub status: ClientStatus,

    pub hostname: Option<String>,
    pub tx_bytes: Option<usize>,
    pub rx_bytes: Option<usize>,

    pub time_connection: String,
    pub start_time: String,
    pub approver: String,
}

// Functions
fn mongodb_id<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    
    Ok (
        value.get("$oid")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned())
        .unwrap_or("".to_string())
    )
}
