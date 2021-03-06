mod app;
mod api;
mod models;
mod service;
pub mod components;
mod hooks;
pub mod router;
pub mod graphql;

use wasm_bindgen::prelude::*;
use crate::app::App;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}