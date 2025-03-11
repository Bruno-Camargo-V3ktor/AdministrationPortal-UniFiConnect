mod app;
mod pages;
mod routes;
mod components;
mod contexts;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
