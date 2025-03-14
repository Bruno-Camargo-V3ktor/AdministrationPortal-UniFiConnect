use crate::{contexts::admin_authorization::AdminAuthorizationContext, routes::Routes};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class={classes!("w-dvw", "h-dvh")}>
            <AdminAuthorizationContext>
                <Routes />
            </ AdminAuthorizationContext>
        </main>
    }
}
