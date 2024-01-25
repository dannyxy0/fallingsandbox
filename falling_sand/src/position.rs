use std::ops;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

pub const LEFT: Position = Position { x: -1, y: 0 };
pub const RIGHT: Position = Position { x: 1, y: 0 };
pub const UP: Position = Position { x: 0, y: -1 };
pub const DOWN: Position = Position { x: 0, y: 1 };

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn new_usize(x: usize, y: usize) -> Self {
        Self::new(x as i32, y as i32)
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}
