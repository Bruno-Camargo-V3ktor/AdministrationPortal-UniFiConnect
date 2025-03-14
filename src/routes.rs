use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::admin_route_guard::AdminRouteGuard, pages::{approver_code::ApproverCodePage, login_page::LoginPage, pending_page::PendingPage}};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pending")]
    Pending,
    #[at("/login")]
    Login,
    #[at("/approver/code")]
    ApproverGeneratedCode,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html { 
    match routes {
        Route::Home => html! {  <AdminRouteGuard> <></> </AdminRouteGuard> },
        Route::Pending => html! { <AdminRouteGuard> <PendingPage /> </AdminRouteGuard> },
        Route::Login => html! { <> <LoginPage /> </> },
        Route::ApproverGeneratedCode => html! { <> <ApproverCodePage /> </> },

        Route::NotFound => html! { <AdminRouteGuard> <> <h1>{"Not Found"}</h1> </> </AdminRouteGuard> }
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
