use yew::{function_component, html, use_context, use_effect_with, Html, Properties};
use yew_router::hooks::use_navigator;

use crate::{contexts::admin_authorization::AdminAuthorizationCtx, routes::Route};


// Struct
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RouteGuardProps {
    pub children: Html,
}
 

// Components
#[function_component(AdminRouteGuard)]
pub fn admin_route_guard(props: &RouteGuardProps) -> Html {
    
    // Props
    let admin_authorization_ctx = use_context::<AdminAuthorizationCtx>().unwrap();
    let navigator = use_navigator().unwrap();
    
    // Effects
    {
        let admin_authorization = admin_authorization_ctx.clone();

        use_effect_with(admin_authorization, move |auth| {
            if auth.token_admin.is_none() {
                navigator.push(&Route::Login);
            } 
        });
    }
    
    // View
    html!{
        <>
            { props.children.clone() } 
        </>
    }

}
