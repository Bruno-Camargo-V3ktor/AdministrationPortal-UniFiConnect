use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::form_login::FormLogin;

// Structs
#[derive(Deserialize)]
struct ApproverCode {
    pub new_code: String,
    pub days: u32,
}

// Components
#[function_component(ApproverCodePage)]
pub fn index() -> Html {
    // States
    let error_menssage = use_state(|| None::<String>);
    let show_modal = use_state(|| false);
    let approver_code = use_state(|| None::<ApproverCode>);

    // Callbacks
    let on_submit = {
        let error_menssage = error_menssage.clone();
        let show_modal = show_modal.clone();
        let approver_code = approver_code.clone();

        Callback::from(move |(username, password): (String, String)| {
            
            let username = username.clone();
            let password = password.clone();
            let approver_code = approver_code.clone();
            let error_menssage = error_menssage.clone();
            let show_modal = show_modal.clone();

            Callback::from(move |e: SubmitEvent| {
                e.prevent_default();
    
                let username = username.clone();
                let password = password.clone();
                let approver_code = approver_code.clone();
                let error_menssage = error_menssage.clone();
                let show_modal = show_modal.clone();
    
                spawn_local(async move {
                    let data = json!({
                        "username": username,
                        "password": password
                    });
    
                    let response = Request::put("-")
                        .header("Content-Type", "application/json")
                        .body(data.to_string())
                        .unwrap()
                        .send()
                        .await;
    
                    match response {
                        Ok(r) => match r.status() {
                            200 => {
                                approver_code.set(r.json().await.ok());
                                show_modal.set(true);
                                error_menssage.set(None);
                            }
    
                            _ => {
                                error_menssage.set(Some(
                                    "Falha no login. Verifique suas credencias.".to_string(),
                                ));
                            }
                        },
    
                        Err(_) => {
                            error_menssage
                                .set(Some("Error ao se comunicar com o servidor.".to_string()));
                        }
                    };
                });
            })
        }) 
    };

    // Render
    html! {
        <div class={classes!("approver-code-page-container")}>
            <FormLogin error_menssage={(*error_menssage).clone()} on_handle_click={on_submit.clone()} />

            {
                if *show_modal {
                    html! {
                        <div class="modal">
                            <div class="modal-content">
                                {
                                    if let Some(data) = &*approver_code {
                                        html! {
                                            <>
                                                <h2>{"Codigo Gerado"}</h2>
                                                <p>
                                                    {"O novo codigo e "}
                                                    <span class={classes!("special-text")}>{data.new_code.clone()}</span>
                                                    {", Valido por "}
                                                    <span class={classes!("special-text")}>{ if data.days != 0 { data.days.to_string() } else { "∞".to_string() }  }</span>
                                                    {" dia(s)."}
                                                </p>
                                            </>
                                        }
                                    }

                                    else { html! {} }
                                }

                                <button onclick={Callback::from(move |_| show_modal.set(false))}>{"Fechar"}</button>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

        </div>
    }
}
