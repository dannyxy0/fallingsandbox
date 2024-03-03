use crate::element_api::ElementApi;
use crate::vector::DOWN;

pub fn solid_behaviour(mut api: ElementApi) {
    api.flip_visited();
    let _ = api.swap(DOWN);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::tests::{name_from_cell, new_marked, new_non_moving};
    use crate::simulation::ElementMatrix;
    use crate::vector::Vector;
    use rand_core::SeedableRng;
    use rand_xoshiro::SplitMix64;

    #[test]
    fn fall_bottom() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("solid"));

        solid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[12].is_none());
        assert_eq!(name_from_cell(&matrix.matrix[17]), "solid");
    }

    #[test]
    fn fall_blocked() {
        let mut matrix = ElementMatrix::new(5, 5, None);
        matrix.matrix[12] = Some(new_marked("solid"));
        matrix.matrix[17] = Some(new_non_moving());

        solid_behaviour(ElementApi::new(
            &mut matrix,
            &mut SplitMix64::from_entropy(),
            Vector::new(2, 2),
        ));

        assert!(matrix.matrix[17].is_some());
        assert_eq!(name_from_cell(&matrix.matrix[12]), "solid");
    }
}
