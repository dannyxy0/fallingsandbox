use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub struct Matrix<T> {
    pub matrix: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        Matrix {
            matrix: vec![default_value; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Vector) -> Result<&T> {
        Ok(&self.matrix[self.index(pos)?])
    }

    pub fn get_mut(&mut self, pos: Vector) -> Result<&mut T> {
        let index = self.index(pos)?;
        Ok(&mut self.matrix[index])
    }

    pub fn set(&mut self, pos: Vector, value: T) -> Result<()> {
        let index = self.index(pos)?;
        self.matrix[index] = value;
        Ok(())
    }

    pub fn in_bounds(&self, pos: Vector) -> bool {
        pos.x >= 0 && pos.x < self.width as isize && pos.y >= 0 && pos.y < self.height as isize
    }

    pub fn swap(&mut self, pos1: Vector, pos2: Vector) -> Result<()> {
        let index1 = self.index(pos1)?;
        let index2 = self.index(pos2)?;
        self.matrix.swap(index1, index2);
        Ok(())
    }

    pub fn fill(&mut self, pos: Vector, size: Vector, value: T) -> Result<()>
    where
        T: Clone,
    {
        self.index(pos + size)?;

        for i in 0..size.x {
            for j in 0..size.y {
                let index = self.index(pos + Vector::new(i, j))?;
                self.matrix[index] = value.clone();
            }
        }

        Ok(())
    }

    fn index(&self, pos: Vector) -> Result<usize> {
        if !self.in_bounds(pos) {
            return Err(anyhow!("Position {pos} is out of bounds"));
        }
        Ok(pos.x as usize + pos.y as usize * self.width) // Casting to usize is safe because pos is in bounds
    }
}

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
