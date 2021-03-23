use wasm_bindgen::prelude::*;
use wasm_logger;
use log;

mod game;

#[wasm_bindgen]
pub fn main()
{
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Rust initialized");
}
