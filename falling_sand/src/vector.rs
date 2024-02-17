use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn new_usize(x: usize, y: usize) -> Self {
        Vector {
            x: x as isize,
            y: y as isize,
        }
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

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for Vector {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
