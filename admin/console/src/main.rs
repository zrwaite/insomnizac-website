mod app;
mod pages;
mod utils;
mod models;
mod components;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
