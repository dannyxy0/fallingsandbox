use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use anyhow::{anyhow, Result};

pub struct Matrix<T> {
    matrix: Vec<T>,
    width: usize,
    height: usize
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self
        where T: Clone {
        Matrix {
            matrix: vec![default_value; width * height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Position) -> Result<&T> {
        Ok(&self.matrix[self.index(pos)?])
    }

    pub fn set(&mut self, pos: Position, value: T) -> Result<()> {
        let index = self.index(pos)?;
        self.matrix[index] = value;
        Ok(())
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width as isize &&
        pos.x >= 0 && pos.x < self.width as isize
    }

    pub fn fill(&mut self, pos: Position, size: Position, value: T) -> Result<()>
        where T: Clone {
        self.index(pos + size)?;

        for i in 0..size.x {
            for j in 0..size.y {
                let index = self.index(pos + Position::new(i, j))?;
                self.matrix[index] = value.clone();
            }
        }

        Ok(())
    }

    fn index(&self, pos: Position) -> Result<usize> {
        if !self.in_bounds(pos) { return Err(anyhow!("Position {pos} is out of bounds")) }
        Ok(pos.x as usize + pos.y as usize * self.width) // Casting to usize is safe because pos is in bounds
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

pub const DOWN: Position = Position { x: 0, y: 1 };
pub const UP: Position = Position { x: 0, y: -1 };
pub const LEFT: Position = Position { x: -1, y: 0 };
pub const RIGHT: Position = Position { x: 1, y: 0 };

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
