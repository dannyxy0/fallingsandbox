use crate::elements::element::Element;
use crate::matrix::Matrix;
use crate::vector::Vector;

pub type Cell = Option<Element>;

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
            for j in (0..self.matrix.height()).rev() {
                let pos = Vector::new(i as isize, j as isize);
                let cell = self.matrix.get(pos).expect("pos is in bounds");
                match cell {
                    None => (),
                    Some(element) => (element.behaviour)(pos, &mut self.matrix),
                }
            }
        }
    }
}
