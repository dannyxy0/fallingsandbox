use anyhow::{anyhow, Result};
use crate::elements::element::Element;
use crate::position::Position;

pub struct Simulation {
    matrix: Vec<Option<Box<dyn Element>>>,
    width: usize,
    height: usize
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            matrix: vec![None; width * height],
            width,
            height
        }
    }

    fn pos_to_index(&self, pos: Position<i32>) -> Result<usize> {
        if !self.in_bounds(pos) {
            return Err(anyhow!("Failed to convert Position '{pos}' to index as it is not in bounds"));
        }

        // Casting to usize should be no problem because pos is in bounds
        Ok(pos.x as usize + pos.y as usize * self.width)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Position<i32>) -> Result<&Option<Box<dyn Element>>> {

        Ok(&self.matrix[self.pos_to_index(pos)?])
    }

    pub fn in_bounds(&self, pos: Position<i32>) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 &&
        pos.y >= 0 && pos.y < self.height as i32
    }

    pub fn move_and_swap(&mut self, pos1: Position<i32>, pos2: Position<i32>) -> Result<()> {
        let index1 = self.pos_to_index(pos1)?;
        let index2 = self.pos_to_index(pos2)?;
        self.matrix.swap(index1, index2);
        Ok(())
    }
}
