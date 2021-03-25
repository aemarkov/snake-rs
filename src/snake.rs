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
            field[cell.coord] = CellType::Head;
        }
    }

    // FIXME: it's really bad idea to pass Field to the Snake
    /// Shift snake one cell ahead
    pub fn shift(&mut self, field: &mut Field) -> Option<()> {
        let tail_coord = self.tail().coord;
        let new_head_coord = self.move_head_coord(field)?;

        // move body
        for i in 0..(self.data.len()-1) {
            self.data[i].coord = self.data[i+1].coord;
            field[self.data[i].coord] = CellType::Snake;
        }

        // move head to the direction
        self.head_mut().coord = new_head_coord;
        field[new_head_coord] = CellType::Head;

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
        let head_coord = self.head().coord;
        let new_coord = self.move_head_coord(field)?;
        self.data.push(SnakeCell{coord: new_coord});
        field[head_coord] = CellType::Snake;
        field[new_coord] = CellType::Head;
        Some(())
    }

    pub fn check_food(&self, field: &Field) -> Option<bool> {
        let new_coord = self.move_head_coord(&field)?;
        Some(field[new_coord] == CellType::Food)
    }

    fn move_head_coord(&self, field: &Field) -> Option<Coord> {
        let head = self.head();
        let new_coord = head.coord.shift(self.direction);
        field.check_collision(new_coord)
    }

    fn head(&self) -> &SnakeCell {
        // TODO: Don't check at runtime, make invariant
        self.data.last()
            .expect("Snake has zero length")
    }

    fn head_mut(&mut self) ->&mut SnakeCell {
        // TODO: Don't check at runtime, make invariant
        self.data.last_mut()
            .expect("Snake has zero length")
    }

    fn tail(&self) -> &SnakeCell {
        // TODO: Don't check at runtime, make invariant
        self.data.first()
            .expect("Snake has zero length")
    }
}
