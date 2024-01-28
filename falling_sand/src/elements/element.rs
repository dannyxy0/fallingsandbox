use crate::matrix::Matrix;
use crate::simulation::Cell;
use crate::vector::Vector;
use dyn_clone::{clone_trait_object, DynClone};

pub trait Element: DynClone {
    fn tick(&mut self, pos: Vector, matrix: &mut Matrix<Cell>);
    fn name(&self) -> &str;
    fn color(&self) -> Color;
}
clone_trait_object!(Element);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
