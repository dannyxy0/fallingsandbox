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

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vectors() {
        assert_eq!(Vector::new(0, 0) + Vector::new(0, 0), Vector::new(0, 0));
        assert_eq!(Vector::new(5, 3) + Vector::new(8, 1), Vector::new(13, 4));
        assert_eq!(
            Vector::new(20, -94) + Vector::new(28, 56),
            Vector::new(48, -38)
        );
        assert_eq!(
            Vector::new(-8, 0) + Vector::new(-17, -3),
            Vector::new(-25, -3)
        );
        assert_eq!(Vector::new(-0, -0) + Vector::new(0, 0), Vector::new(0, 0));
    }

    #[test]
    fn test_sub_vectors() {
        assert_eq!(Vector::new(0, 0) - Vector::new(0, 0), Vector::new(0, 0));
        assert_eq!(Vector::new(5, 3) - Vector::new(8, 1), Vector::new(-3, 2));
        assert_eq!(
            Vector::new(20, -94) - Vector::new(28, 56),
            Vector::new(-8, -150)
        );
        assert_eq!(Vector::new(-8, 0) - Vector::new(-17, -3), Vector::new(9, 3));
        assert_eq!(Vector::new(-0, -0) - Vector::new(0, 0), Vector::new(0, 0));
    }
}
