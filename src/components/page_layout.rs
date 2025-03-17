use yew::{classes, function_component, html, Html, Properties};


// Struct
#[derive(Properties, PartialEq, Debug)]
pub struct PropsPageLayout {
    pub children: Html
}

// Components
#[function_component(PageAdminLayout)]
pub fn page_admin_layout(props: &PropsPageLayout) -> Html {
    

    // View
    html! {
        <>
            <div class={classes!("min-h-full", "flex")}>
                <div class={classes!("flex", "flex-col", "min-h-full", "min-w-[6rem]", "bg-orange-500", "shadow-2xl")}>
                
                </div>

                { props.children.clone() }
            </div> 
        </>
    }

}
