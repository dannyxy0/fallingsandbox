use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

pub const DOWN: Vector = Vector { x: 0, y: 1 };
pub const UP: Vector = Vector { x: 0, y: -1 };
pub const LEFT: Vector = Vector { x: -1, y: 0 };
pub const RIGHT: Vector = Vector { x: 1, y: 0 };

impl Vector {
    pub fn new(x: isize, y: isize) -> Self {
        Vector { x, y }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
