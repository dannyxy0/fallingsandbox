use nalgebra::Vector2;

pub mod color;
pub mod element_api;
pub mod elements;
pub mod simulation;

pub const UP: Vector2<isize> = Vector2::new(0, -1);
pub const DOWN: Vector2<isize> = Vector2::new(0, 1);
pub const LEFT: Vector2<isize> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<isize> = Vector2::new(1, 0);

#[macro_export]
macro_rules! idx {
    ($index:expr) => {
        ($index[0], $index[1])
    };
}
