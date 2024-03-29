use crate::color::Color;
use crate::elements::behaviour::powder_behaviour::powder_behaviour;
use crate::elements::element::{Element, ElementProperties};

#[derive(Clone)]
pub struct SandProperties {
    color: Color,
    visited: bool,
}

impl ElementProperties for SandProperties {
    fn name(&self) -> &str {
        "Sand"
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

impl Default for SandProperties {
    fn default() -> Self {
        SandProperties {
            color: Color::new(239, 221, 111),
            visited: false,
        }
    }
}

pub fn new_sand() -> Element {
    Element {
        properties: Box::<SandProperties>::default(),
        behaviour: powder_behaviour,
    }
}
