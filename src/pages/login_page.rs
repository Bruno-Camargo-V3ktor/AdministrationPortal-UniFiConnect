use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_context, use_effect_with, use_state, Callback, Html, SubmitEvent};
use yew_router::hooks::use_navigator;
use crate::{components::form_login::FormLogin, contexts::admin_authorization::AdminAuthorizationCtx, http::connect_api::UnifiConnect, routes::Route};


// Components
#[function_component(LoginPage)]
pub fn page_login() -> Html {
    
    // States & Props
    let error_msg = use_state(|| None::<String>);
    let admin_authorization_ctx = use_context::<AdminAuthorizationCtx>().unwrap();
    let navigator = use_navigator().unwrap();

    // Callbacks
    let on_submit = {
        let error_msg = error_msg.clone();
        let ctx = admin_authorization_ctx.clone();
        let nav = navigator.clone();

        Callback::from(move |(username, password): (String, String)| 
        {
            let username = username.clone();
            let password = password.clone();
            let error_msg = error_msg.clone();
            let ctx = ctx.clone();
            let nav = nav.clone();

            Callback::from(move |e: SubmitEvent|
            {
                e.prevent_default();
                let username = username.clone();
                let password = password.clone();
                let error_msg = error_msg.clone();
                let ctx = ctx.clone();
                let nav = nav.clone();
                
                error_msg.set(None);

                spawn_local( async move {
                    let res = UnifiConnect::get_admin_token(&username, &password).await;
                    
                    match res {
                        Ok(op) => match op {
                            Some(token) => {
                                ctx.set_token.emit( Some(token.clone()) );
                                nav.push(&Route::Pending);
                                error_msg.set(None);
                            }
    
                            _ => {
                                error_msg.set(Some(
                                    "Falha no login. Verifique suas credencias.".to_string(),
                                ));
                            }
                        },
    
                        Err(_) => {
                            error_msg
                                .set(Some("Error ao se comunicar com o servidor.".to_string()));
                        }
                    };

                } );

            })
        })
    };
    
    // Effects
    {
        let ctx = admin_authorization_ctx.clone();
        let nav = navigator.clone();
        use_effect_with((),move |_| {
            if ctx.token_admin.is_some() {
                nav.push(&Route::Pending)
            }
        });
    }

    // View
    html! {
        <>
            <div class={classes!("bg-gradient-to-r", "from-orange-500", "to-orange-400", "h-screen", "w-screen", "flex", "justify-center", "items-center")}>
                <FormLogin error_menssage={(*error_msg).clone()} on_handle_click={on_submit} btn_text={"Login".to_string()}  />
            </div>
        </>
    }
}
