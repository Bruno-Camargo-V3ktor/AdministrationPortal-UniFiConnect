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

// Impls
impl AdminAuthorization {
    pub fn default() -> Self {
        Self {
            token_admin: None,
            set_token: Callback::from(|_| {})
        }
    } 
}

// Components
#[function_component(AdminAuthorizationContext)]
pub fn admin_authorization_context(props: &PropsAdminAuthorizationContext) -> Html {

    // States
    let ctx = use_state(|| AdminAuthorization::default() );

    // Callbacks
    let set_token_callback = {
        let ctx = ctx.clone();

        Callback::from(move |token: Option<AdminToken>| {
            let ctx = ctx.clone();
            ctx.set( AdminAuthorization { token_admin: token, set_token: ctx.set_token.clone()  } );
        })
    };
    
    
    // Effects
    {
        let ctx = ctx.clone();
        use_effect_with((), move |_| {
            ctx.set( AdminAuthorization { set_token: set_token_callback, token_admin: ctx.token_admin.clone()  } );
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
