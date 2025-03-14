use yew::{classes, function_component, html, Html, Properties, UseStateHandle};

use crate::models::client::Client;


// Struct
#[derive(Properties, PartialEq)]
pub struct CardClientPendingProps {
    pub client: Client,
    //pub list: UseStateHandle<Vec<Client>>
}

// Component
#[function_component(CardClientPending)]
pub fn card_client_pending(props: &CardClientPendingProps) -> Html {

    
    // View
    html! {
        <>
        <div class={classes!("flex", "flex-row", "items-center", "justify-center", "bg-slate-100", "p-4", "m-4", "rounded-lg", "h-48", "max-h-48", "w-full", "max-w-full", "font-mono", "shadow-lg", "shadow-orange-300/30")}>
        
           <div class={classes!("flex", "flex-col", "flex-1", "h-full", "p-4")}>
                <h3>
                   { props.client.full_name.clone() } 
                </h3>

                <p>
                    { props.client.companion.clone() }
                </p>

                <p>
                    { props.client.email.clone() }
                </p> 
                
                <p>
                    { props.client.phone.clone() }
                </p>
           </div>

           <div class={classes!("flex", "flex-col", "gap-2", "h-full", "justify-around", "m-2")}>
                <button class={classes!()} >
                    { "Sim" }
                </button>

                <button class={classes!()}>
                    { "Nao" }
                </button>
           </div> 

        </div>
        </>
    }

}
