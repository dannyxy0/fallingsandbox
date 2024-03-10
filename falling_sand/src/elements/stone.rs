use crate::color::Color;
use crate::elements::behaviour::solid_behaviour::solid_behaviour;
use crate::elements::element::{Element, ElementProperties};

#[derive(Clone)]
pub struct StoneProperties {
    color: Color,
    visited: bool,
}

impl ElementProperties for StoneProperties {
    fn name(&self) -> &str {
        "Stone"
    }

    fn color(&self) -> Color {
        self.color
    }

    fn visited(&self) -> bool {
        self.visited
    }

    fn set_visited(&mut self, value: bool) {
        self.visited = value;
    }

    fn swap_priority(&self) -> u8 {
        212
    }
}

impl Default for StoneProperties {
    fn default() -> Self {
        StoneProperties {
            color: Color::new(65, 64, 64),
            visited: false,
        }
    }
}

pub fn new_stone() -> Element {
    Element {
        properties: Box::<StoneProperties>::default(),
        behaviour: solid_behaviour,
    }
}
