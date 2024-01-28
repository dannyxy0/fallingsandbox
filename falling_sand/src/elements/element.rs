use crate::matrix::Matrix;
use crate::simulation::Cell;
use crate::vector::Vector;
use dyn_clone::{clone_trait_object, DynClone};

pub trait ElementProperties: DynClone {
    fn name(&self) -> &str;
    fn color(&self) -> Color;
}
clone_trait_object!(ElementProperties);

#[derive(Clone)]
pub struct Element {
    pub properties: Box<dyn ElementProperties>,
    pub behaviour: fn(Vector, &mut Matrix<Cell>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
