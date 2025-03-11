mod app;
mod pages;
mod routes;
mod components;
mod contexts;
mod http;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
