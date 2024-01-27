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

    pub fn get(&self, pos: Position<usize>) -> Result<&T> {
        self.matrix.get(self.index(pos))
            .ok_or(anyhow!("Position {pos} is out of bounds"))
    }

    pub fn set(&mut self, pos: Position<usize>, value: T) -> Result<()> {
        let index = self.index(pos);
        if index > self.matrix.len() { return Err(anyhow!("Position {pos} is out of bounds")); }

        self.matrix[index] = value;
        Ok(())
    }

    pub fn fill(&mut self, pos: Position<usize>, size: Position<usize>, value: T) -> Result<()>
        where T: Clone {
        let max_pos = self.index(pos + size);
        if max_pos >= self.matrix.len() { return Err(anyhow!("Fill is partially out of bounds")); }

        for i in 0..size.x {
            for j in 0..size.y {
                self.set(Position::new(i, j), value.clone())?;
            }
        }

        Ok(())
    }

    fn index(&self, pos: Position<usize>) -> usize {
        pos.x + pos.y * self.width
    }
}

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

impl<T> Add for Position<T>
    where T: Add<Output=T> {
    type Output = Position<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Position<T>
    where T: Sub<Output=T> {
    type Output = Position<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Display for Position<T>
    where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
