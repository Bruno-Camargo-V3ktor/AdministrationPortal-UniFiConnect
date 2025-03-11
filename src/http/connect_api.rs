use gloo_net::http::Request;
use serde_json::json;
use crate::models::approver::ApproverCode;



// Struct
pub struct UnifiConnect {}

// Impl
impl UnifiConnect {
    const URL: &str = "http://10.10.2.79:8000";

    pub async fn generate_approver_code(username: &String, password: &String) -> Result<Option<ApproverCode>, ()>  {
        let data = json!({
            "username": username,
            "password": password
        });

        let response = Request::put( format!("{}/api/approver/code", Self::URL).as_str() )
                .header("Content-Type", "application/json")
                .body(data.to_string())
                .unwrap()
                .send()
                .await;

        match response {
            Ok(r) => match r.status() {
                200 => Ok( r.json::<ApproverCode>().await.ok() ),

                _ => Ok(None),
            },

            _ => Err(()),
        }

    }

}
