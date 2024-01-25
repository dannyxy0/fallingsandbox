use crate::elements::element::Element;

#[derive(Clone, Default)]
pub enum Cell<'a> {
    #[default] Empty,
    Element(&'a dyn Element)
}
