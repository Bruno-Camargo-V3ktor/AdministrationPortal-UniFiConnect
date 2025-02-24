use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::approver_code::ApproverCodePage;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/approver/code")]
    ApproverGeneratedCode,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Home"}</h1> },
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
