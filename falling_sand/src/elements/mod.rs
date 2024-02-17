pub mod element;
pub mod liquid_behaviour;
pub mod powder_behaviour;
pub mod sand;
pub mod water;

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::elements::element::{Element, ElementProperties};

    #[derive(Clone)]
    pub struct NonMovingProperties<'a> {
        pub name: &'a str,
        pub visited: bool,
    }

    impl<'a> ElementProperties for NonMovingProperties<'a> {
        fn name(&self) -> &str {
            self.name
        }

        fn color(&self) -> Color {
            Color::black()
        }

        fn visited(&self) -> bool {
            self.visited
        }

        fn set_visited(&mut self, value: bool) {
            self.visited = value;
        }

        fn swap_priority(&self) -> u8 {
            255
        }
    }

    pub fn new_non_moving() -> Element {
        Element {
            properties: Box::new(NonMovingProperties {
                name: "NonMoving",
                visited: false,
            }),
            behaviour: |_| {},
        }
    }

    pub fn new_marked(name: &'static str) -> Element {
        Element {
            properties: Box::new(NonMovingProperties {
                name,
                visited: false,
            }),
            behaviour: |_| {},
        }
    }

    pub fn name_from_cell(cell: &Option<Element>) -> String {
        cell.clone().unwrap().properties.name().to_string()
    }

    pub fn first_name(cells: Vec<&Option<Element>>) -> String {
        for cell in cells {
            if let Some(element) = cell {
                return element.properties.name().to_string();
            }
        }

        panic!("No cells contain elements")
    }
}
