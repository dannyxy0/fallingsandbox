use crate::elements::element::{Color, Element};
use crate::matrix::{Matrix, Position, DOWN, LEFT, RIGHT};
use crate::simulation::Cell;

#[derive(Clone)]
pub struct Sand {
    color: Color,
}

impl Element for Sand {
    fn tick(&mut self, pos: Position, matrix: &mut Matrix<Cell>) {
        let bottom = pos + DOWN;
        let bottom_left = bottom + LEFT;
        let bottom_right = bottom + RIGHT;

        if matrix.get(bottom).is_ok_and(Option::is_some) {
            let _ = matrix.swap(pos, bottom);
        } else if matrix.get(bottom_left).is_ok_and(Option::is_some) {
            let _ = matrix.swap(pos, bottom_left);
        } else if matrix.get(bottom_right).is_ok_and(Option::is_some) {
            let _ = matrix.swap(pos, bottom_right);
        }
    }

    fn name(&self) -> &str {
        "Sand"
    }
    fn color(&self) -> Color {
        self.color
    }
}

impl Default for Sand {
    fn default() -> Self {
        Sand {
            color: Color {
                red: 255,
                green: 255,
                blue: 0,
                alpha: 255,
            },
        }
    }
}
