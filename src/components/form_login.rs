use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, Properties, SubmitEvent, TargetCast, classes};

// Props
#[derive(Properties, PartialEq)]
pub struct FormLoginProps {
    pub on_handle_click: Callback<(String, String), Callback<SubmitEvent>>,
    pub error_menssage: Option<String>
}

// Components
#[function_component(FormLogin)]
pub fn form_login(props: &FormLoginProps) -> Html {

    // States
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());

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

    // View
    html! {
        <>
            <div class={classes!("login-container")}>
                <form onsubmit={ props.on_handle_click.emit( ( (*username).clone(), (*password).clone() ) ) }>
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
                    if let Some(err) = &props.error_menssage {
                        html! { <p class="error">{err}</p> }
                    } else {
                        html! {}
                    }
                }

            </ div>
        </>
    }
}