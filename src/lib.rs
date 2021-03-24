use wasm_bindgen::prelude::*;
use wasm_logger;
use log;

mod game;
mod draw;
mod field;
mod snake;
mod coord;

#[wasm_bindgen]
pub fn main()
{
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Rust initialized");
}
