#![recursion_limit = "1024"]

mod app;
mod components;
mod routes;

use wasm_bindgen::prelude::*;

use crate::app::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
