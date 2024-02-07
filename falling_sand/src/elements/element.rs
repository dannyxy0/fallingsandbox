use crate::color::Color;
use crate::simulation::ElementMatrix;
use crate::vector::Vector;
use dyn_clone::{clone_trait_object, DynClone};

pub trait ElementProperties: DynClone {
    fn name(&self) -> &str;
    fn color(&self) -> Color;

    /// Used to keep track of whether the element has been processed this tick
    fn visited(&self) -> bool;
    fn set_visited(&mut self, value: bool);
}
clone_trait_object!(ElementProperties);

#[derive(Clone)]
pub struct Element {
    pub properties: Box<dyn ElementProperties>,
    pub behaviour: fn(Vector, &mut ElementMatrix),
}
