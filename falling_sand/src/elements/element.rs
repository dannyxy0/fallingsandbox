use crate::color::Color;
use crate::element_api::ElementApi;
use dyn_clone::{clone_trait_object, DynClone};

pub trait ElementProperties: DynClone {
    fn name(&self) -> &str;
    fn color(&self) -> Color;

    /// Used to keep track of whether the element has been processed this tick
    fn visited(&self) -> bool;
    fn set_visited(&mut self, value: bool);

    /// Decides whether two elements can swap. Bigger number can swap with smaller.
    ///
    /// Around 42 is typical for a gas
    /// Around 127 is typical for a liquid
    /// Around 212 is typical for a solid
    fn swap_priority(&self) -> u8;
}
clone_trait_object!(ElementProperties);

#[derive(Clone)]
pub struct Element {
    pub properties: Box<dyn ElementProperties + Sync + Send>,
    pub behaviour: fn(api: ElementApi),
}
