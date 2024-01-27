use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position<T> {
    pub x: T,
    pub y: T
}

pub const DOWN: Position<i32> = Position { x: 0, y: 1 };
pub const UP: Position<i32> = Position { x: 0, y: -1 };
pub const LEFT: Position<i32> = Position { x: -1, y: 0 };
pub const RIGHT: Position<i32> = Position { x: 1, y: 0 };

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Position { x, y }
    }
}

impl<T> Add for Position<T> where T: Add<Output=T> {
    type Output = Position<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Position<T> where T: Sub<Output=T> {
    type Output = Position<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Display for Position<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
