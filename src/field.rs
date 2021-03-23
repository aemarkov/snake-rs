use std::usize;

/// Type of the field cell
#[derive(Debug, Copy, Clone)]
pub enum CellType {
    Empty,
    Snake,
    Food
}

/// Cell of the field
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    cell_type: CellType
}

impl Cell {
    fn new() -> Cell {
        Cell {cell_type: CellType::Empty }
    }
}


/// Game field
pub struct Field {
    pub width: u32,
    pub height: u32,
    data: Vec<Cell>
}

impl Field {
    /// Create new Field with given size (width, height) in cells
    pub fn new(width: u32, height: u32) -> Field {
        Field {
            width, height,
            data: vec![Cell::new(); (width * height) as usize]
        }
    }
}

/// Immutable access to field cells
impl std::ops::Index<(u32, u32)> for Field {
    type Output = CellType;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.data[(index.0 * self.width + index.1) as usize].cell_type
    }
}

/// Mutable access to field cells
// TODO: Maybe need a better approach
impl std::ops::IndexMut<(u32, u32)> for Field {
    fn index_mut(&mut self, index: (u32, u32)) ->&mut Self::Output {
        &mut self.data[(index.0 * self.width + index.1) as usize].cell_type
    }
}
