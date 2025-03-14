use yew::{classes, function_component, html, Html};

use crate::{components::card_client_pending::CardClientPending, models::client::Client};



// Components
#[function_component(PendingPage)]
pub fn pending_page() -> Html {
    
    let client = Client {
        id: String::from("23123123"),
        cpf: None,
        mac: "12:12:12:12".to_string(),
        site: String::from("Default"),
        email: String::from("email@example.com"),
        phone: String::from("00123456789"),
        status: crate::models::client::ClientStatus::Expired,
        hostname: None,
        tx_bytes: None,
        rx_bytes: None,
        approver: "Tia".to_string(),
        full_name: "Bruno Camargo Ferreira".to_string(),
        companion: "Leo".to_string(),
        start_time: "12/12/12".to_string(),
        client_type: "Visitante".to_string(),
        time_connection: "".to_string()
    };

    // View
    html! {
        <>
        <div class={classes!("flex", "h-screen", "min-w-full", "bg-slate-200")}>
        <CardClientPending 
            client={ client.clone() }
        />        
        </div>
        </>
    }
}
