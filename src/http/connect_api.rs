use gloo_net::http::Request;
use serde_json::json;
use crate::models::{admin::AdminToken, approver::ApproverCode, client::Client};


// Enums
pub enum ErrorReq {
    Unauthorized,
    InternalError,
}

// Struct
pub struct UnifiConnect {}

// Impl
impl UnifiConnect {
    const URL: &str = "http://10.10.2.79:8000";

    pub async fn generate_approver_code(username: &String, password: &String) -> Result<Option<ApproverCode>, ErrorReq>  {
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

            _ => Err(ErrorReq::InternalError),
        }

    }

    pub async fn get_admin_token(username: &String, password: &String) -> Result<Option<AdminToken>, ErrorReq> {
         let data = json!({
            "username": username,
            "password": password
        });

        let response = Request::post( format!("{}/api/admin/login", Self::URL).as_str() )
                .header("Content-Type", "application/json")
                .body(data.to_string())
                .unwrap()
                .send()
                .await;

        match response {
            Ok(r) => match r.status() {
                202 => Ok( r.json::<AdminToken>().await.ok() ),

                _ => Ok(None),
            },

            _ => Err(ErrorReq::InternalError),
        }

    }
    
    pub async fn get_clients(token: AdminToken) -> Result<Vec<Client>, ErrorReq> {

        let response = Request::get( format!("{}/api/client", Self::URL).as_str() )
            .header( "Content-Type", "application/json" )
            .header( "Authorization", format!("Bearer {}", token.token).as_str() )
            .build()
            .unwrap()
            .send()
            .await;
        
        match response {
            Ok(r) => match r.status() {
                200 => Ok( r.json::<Vec<Client>>().await.unwrap() ),
                _ => Err( ErrorReq::Unauthorized )
            },

            _ => Err( ErrorReq::InternalError )

        }
    }
    


}
