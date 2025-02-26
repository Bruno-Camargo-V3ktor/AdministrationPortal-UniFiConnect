use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Structs
#[derive(Deserialize)]
struct ApproverCode {
    pub code: String,
    pub days: u32,
}

// Components
#[function_component(ApproverCodePage)]
pub fn index() -> Html {
    // States
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error_menssage = use_state(|| None::<String>);
    let show_modal = use_state(|| false);
    let approver_code = use_state(|| None::<ApproverCode>);

    // Callbacks
    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error_menssage = error_menssage.clone();
        let show_modal = show_modal.clone();
        let approver_code = approver_code.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username_value = (*username).clone();
            let password_value = (*password).clone();
            let approver_code = approver_code.clone();

            let error_menssage = error_menssage.clone();
            let show_modal = show_modal.clone();

            spawn_local(async move {
                let data = json!({
                    "username": username_value,
                    "password": password_value
                });

                let response = Request::put(".../api/approver/code")
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
    };

    // Render
    html! {
        <div class={classes!("approver-code-page-container")}>
        <div class={classes!("login-container")}>
            <form onsubmit={on_submit}>
                <div class={classes!("input-group")}>
                    <label for="username">{"Usuário:"}</label>
                    <input id="username" type="text" value={(*username).clone()} oninput={on_username_input} />
                </div>

                <div class={classes!("input-group")}>
                    <label for="password">{"Senha:"}</label>
                    <input id="password" type="password" value={(*password).clone()} oninput={on_password_input} />
                </div>

                    <button type="submit">{"Gerar Codigo"}</button>
            </form>

            {
                if let Some(err) = &*error_menssage {
                    html! { <p class="error">{err}</p> }
                } else {
                    html! {}
                }
            }

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
                                                    <span class={classes!("special-text")}>{data.code.clone()}</span>
                                                    {", Valido por "}
                                                    <span class={classes!("special-text")}>{data.days}</span>
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
        </div>
    }
}
