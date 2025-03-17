use gloo_net::http::Request;
use serde_json::json;
use crate::models::{admin::AdminToken, approver::ApproverCode, client::Client};


// Enums
#[derive(Debug)]
pub enum ErrorReq {
    Unauthorized,
    InternalError,
    BadRequest,
}

// Struct
pub struct UnifiConnect {}

// Impl
impl UnifiConnect {
    const URL: &str = "http://localhost:8000";

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
                200 => {
                    let json = r.text().await.unwrap_or("".to_string());
                    Ok( serde_json::from_str::<Vec<Client>>(json.as_str()).unwrap_or(Vec::new()) )
                },
                401 => Err( ErrorReq::Unauthorized ),
                _ => Err( ErrorReq::BadRequest )
            },

            _ => Err( ErrorReq::InternalError )

        }
    }
    
    pub async fn approving_client_by_id(token: AdminToken, id: String) -> Result<(), ErrorReq> {
        let data = json!({
            "id": id,
            "connect": true,
            
            "mac": "",
            "site": "",
            "minutes": 0,
        });

        let response = Request::post( format!("{}/api/client/connect", Self::URL).as_str() )
            .header( "Content-Type", "application/json" )
            .header( "Authorization", format!("Bearer {}", token.token).as_str() )
            .body(data.to_string())
            .unwrap()
            .send()
            .await;

        match response {
            Ok(r) => match r.status() {
                200 => Ok( () ),
                401 => Err( ErrorReq::Unauthorized ),
                _ => Err( ErrorReq::BadRequest )
            },

            _ => Err( ErrorReq::InternalError )

        }

    } 
    
     pub async fn rejecting_client_by_id(token: AdminToken, id: String) -> Result<(), ErrorReq> {
        let data = json!({
            "id": id,
            "connect": false,
            
            "mac": "",
            "site": "",
            "minutes": 0,
        });

        let response = Request::post( format!("{}/api/client/connect", Self::URL).as_str() )
            .header( "Content-Type", "application/json" )
            .header( "Authorization", format!("Bearer {}", token.token).as_str() )
            .body(data.to_string())
            .unwrap()
            .send()
            .await;

        match response {
            Ok(r) => match r.status() {
                200 => Ok( () ),
                401 => Err( ErrorReq::Unauthorized ),
                _ => Err( ErrorReq::BadRequest )
            },

            _ => Err( ErrorReq::InternalError )

        }

    }

}
