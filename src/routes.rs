use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{approver_code::ApproverCodePage, login_page::LoginPage};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/approver/code")]
    ApproverGeneratedCode,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <></> },
        Route::Login => html! { <> <LoginPage /> </> },
        Route::ApproverGeneratedCode => html! { <> <ApproverCodePage /> </> },
    }
}

#[function_component(Routes)]
pub fn routes() -> Html {
    html! {
        <BrowserRouter basename="/admin">
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
