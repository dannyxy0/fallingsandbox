use crate::color::Color;
use crate::elements::element::{Element, ElementProperties};
use crate::elements::powder_behaviour::powder_behaviour;

#[derive(Clone)]
pub struct SandProperties {
    color: Color,
}

impl ElementProperties for SandProperties {
    fn name(&self) -> &str {
        "Sand"
    }

    fn color(&self) -> Color {
        self.color
    }
}

impl Default for SandProperties {
    fn default() -> Self {
        SandProperties {
            color: Color::new(239, 221, 111),
        }
    }
}

pub fn new_sand() -> Element {
    Element {
        properties: Box::<SandProperties>::default(),
        behaviour: powder_behaviour,
    }
}
