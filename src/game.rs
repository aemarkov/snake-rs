use std::cell;

use wasm_bindgen::prelude::*;
use log;
use crate::coord::Direction;
use crate::draw::Draw;
use crate::field::Field;
use crate::snake::Snake;


/// Main game context
#[wasm_bindgen]
pub struct Game {
    draw: Draw,
    field: Field,
    snake: Snake,
}

fn get_direction(e: web_sys::KeyboardEvent) -> Option<Direction> {
    match e.key().as_str() {
        "ArrowUp" => Some(Direction::Up),
        "ArrowDown" => Some(Direction::Down),
        "ArrowLeft" => Some(Direction::Left),
        "ArrowRight" => Some(Direction::Right),
        _ => None,
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        log::info!("Game initialized");
        let mut game = Game {
            field: Field::new(10, 10),
            draw: Draw::new("canvas_game"),
            snake: Snake::new()
        };

        game.snake.increase(&mut game.field);
        game.snake.increase(&mut game.field);
        game.snake.add_to_field(&mut game.field);
        game.draw.draw(&game.field);

        return game;
    }

    // Key is pressed. External event from JS
    pub fn key_down(&mut self, e: web_sys::KeyboardEvent) {
        match get_direction(e) {
            Some(direction) => {
                self.snake.rotate(direction);
                self.snake.shift(&mut self.field);
                self.draw.draw(&self.field);
            },
            _ => return
        }
    }

    // Game tick. External event from JS
    pub fn update(&mut self) {
        // self.snake.shift(&mut self.field);
        // self.draw.draw(&self.field);
    }
}
