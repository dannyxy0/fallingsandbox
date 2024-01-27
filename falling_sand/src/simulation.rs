use crate::elements::element::Element;
use crate::matrix::{Matrix, Position};

pub type Cell = Option<Box<dyn Element>>;

pub struct Simulation {
    pub matrix: Matrix<Cell>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            matrix: Matrix::new(width, height, None),
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.matrix.width() {
            for j in 0..self.matrix.height() {
                let pos = Position::new(i as isize, j as isize);
                let _cell = self.matrix.get_mut(pos).expect("Position is in bounds");
            }
        }
    }
}
