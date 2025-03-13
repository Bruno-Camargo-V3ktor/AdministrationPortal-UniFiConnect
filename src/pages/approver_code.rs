use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{components::form_login::FormLogin, http::connect_api::{ErrorReq, UnifiConnect}, models::approver::ApproverCode};

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
                
                error_menssage.set(None);

                spawn_local(async move {
                    let res = UnifiConnect::generate_approver_code(&username, &password).await;
                    
                    match res {
                        Ok(op) => match op {
                            Some(ap_code) => {
                                approver_code.set( Some(ap_code) );
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
        <div class={classes!("bg-gradient-to-r", "from-orange-500", "to-orange-400", "h-screen", "w-screen", "flex", "justify-center", "items-center")}> 
            {
                if *show_modal {
                    html! {
                        <div class={classes!("flex", "flex-col", "items-center", "justify-center", "bg-slate-200", "p-8", "rounded-lg", "h-96", "max-h-96", "w-96", "max-w-96", "font-mono")}>
                            <div class={classes!("flex", "flex-col", "w-full", "h-full", "m-4", "gap-8", "text-center")} >
                                {
                                    if let Some(data) = &*approver_code {
                                        html! {
                                            <>
                                                <h2 class={classes!("text-center", "font-bold", "text-2xl")}>
                                                    {"Codigo Gerado !"}
                                                </h2>
                                                <p class={classes!("mt-auto")}>
                                                    {"O novo codigo eh "}
                                                    <span class={classes!("font-bold", "italic", "text-center", "text-yellow-600")}>
                                                        {data.new_code.clone()}
                                                    </span>

                                                    <br />

                                                    {"Valido por "}
                                                    <span class={classes!("font-bold", "italic", "text-center", "text-yellow-600")}>
                                                        { if data.days != 0 { data.days.to_string() } else { "∞".to_string() }  }
                                                    </span>

                                                    {" dia(s)."}
                                                </p>
                                            </>
                                        }
                                    }

                                    else { html! {} }
                                }

                                <button
                                    class={classes!("mt-auto", "w-full", "bg-green-600", "p-2", "rounded", "hover:bg-green-500", "text-white", "font-bold", "text-lg") } 
                                    onclick={Callback::from(move |_| show_modal.set(false))}>
                                    {"Fechar"}
                                </button>

                            </div>
                        </div>
                    }
                }

                else {
                    html! { 
                        <FormLogin error_menssage={(*error_menssage).clone()} on_handle_click={on_submit.clone()} btn_text={"Gerar Código".to_string()} />
                    }
                }

            }

        </div>
    }
}
