use std::usize;

/// Type of the field cell
#[derive(Debug, Copy, Clone)]
pub enum CellType { Empty, Snake, Food }

/// Snake moving direction
#[derive(Debug, Copy, Clone)]
pub enum Direction { Unknow, Up, Left, Down, Right }

/// Cell of the field
#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub direction: Direction,
}

impl Cell {
    fn new() -> Cell {
        Cell {cell_type: CellType::Empty, direction: Direction::Unknow }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord
{
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord { Coord {x, y } }
    pub fn zero() -> Coord { Coord {x: 0, y: 0} }
    pub fn up(&self) -> Coord { Coord { x: self.x, y: self.y - 1 }}
    pub fn left(&self) -> Coord { Coord { x: self.x - 1, y: self.y }}
    pub fn down(&self) -> Coord { Coord { x: self.x, y: self.y + 1 }}
    pub fn right(&self) -> Coord { Coord { x: self.x + 1, y: self.y }}
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

    pub fn shift_coord(&self, coord: Coord) -> Option<Coord> {
        match self[coord].direction {
            Direction::Unknow => Some(coord),
            Direction::Up => self.coord_if_ok(coord.up()),
            Direction::Left => self.coord_if_ok(coord.left()),
            Direction::Down => self.coord_if_ok(coord.down()),
            Direction::Right => self.coord_if_ok(coord.right())
        }
    }

    fn check_coord(&self, coord: Coord) -> bool {
        coord.x < 0 || coord.x as u32 >= self.width || coord.y < 0 || coord.y as u32 >= self.height
    }

    fn coord_if_ok(&self, coord: Coord) -> Option<Coord>
    {
        if self.check_coord(coord) { None } else { Some(coord) }
    }
}

/// Immutable access to field cells
impl std::ops::Index<Coord> for Field {
    type Output = Cell;

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
