use log;
use crate::coord::{Coord, Direction};
use crate::field::{Field, CellType};
use wasm_bindgen::__rt::core::cell::Cell;

struct SnakeCell {
    coord: Coord,
}

pub struct Snake {
    data: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            data: vec![ SnakeCell{ coord: Coord::new(0, 0) } ],
            direction: Direction::Right,
        }
    }

    pub fn add_to_field(&self, field: &mut Field) {
        for cell in &self.data {
            if field.check_bounds(cell.coord).is_none() {
                panic!("Snake initial position is outside the field");
            }
            field[cell.coord] = CellType::Snake;
        }
    }

    // FIXME: it's really bad idea to pass Field to the Snake
    /// Shift snake one cell ahead
    pub fn shift(&mut self, field: &mut Field) -> Option<()> {
        // TODO: Don't check at runtime, make invariant
        let tail_coord = self.data.first()
            .expect("Snake has zero size")
            .coord;

        let new_head_coord = self.move_head_coord(field)?;

        // move body
        for i in 0..(self.data.len()-1) {
            self.data[i].coord = self.data[i+1].coord;
        }

        // move head to the direction
        // array bounds already checked, safe to unwrap
        self.data.last_mut().unwrap().coord = new_head_coord;
        field[new_head_coord] = CellType::Snake;

        // clean the last cell
        field[tail_coord] = CellType::Empty;
        Some(())
    }

    /// Change snake direction (don't change actual position)
    pub fn rotate(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Increase snake length one cell ahead
    pub fn increase(&mut self, field: &mut Field) -> Option<()> {
        let new_coord = self.move_head_coord(field)?;
        self.data.push(SnakeCell{coord: new_coord});
        field[new_coord] = CellType::Snake;
        Some(())
    }

    fn move_head_coord(&self, field: &Field) -> Option<Coord> {
        // TODO: Don't check at runtime, make invariant
        let head = self.data.last()
            .expect("Snake has zero length");
        let new_coord = head.coord.shift(self.direction);
        field.check_collision(new_coord)
    }
}
