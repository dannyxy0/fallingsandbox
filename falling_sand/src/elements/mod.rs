pub mod element;
pub mod powder_behaviour;
pub mod sand;

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::elements::element::{Element, ElementProperties};

    #[derive(Clone)]
    pub struct NonMovingProperties<'a> {
        pub name: &'a str,
    }

    impl<'a> ElementProperties for NonMovingProperties<'a> {
        fn name(&self) -> &str {
            self.name
        }

        fn color(&self) -> Color {
            Color::black()
        }
    }

    pub fn new_non_moving() -> Element {
        Element {
            properties: Box::new(NonMovingProperties { name: "NonMoving" }),
            behaviour: |_, _| {},
        }
    }

    pub fn new_marked(name: &'static str) -> Element {
        Element {
            properties: Box::new(NonMovingProperties { name }),
            behaviour: |_, _| {},
        }
    }
}
