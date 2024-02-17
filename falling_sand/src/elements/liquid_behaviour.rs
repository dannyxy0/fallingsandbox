use crate::element_api::ElementApi;
use crate::vector::{DOWN, LEFT};

#[rustfmt::skip]
#[allow(clippy::short_circuit_statement)]
pub fn liquid_behaviour(mut api: ElementApi) {
    api.flip_visited();

    let dx = api.rand_dir() as isize;
    let _ = api.swap(DOWN)
        || api.swap(DOWN + LEFT * dx)
        || api.swap(DOWN + LEFT * -dx)
         || api.swap(LEFT * dx)
         || api.swap(LEFT * -dx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::tests::{first_name, name_from_cell, new_marked, new_non_moving};
    use crate::simulation::ElementMatrix;
    use crate::vector::Vector;
    use rand_core::SeedableRng;
    use rand_xoshiro::SplitMix64;

    #[test]
    fn fall_bottom() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert_eq!(name_from_cell(&matrix.matrix[17]), "liquid");
    }

    #[test]
    fn fall_either_side() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[17] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[17].is_some());
        let left_or_right = first_name(vec![&matrix.matrix[16], &matrix.matrix[18]]);
        assert_eq!(left_or_right, "liquid");
    }

    #[test]
    fn fall_left() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[17] = Some(new_non_moving());
        matrix.matrix[18] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[17].is_some());
        assert!(matrix.matrix[18].is_some());
        assert_eq!(name_from_cell(&matrix.matrix[16]), "liquid");
    }

    #[test]
    fn fall_right() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[16] = Some(new_non_moving());
        matrix.matrix[17] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[16].is_some());
        assert!(matrix.matrix[17].is_some());
        assert_eq!(name_from_cell(&matrix.matrix[18]), "liquid");
    }

    #[test]
    fn move_either_side() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[16] = Some(new_non_moving());
        matrix.matrix[17] = Some(new_non_moving());
        matrix.matrix[18] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[16].is_some());
        assert!(matrix.matrix[17].is_some());
        assert!(matrix.matrix[18].is_some());
        let left_or_right = first_name(vec![&matrix.matrix[11], &matrix.matrix[13]]);
        assert_eq!(left_or_right, "liquid");
    }

    #[test]
    fn move_left() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[13] = Some(new_non_moving());
        matrix.matrix[16] = Some(new_non_moving());
        matrix.matrix[17] = Some(new_non_moving());
        matrix.matrix[18] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[13].is_some());
        assert!(matrix.matrix[16].is_some());
        assert!(matrix.matrix[17].is_some());
        assert!(matrix.matrix[18].is_some());
        assert_eq!(name_from_cell(&matrix.matrix[11]), "liquid");
    }

    #[test]
    fn move_right() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("liquid"));
        matrix.matrix[11] = Some(new_non_moving());
        matrix.matrix[16] = Some(new_non_moving());
        matrix.matrix[17] = Some(new_non_moving());
        matrix.matrix[18] = Some(new_non_moving());

        liquid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert!(matrix.matrix[11].is_some());
        assert!(matrix.matrix[16].is_some());
        assert!(matrix.matrix[17].is_some());
        assert!(matrix.matrix[18].is_some());
        assert_eq!(name_from_cell(&matrix.matrix[13]), "liquid");
    }
}
