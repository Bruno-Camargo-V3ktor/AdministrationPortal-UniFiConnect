use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, Properties, SubmitEvent, TargetCast, classes};

// Props
#[derive(Properties, PartialEq)]
pub struct FormLoginProps {
    pub on_handle_click: Callback<(String, String), Callback<SubmitEvent>>,
    pub error_menssage: Option<String>,
    pub btn_text: String,
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
            <div class={classes!("flex", "flex-col", "items-center", "justify-center", "bg-slate-200", "p-8", "rounded-lg", "h-96", "max-h-96", "w-96", "max-w-96", "font-mono")}>

                <form 
                    class={classes!("flex", "flex-col", "w-full", "h-full", "m-4", "gap-8")}
                    onsubmit={ props.on_handle_click.emit( ( (*username).clone(), (*password).clone() ) ) }
                >

                    <div class={classes!("self-stretch", "w-full", "mt-auto")}>
                        <label class={classes!("flex", "items-center", "gap-1", "before:flex", "before:h-3", "before:w-0.5", "before:bg-gray-700", "before:rounded-full")} for="username">
                            {"Usuário"}
                        </label>

                        <input
                            class={ classes!("border-2", "bg-slate-300", "w-full", "rounded", "text-lg", "h-11", "p-1", "font-thin", "border-slate-600") }
                            id="username" type="text" value={(*username).clone()} oninput={on_username_input} 
                        />
                    </div>

                    <div class={classes!("self-stretch", "w-full")}>
                        <label class={classes!("flex", "items-center", "gap-1", "before:flex", "before:h-3", "before:w-0.5", "before:bg-gray-700", "before:rounded-full")} for="password">
                            {"Senha"}
                        </label>

                        <input
                            class={classes!("border-2", "bg-slate-300", "w-full", "rounded", "text-lg", "h-11", "p-1", "font-thin", "border-slate-600")}
                            id="password" type="password" value={(*password).clone()} oninput={on_password_input} 
                        />
                    </div>

                    <button 
                        class={ classes!("mt-auto", "w-full", "bg-green-600", "p-2", "rounded", "hover:bg-green-500", "text-white", "font-bold", "text-lg") } 
                        type="submit"
                    >
                        { props.btn_text.clone() }
                    </button>
                </form>

                {
                    if let Some(err) = &props.error_menssage {
                        html! { 
                            <p class={classes!("text-sm", "font-bold", "italic", "text-red-400", "text-center")}>
                                {err}
                            </p> 
                        }
                    } else {
                        html! {}
                    }
                }

            </ div>
        </>
    }
}
