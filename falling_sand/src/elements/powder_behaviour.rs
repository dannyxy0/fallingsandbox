use crate::element_api::ElementApi;
use crate::vector::{Vector, DOWN};

#[allow(clippy::short_circuit_statement)]
pub fn powder_behaviour(mut api: ElementApi) {
    let dx = api.rand_dir() as isize;
    let _ = api.swap(DOWN) || api.swap(Vector::new(dx, 1)) || api.swap(Vector::new(-dx, 1));

    api.flip_visited();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::tests::{new_marked, new_non_moving};
    use crate::simulation::ElementMatrix;
    use crate::vector::Vector;
    use rand_core::SeedableRng;
    use rand_xoshiro::SplitMix64;

    #[test]
    fn test_tick_fall_bottom() {
        let mut matrix = ElementMatrix::new(3, 3, None);
        matrix.matrix[4] = Some(new_marked("test object"));

        powder_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::seed_from_u64(16),
            Vector::new(1, 1),
        ));

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

        powder_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::seed_from_u64(16),
            Vector::new(1, 1),
        ));

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

        powder_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::seed_from_u64(16),
            Vector::new(1, 1),
        ));

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

        powder_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::seed_from_u64(16),
            Vector::new(1, 1),
        ));

        assert!(matrix.matrix[4].is_none());
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert_eq!(
            matrix.matrix[8].clone().unwrap().properties.name(),
            "test object"
        );
    }

    #[test]
    fn test_tick_blocked() {
        let mut matrix = ElementMatrix::new(3, 3, None);

        matrix.matrix[4] = Some(new_marked("test object"));
        matrix.matrix[6] = Some(new_non_moving());
        matrix.matrix[7] = Some(new_non_moving());
        matrix.matrix[8] = Some(new_non_moving());

        powder_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::seed_from_u64(16),
            Vector::new(1, 1),
        ));

        assert_eq!(
            matrix.matrix[4].clone().unwrap().properties.name(),
            "test object"
        );
        assert!(matrix.matrix[6].is_some());
        assert!(matrix.matrix[7].is_some());
        assert!(matrix.matrix[8].is_some());
    }
}
