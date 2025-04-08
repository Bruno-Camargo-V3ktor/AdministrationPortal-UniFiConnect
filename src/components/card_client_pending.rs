use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_context, Callback, Html, Properties, UseStateHandle};
use yew_router::hooks::use_navigator;

use crate::{contexts::admin_authorization::AdminAuthorizationCtx, http::connect_api::{ErrorReq, UnifiConnect}, models::client::Client, routes::Route};


// Struct
#[derive(Properties, PartialEq)]
pub struct CardClientPendingProps {
    pub client: Client,
    pub list: UseStateHandle<Vec<Client>>
}

// Component
#[function_component(CardClientPending)]
pub fn card_client_pending(props: &CardClientPendingProps) -> Html {
    
    // Hooks
    let ctx_auth = use_context::<AdminAuthorizationCtx>().unwrap();
    let nav = use_navigator().unwrap();

    // Callbacks
    let on_approval_click = {
        let client = props.client.clone();
        let list = props.list.clone();
        let ctx = ctx_auth.clone();
        let nav = nav.clone();

        Callback::from(move |_| {
            let client = client.clone();
            let list = list.clone();
            let ctx = ctx.clone();
            let nav = nav.clone();

            spawn_local(async move {
                let client = client.clone();
                let list = list.clone();
                let ctx = ctx.clone();
                let navigator = nav.clone();

                let res = UnifiConnect::approving_client_by_id(ctx.token_admin.clone().unwrap(), client.id.clone()).await;
                match res {
                    Ok(_) => {
                        let new_list = list.iter().filter( |c| c.id.clone() != client.id.clone()  ).map( |c| c.clone() ).collect::<Vec<Client>>();
                        list.set(new_list.clone());
                    },

                    Err(e) => match e {
                        ErrorReq::Unauthorized => {
                            ctx.set_token.emit(None);
                            navigator.push(&Route::Login);
                        },

                        _ => {}
                    }
                }

            });
        })
    };
    
    let on_reject_click = {
        let client = props.client.clone();
        let list = props.list.clone();
        let ctx = ctx_auth.clone();
        let nav = nav.clone();

        Callback::from(move |_| {
            let client = client.clone();
            let list = list.clone();
            let ctx = ctx.clone();
            let nav = nav.clone();

            spawn_local(async move {
                let client = client.clone();
                let list = list.clone();
                let ctx = ctx.clone();
                let navigator = nav.clone();

                let res = UnifiConnect::rejecting_client_by_id(ctx.token_admin.clone().unwrap(), client.id.clone()).await;
                match res {
                    Ok(_) => {
                        let new_list = list.iter().filter( |c| c.id.clone() != client.id.clone()  ).map( |c| c.clone() ).collect::<Vec<Client>>();
                        list.set(new_list.clone());
                    },

                    Err(e) => match e {
                        ErrorReq::Unauthorized => {
                            ctx.set_token.emit(None);
                            navigator.push(&Route::Login);
                        },

                        _ => {}
                    }
                }

            });
        })
    };

    // View
    html! {
        <>
        <div class={classes!("flex", "flex-row", "items-center", "justify-center", "bg-slate-100", "p-4", "m-4", "rounded-lg", "h-48", "w-1/4", "font-mono", "shadow-lg", "shadow-orange-300/30")}>
        
           <div class={classes!("flex", "flex-col", "flex-1", "h-full", "p-4")}>
                <h3 class={classes!("text-xl", "mb-4", "font-bold")}>
                   { props.client.full_name.clone() } 
                </h3>

                //<p>
                //    <span class={classes!("italic", "text-gray-500")}>
                //        { "Acompanhado: " }
                //    </span>

                    //<span>
                    //    { props.client.companion.clone() }
                    //</span>
                //</p>

                <p>
                    <span class={classes!("italic", "text-gray-500")}>
                        { "E-mail: " }
                    </span>

                    <span>
                        { props.client.email.clone() }
                    </span>
                </p> 
                
                <p>
                    <span class={classes!("italic", "text-gray-500")}>
                        { "Telefone: " }
                    </span>

                    <span>
                        { props.client.phone.clone() }
                    </span>
                </p>
           </div>

           <div class={classes!("flex", "flex-col", "gap-2", "h-full", "justify-around", "m-2")}>
                <button 
                    onclick={ on_approval_click.clone() }
                    class={classes!("w-10", "h-10", "bg-green-600", "text-white", "rounded", "font-bold", "hover:bg-green-700", "hover:size-[120%]")} 
                >
                    { "✓" }
                </button>

                <button
                    onclick={ on_reject_click.clone() }
                    class={classes!("w-10", "h-10", "bg-red-600", "text-white", "rounded", "font-bold", "hover:bg-red-700", "hover:size-[120%]")}
                >
                    { "X" }
                </button>
           </div> 

        </div>
        </>
    }

}
