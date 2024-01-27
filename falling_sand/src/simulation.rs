use crate::elements::element::Element;
use crate::matrix::Matrix;

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
}
