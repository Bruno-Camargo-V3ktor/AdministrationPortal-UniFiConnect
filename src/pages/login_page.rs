use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_state, Callback, Html, SubmitEvent};

use crate::{components::form_login::FormLogin, http::connect_api::UnifiConnect};


// Components
#[function_component(LoginPage)]
pub fn page_login() -> Html {
    // States
    let error_msg = use_state(|| None::<String>);

    // Callbacks
    let on_submit = {
        let error_msg = error_msg.clone();

        Callback::from(move |(username, password): (String, String)| 
        {
            let username = username.clone();
            let password = password.clone();
            let error_msg = error_msg.clone();

            Callback::from(move |e: SubmitEvent|
            {
                e.prevent_default();
                let username = username.clone();
                let password = password.clone();
                let error_msg = error_msg.clone();

                spawn_local( async move {
                    let res = UnifiConnect::get_admin_token(&username, &password).await;

                    match res {
                        Ok(op) => match op {
                            Some(token) => {
                                web_sys::console::log_1(&token.token.into());
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

    // View
    html! {
        <>
            <div class={classes!("login-page-container")}>
                <FormLogin error_menssage={(*error_msg).clone()} on_handle_click={on_submit}  />
            </div>
        </>
    }
}