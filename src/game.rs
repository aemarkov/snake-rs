use wasm_bindgen::prelude::*;
use log;


#[wasm_bindgen]
pub struct Game {

}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        log::info!("Game initialized");
        Game {}
    }

    pub fn shit() {
        log::info!("Game");
    }

    pub fn key_down(&self, e: web_sys::KeyboardEvent) {
        log::info!("key pressed");
    }
}

impl std::ops::Drop for Game {
    fn drop(&mut self) {
        log::info!("Drop game");
        panic!();
    }
}
