mod app;
mod pages;
mod routes;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
