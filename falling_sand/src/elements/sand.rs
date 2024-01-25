use anyhow::Result;
use crate::elements::element::Element;
use crate::position;
use crate::position::Position;
use crate::simulation::Simulation;

pub struct Sand { }

impl Element for Sand {
    fn name(&self) -> &str { "Sand" }

    fn tick(&self, pos: Position, simulation: &mut Simulation) -> Result<()> {
        let bottom = pos + position::DOWN;
        let bottom_left = bottom + position::LEFT;
        let bottom_right = bottom + position::RIGHT;

        if simulation.in_bounds(pos) && simulation.get(bottom).is_none() {
            simulation.move_and_swap(pos, bottom);
        } else if simulation.in_bounds(pos) && simulation.get(bottom_left).is_none() {
            simulation.move_and_swap(pos, bottom_left)
        } else if simulation.in_bounds(pos) && simulation.get(bottom_right).is_none() {
            simulation.move_and_swap(pos, bottom_right)
        }

        Ok(())
    }
}
