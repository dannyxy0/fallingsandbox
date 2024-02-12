use crate::element_api::ElementApi;
use crate::elements::element::Element;
use crate::matrix::Matrix;
use crate::vector::Vector;
use rand_core::SeedableRng;
use rand_xoshiro::SplitMix64;

pub type ElementMatrix = Matrix<Option<Element>>;

pub struct Simulation {
    pub matrix: ElementMatrix,
    tick_visit: bool,
    rng: SplitMix64,
}

impl Simulation {
    pub fn new_with_rand(width: usize, height: usize, rng: SplitMix64) -> Self {
        Simulation {
            matrix: Matrix::new(width, height, None),
            tick_visit: false,
            rng,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_rand(width, height, SplitMix64::from_entropy())
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

                        (element.behaviour)(ElementApi::new(&mut self.matrix, &mut self.rng, pos));
                    }
                }
            }
        }

        self.tick_visit = !self.tick_visit;
    }
}
