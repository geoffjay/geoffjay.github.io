extern crate yew;
extern crate yew_router;

mod app;
mod components;
mod pages;
mod utils;

use app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
