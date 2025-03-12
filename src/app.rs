use crate::{contexts::admin_authorization::AdminAuthorizationContext, routes::Routes};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <AdminAuthorizationContext>
                <Routes />
            </ AdminAuthorizationContext>
        </main>
    }
}
