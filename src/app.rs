use crate::routes::Routes;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Routes />
        </main>
    }
}
