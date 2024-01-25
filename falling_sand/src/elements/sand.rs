use crate::elements::element::Element;

pub struct Sand { }

impl Element for Sand {
    fn name(&self) -> &str { "Sand" }
}
