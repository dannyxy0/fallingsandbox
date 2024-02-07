use crate::matrix::Matrix;
use crate::simulation::Cell;
use crate::vector::{Vector, DOWN, LEFT, RIGHT};

pub fn liquid_behaviour(pos: Vector, matrix: &mut Matrix<Cell>) {
    if let Ok(Some(element)) = matrix.get_mut(pos) {
        element
            .properties
            .set_visited(!element.properties.visited());
    }

    let left = pos + LEFT;
    let right = pos + RIGHT;
    let bottom = pos + DOWN;
    let bottom_left = bottom + LEFT;
    let bottom_right = bottom + RIGHT;

    if matrix.get(bottom).is_ok_and(Option::is_none) {
        let _ = matrix.swap(pos, bottom);
    } else if matrix.get(bottom_left).is_ok_and(Option::is_none) {
        let _ = matrix.swap(pos, bottom_left);
    } else if matrix.get(bottom_right).is_ok_and(Option::is_none) {
        let _ = matrix.swap(pos, bottom_right);
    } else if matrix.get(left).is_ok_and(Option::is_none) {
        let _ = matrix.swap(pos, left);
    } else if matrix.get(right).is_ok_and(Option::is_none) {
        let _ = matrix.swap(pos, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::tests::{new_marked, new_non_moving};

    #[test]
    fn test_tick_fall_bottom() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert_eq!(
            matrix.matrix[7].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_fall_either_side() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[7].is_some());
        let left_or_right = matrix.matrix[6]
            .clone()
            .unwrap_or_else(|| matrix.matrix[8].clone().unwrap());
        assert_eq!(left_or_right.properties.name(), "test object");
    }

    #[test]
    fn test_tick_fall_left() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
        assert_eq!(
            matrix.matrix[6].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_fall_right() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[6] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert_eq!(
            matrix.matrix[8].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_move_either_side() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());

        let left_or_right = matrix.matrix[3]
            .clone()
            .unwrap_or_else(|| matrix.matrix[5].clone().unwrap());
        assert_eq!(left_or_right.properties.name(), "test object");
    }

    #[test]
    fn test_tick_move_left() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());
        matrix.matrix[5] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
        assert!(matrix.matrix[5].is_some());
        assert_eq!(
            matrix.matrix[3].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_move_right() {
        let mut matrix = Matrix::<Cell>::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());
        matrix.matrix[3] = Some(new_non_moving());

        liquid_behaviour(Vector::new(1, 1), &mut matrix);

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
        assert!(matrix.matrix[3].is_some());
        assert_eq!(
            matrix.matrix[5].clone().unwrap().properties.name(),
            "test object"
        );
    }
}
