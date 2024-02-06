use crate::elements::element::Element;
use crate::matrix::Matrix;
use crate::vector::Vector;

pub type Cell = Option<Element>;

pub struct Simulation {
    pub matrix: Matrix<Cell>,
    tick_visit: bool,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            matrix: Matrix::new(width, height, None),
            tick_visit: false,
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.matrix.width() {
            for j in (0..self.matrix.height()).rev() {
                let pos = Vector::new_usize(i, j);
                let cell = self.matrix.get_mut(pos).expect("pos is in bounds");
                match cell {
                    None => (),
                    Some(element) => {
                        if element.properties.visited() != self.tick_visit {
                            continue;
                        }

                        (element.behaviour)(pos, &mut self.matrix);
                    }
                }
            }
        }

        self.tick_visit = !self.tick_visit;
    }
}
