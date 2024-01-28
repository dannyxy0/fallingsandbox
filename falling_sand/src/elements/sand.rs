use crate::elements::element::{Color, Element};
use crate::matrix::Matrix;
use crate::simulation::Cell;
use crate::vector::{Vector, DOWN, LEFT, RIGHT};

#[derive(Clone)]
pub struct Sand {
    color: Color,
}

impl Element for Sand {
    fn tick(&mut self, pos: Vector, matrix: &mut Matrix<Cell>) {
        let bottom = pos + DOWN;
        let bottom_left = bottom + LEFT;
        let bottom_right = bottom + RIGHT;

        if matrix.get(bottom).is_ok_and(Option::is_none) {
            let _ = matrix.swap(pos, bottom);
        } else if matrix.get(bottom_left).is_ok_and(Option::is_none) {
            let _ = matrix.swap(pos, bottom_left);
        } else if matrix.get(bottom_right).is_ok_and(Option::is_none) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_fall_bottom() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(Box::new(Sand::default()));
        let mut sand = matrix.matrix[4].clone().unwrap();
        sand.tick(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[7].is_some());
    }

    #[test]
    fn test_tick_fall_either_side() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(Box::new(Sand::default()));
        matrix.matrix[7] = Some(Box::new(Sand::default()));

        let mut sand = matrix.matrix[4].clone().unwrap();
        sand.tick(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[6].is_some() || matrix.matrix[8].is_some());
    }

    #[test]
    fn test_tick_fall_left() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(Box::new(Sand::default()));
        matrix.matrix[7] = Some(Box::new(Sand::default()));
        matrix.matrix[8] = Some(Box::new(Sand::default()));

        let mut sand = matrix.matrix[4].clone().unwrap();
        sand.tick(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
    }

    #[test]
    fn test_tick_fall_right() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(Box::new(Sand::default()));
        matrix.matrix[7] = Some(Box::new(Sand::default()));
        matrix.matrix[6] = Some(Box::new(Sand::default()));

        let mut sand = matrix.matrix[4].clone().unwrap();
        sand.tick(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
    }

    #[test]
    fn test_tick_blocked() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(Box::new(Sand::default()));
        matrix.matrix[6] = Some(Box::new(Sand::default()));
        matrix.matrix[7] = Some(Box::new(Sand::default()));
        matrix.matrix[8] = Some(Box::new(Sand::default()));

        let mut sand = matrix.matrix[4].clone().unwrap();
        sand.tick(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_some());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
    }
}
