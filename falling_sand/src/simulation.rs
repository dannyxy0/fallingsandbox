use crate::cell::Cell;
use crate::matrix::Matrix;

pub struct Simulation<'a> {
    pub matrix: Matrix<Cell<'a>>
}

impl Simulation<'_> {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            matrix: Matrix::new(width, height, &Cell::default())
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.matrix.width() {
            for j in 0..self.matrix.height() {
                let _ = self.matrix.get(i, j);
            }
        }
    }
}
