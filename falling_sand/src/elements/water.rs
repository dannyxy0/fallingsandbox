use crate::color::Color;
use crate::elements::behaviour::liquid_behaviour::liquid_behaviour;
use crate::elements::element::{Element, ElementProperties};

#[derive(Clone)]
pub struct WaterProperties {
    color: Color,
    visited: bool,
}

impl ElementProperties for WaterProperties {
    fn name(&self) -> &str {
        "Water"
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
        42
    }
}

impl Default for WaterProperties {
    fn default() -> Self {
        WaterProperties {
            color: Color::new(29, 162, 255),
            visited: false,
        }
    }
}

pub fn new_water() -> Element {
    Element {
        properties: Box::<WaterProperties>::default(),
        behaviour: liquid_behaviour,
    }
}
