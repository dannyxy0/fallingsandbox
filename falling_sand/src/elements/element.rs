use anyhow::Result;
use crate::position::Position;
use crate::simulation::Simulation;

pub trait Element {
    fn name(&self) -> &str;
    fn tick(&self, pos: Position, simulation: &mut Simulation) -> Result<()>;
}
