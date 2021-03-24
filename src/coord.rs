#[derive(Debug, Copy, Clone)]
pub enum Direction { Unknow, Up, Left, Down, Right }

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord
{
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord { Coord {x, y } }
    pub fn zero() -> Coord { Coord {x: 0, y: 0} }

    pub fn shift(&self, direction: Direction) -> Coord {
        match direction {
            Direction::Up => Coord { x: self.x, y: self.y - 1},
            Direction::Down => Coord { x: self.x, y: self.y + 1},
            Direction::Left => Coord { x: self.x - 1, y: self.y},
            Direction::Right => Coord { x: self.x + 1, y: self.y},
            _ => Coord { x: self.x, y: self.y }
        }
    }
}
