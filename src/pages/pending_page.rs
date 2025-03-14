use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_context, use_effect_with, use_state, Html};
use crate::{components::card_client_pending::CardClientPending, contexts::admin_authorization::AdminAuthorizationCtx, http::connect_api::{ErrorReq, UnifiConnect}, models::{admin::AdminToken, client::{Client, ClientStatus}}};



// Components
#[function_component(PendingPage)]
pub fn pending_page() -> Html {
    
    // Hooks
    let ctx_auth = use_context::<AdminAuthorizationCtx>().unwrap();
    let client_list = use_state(|| Vec::<Client>::new());
    
    
    // Effects
    {
        let client_list = client_list.clone();
        let ctx_auth = ctx_auth.clone();

        use_effect_with((), move |_| {
            let list = client_list.clone();
            let ctx = ctx_auth.clone();

            spawn_local(async move {
                let res = UnifiConnect::get_clients(
                    ctx.token_admin.clone().unwrap_or( AdminToken { token: "".to_string() } )
                ).await;
                
                match res {
                    Ok(l) => {                           
                        let pendings: Vec<Client> = l.into_iter().filter(|c| c.status == ClientStatus::Pending ).collect();
                        list.set(pendings);
                    },

                    Err(e) => match e {
                         ErrorReq::Unauthorized => {
                            ctx.set_token.emit(None);
                        },

                        _ => {}
                    }
                }
            });
            
        });
    }

    // View
    html! {
        <>
        <div class={classes!("flex", "h-screen", "min-w-full", "bg-slate-200")}>
           
            <div class={classes!("flex", "flex-wrap", "w-full", "m-4", "content-start")}>
                {
                    client_list.iter().map( |c| {
                        html! {
                            <CardClientPending client={c.clone()} list={client_list.clone()} />
                        }
                    } ).collect::<Vec<Html>>() 
                }
            </div>

        </div>
        </>
    }
}
