use std::cell;

use wasm_bindgen::prelude::*;
use log;
use crate::draw::*;
use crate::field::*;

struct Snake {
    head: Coord,
    tail: Coord,
}

/// Main game context
#[wasm_bindgen]
pub struct Game {
    draw: Draw,
    field: Field,
    snake: Snake,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        log::info!("Game initialized");
        let mut game = Game {
            field: Field::new(10, 10),
            draw: Draw::new("canvas_game"),
            snake: Snake {
                tail: Coord::new(0, 0),
                head: Coord::new(1, 0),
            }
        };

        // TODO: Better snake initialization
        game.field[Coord::new(0, 0)] = Cell {
            cell_type: CellType::Snake,
            direction: Direction::Right
        };
        game.field[Coord::new(1, 0)] = Cell {
            cell_type: CellType::Snake,
            direction: Direction::Right
        };
        game.draw.draw(&game.field);

        return game;
    }

    // Key is pressed. External event from JS
    pub fn key_down(&self, e: web_sys::KeyboardEvent) {
        log::info!("key pressed");
    }

    // Game tick. External event from JS
    pub fn update(&mut self) {
        if self.shift_snake().is_none() {
            log::info!("killed")
        }
        self.draw.draw(&self.field);
    }

    fn shift_snake(&mut self) -> Option<()>{
        log::debug!("Shifted");
        let mut coord = self.snake.tail;
        let mut cell = self.field[coord];
        let empty_cell = Cell {
            cell_type: CellType::Empty,
            direction: Direction::Unknow
        };

        let next_head = self.field.shift_coord(self.snake.head)?;
        let next_tail = self.field.shift_coord(self.snake.tail)?;

        log::debug!("coord: {:?}, next_head {:?}", coord, next_head);

        while coord != next_head {
            log::debug!("Coord: {:?}", coord);
            let next_coord = self.field.shift_coord(coord)?;
            let next_cell = self.field[next_coord];
            self.field[coord] = empty_cell;
            self.field[next_coord] = cell;
            coord = next_coord;
            cell = next_cell;
        }

        self.snake.head = next_head;
        self.snake.tail = next_tail;

        Some(())
    }
}
