use crate::element_api::ElementApi;
use crate::elements::element::Element;
use crate::idx;
use nalgebra::{DMatrix, Dim, Dyn, VecStorage, Vector2};
use rand_core::SeedableRng;
use rand_xoshiro::SplitMix64;

pub type ElementMatrix = DMatrix<Option<Element>>;

pub struct Simulation {
    pub matrix: ElementMatrix,
    tick_visit: bool,
    rng: SplitMix64,
}

impl Simulation {
    pub fn new_with_rand(width: usize, height: usize, rng: SplitMix64) -> Self {
        let data = VecStorage::new(
            Dyn::from_usize(height),
            Dyn::from_usize(width),
            vec![None; width * height],
        );
        Simulation {
            matrix: ElementMatrix::from_data(data),
            tick_visit: false,
            rng,
        }
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_rand(width, height, SplitMix64::from_entropy())
    }

    pub fn tick(&mut self) {
        for i in 0..self.matrix.ncols() {
            for j in (0..self.matrix.nrows()).rev() {
                let pos = Vector2::new(i, j);
                let cell = &mut self.matrix[idx!(pos)];
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
