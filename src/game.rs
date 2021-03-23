use wasm_bindgen::prelude::*;
use log;
use crate::draw::*;
use crate::field::*;


/// Main game context
#[wasm_bindgen]
pub struct Game {
    draw: Draw,
    field: Field
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        log::info!("Game initialized");
        let mut field = Field::new(10, 10);
        let draw = Draw::new("canvas_game");
        field[(0, 0)] = CellType::Snake;
        field[(1, 1)] = CellType::Food;
        draw.draw(&field);
        Game { draw, field }
    }

    // Key is pressed. External event from JS
    pub fn key_down(&self, e: web_sys::KeyboardEvent) {
        log::info!("key pressed");
    }

    // Game tick. External event from JS
    pub fn update(&self) {
        log::info!("update");
    }
}
