use std::usize;
use crate::coord::Coord;

/// Type of the field cell
#[derive(Debug, Copy, Clone)]
pub enum CellType { Empty, Snake, Food }

/// Game field
pub struct Field {
    pub width: u32,
    pub height: u32,
    data: Vec<CellType>
}

impl Field {
    /// Create new Field with given size (width, height) in cells
    pub fn new(width: u32, height: u32) -> Field {
        let field = Field {
            width, height,
            data: vec![CellType::Empty; (width * height) as usize]
        };
        log::info!("Field {}x{} initialized", width, height);
        return field;
    }

    pub fn check_bounds(&self, coord: Coord) -> Option<Coord> {
        if coord.x >= 0 && (coord.x as u32) < self.width && coord.y >= 0 && (coord.y as u32) < self.height
        { Some(coord) }  else {None}
    }

    pub fn check_collision(&self, coord: Coord) -> Option<Coord> {
        self.check_bounds(coord).and_then(|c|self._check_collision(c))
    }

    fn _check_collision(&self, coord: Coord) -> Option<Coord> {
        match self[coord] {
            CellType::Snake => None,
            _ => Some(coord)
        }
    }
}

/// Immutable access to field cells
impl std::ops::Index<Coord> for Field {
    type Output = CellType;

    fn index(&self, index: Coord) -> &Self::Output {
        // fuck rust
        &self.data[index.x as usize * self.width as usize + index.y as usize]
    }
}

/// Mutable access to field cells
// TODO: Maybe need a better approach
impl std::ops::IndexMut<Coord> for Field {
    fn index_mut(&mut self, index: Coord) ->&mut Self::Output {
        // fuck rust
        &mut self.data[index.x as usize * self.width as usize + index.y as usize]
    }
}
