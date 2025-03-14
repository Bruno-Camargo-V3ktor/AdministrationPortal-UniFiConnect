use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use crate::models::admin::AdminToken;

// Types
pub type AdminAuthorizationCtx = UseStateHandle<AdminAuthorization>;

// Struct
#[derive(Properties, PartialEq)]
pub struct PropsAdminAuthorizationContext {
    pub children: Html,
}

#[ derive(Clone, Debug, PartialEq) ]
pub struct AdminAuthorization {
    pub token_admin: Option<AdminToken>,
    pub set_token: Callback<Option<AdminToken>, ()>,
} 


// Components
#[function_component(AdminAuthorizationContext)]
pub fn admin_authorization_context(props: &PropsAdminAuthorizationContext) -> Html {

    // States
    let ctx = use_state(|| {
        let token_str: String = LocalStorage::get("token").unwrap_or("".to_string());
        let admin_token = if token_str.is_empty() { None } else { Some( AdminToken { token: token_str } ) };
        AdminAuthorization { set_token: Callback::from(|_| {}), token_admin: admin_token  }
    } );


    // Callbacks
    let set_token_callback = {
        let ctx = ctx.clone();

        Callback::from(move |token: Option<AdminToken>| {
            let ctx = ctx.clone();
            ctx.set( AdminAuthorization { token_admin: token.clone(), set_token: ctx.set_token.clone()  });
            
            match &token {
                Some(t) => {
                    let _ = LocalStorage::set("token", t.token.clone());
                },

                None => {
                    LocalStorage::delete("token");
                }
            }

        })
    };
    
    
    // Effects
    {
        let ctx = ctx.clone();
        use_effect_with((), move |_| {
            let token_str: String = LocalStorage::get("token").unwrap_or("".to_string());
            let admin_token = if token_str.is_empty() { None } else { Some( AdminToken { token: token_str } ) };
            ctx.set( AdminAuthorization { set_token: set_token_callback, token_admin: admin_token  } );
        });
    }

    // View
    html! {
        <>
            <ContextProvider< UseStateHandle<AdminAuthorization> > context={ ctx.clone() } >
                { props.children.clone() }
            </ContextProvider< UseStateHandle<AdminAuthorization>  > >
        </>
    }
}
