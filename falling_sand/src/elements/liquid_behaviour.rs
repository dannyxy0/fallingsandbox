use crate::element_api::ElementApi;
use crate::vector::{DOWN, LEFT, RIGHT};

#[allow(clippy::short_circuit_statement)]
pub fn liquid_behaviour(mut api: ElementApi) {
    let _ = api.swap(DOWN)
        || api.swap(DOWN + LEFT)
        || api.swap(DOWN + RIGHT)
        || api.swap(LEFT)
        || api.swap(RIGHT);

    api.flip_visited();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::tests::{new_marked, new_non_moving};
    use crate::simulation::ElementMatrix;
    use crate::vector::Vector;

    #[test]
    fn test_tick_fall_bottom() {
        let mut matrix = ElementMatrix::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

        assert!(matrix.matrix[4].is_none());
        assert_eq!(
            matrix.matrix[7].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_fall_either_side() {
        let mut matrix = ElementMatrix::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[7].is_some());
        let left_or_right = matrix.matrix[6]
            .clone()
            .unwrap_or_else(|| matrix.matrix[8].clone().unwrap());
        assert_eq!(left_or_right.properties.name(), "test object");
    }

    #[test]
    fn test_tick_fall_left() {
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

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
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[6] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

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
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

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
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());
        matrix.matrix[5] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

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
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());
        matrix.matrix[3] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));

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

    #[test]
    fn test_flip_visited() {
        let mut matrix = ElementMatrix::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));

        assert!(!matrix.matrix[4].clone().unwrap().properties.visited());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 1)));
        assert!(matrix.matrix[7].clone().unwrap().properties.visited());

        liquid_behaviour(ElementApi::new(&mut matrix, Vector::new(1, 2)));
        let left_or_right = matrix.matrix[6]
            .clone()
            .unwrap_or_else(|| matrix.matrix[8].clone().unwrap());
        assert!(!left_or_right.properties.visited());
    }
}
