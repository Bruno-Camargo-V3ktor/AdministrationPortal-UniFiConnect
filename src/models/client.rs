use serde::Deserialize;


// Enums
#[derive(Deserialize, Debug)]
pub enum ClientStatus {
    Approved,
    Pending,
    Reject,
    Expired,
}


// Structs
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Client {
    pub id: String,
    pub client_type: String,

    pub full_name: String,
    pub companion: String,
    pub email: String,
    pub phone: String,
    pub cpf: Option<String>,

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
