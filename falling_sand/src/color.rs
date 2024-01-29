#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new_with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color::new_with_alpha(red, green, blue, 255)
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }
}
