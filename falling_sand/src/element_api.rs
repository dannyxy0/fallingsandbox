use crate::elements::element::Element;
use crate::idx;
use crate::simulation::ElementMatrix;
use nalgebra::Vector2;
use rand_core::RngCore;
use rand_xoshiro::SplitMix64;

/// A wrapper around an ElementMatrix which provides helper methods for element implementations
pub struct ElementApi<'a> {
    pub matrix: &'a mut ElementMatrix,
    pub rng: &'a mut SplitMix64,
    pub position: Vector2<usize>,
}

impl<'a> ElementApi<'a> {
    /// # Arguments
    ///
    /// * `matrix` - The ElementMatrix
    /// * `rng` - The random number generator
    /// * `position` - The absolute position to the current element. Needs to be a valid position containing an element
    pub fn new(
        matrix: &'a mut ElementMatrix,
        rng: &'a mut SplitMix64,
        position: Vector2<usize>,
    ) -> Self {
        Self {
            matrix,
            rng,
            position,
        }
    }

    /// Flips the visited flag of the current element
    pub fn flip_visited(&mut self) {
        let element = self.element();
        element
            .properties
            .set_visited(!element.properties.visited());
    }

    /// Returns the current element
    pub fn element(&mut self) -> &mut Element {
        self.matrix
            .get_mut(idx!(self.position))
            .expect("ElementApi expects valid position")
            .as_mut()
            .expect("ElementApi expects position containing element")
    }

    /// Returns the element at `pos`
    ///
    /// # Arguments
    ///
    /// * `rel_pos` - Relative position to the element
    pub fn other_element(&mut self, rel_pos: Vector2<isize>) -> Option<&mut Element> {
        let pos = self.position.cast() + rel_pos;
        self.matrix
            .get_mut(idx!(pos.try_cast()?))
            .and_then(|x| x.as_mut())
    }

    /// Swaps `self.position` with `other_pos` if possible.
    /// Returns true if the swap was successful
    ///
    /// # Arguments
    ///
    /// * `other_pos` - Relative position to the element to swap with
    pub fn swap(&mut self, other_pos: Vector2<isize>) -> bool {
        let swap_priority = self.element().properties.swap_priority();
        if self
            .other_element(other_pos)
            .is_some_and(|other| other.properties.swap_priority() >= swap_priority)
            || !self.in_bounds(other_pos)
        {
            return false;
        }

        let abs_other_pos = (self.position.cast() + other_pos).try_cast();
        if let Some(pos) = abs_other_pos {
            self.matrix.swap(idx!(self.position), idx!(pos));
            self.position = pos;
            true
        } else {
            false
        }
    }

    /// Returns -1 or 1 using `self.rng`
    pub fn rand_dir(&mut self) -> i32 {
        (self.rng.next_u32() as i32 % 2).abs() * 2 - 1
    }

    /// Checks if the position is in bounds of `self.matrix`
    ///
    /// # Arguments
    ///
    /// * `rel_pos` - Relative position that is to be checked
    pub fn in_bounds(&self, rel_pos: Vector2<isize>) -> bool {
        let pos = self.position.cast() + rel_pos;
        pos.x >= 0
            && pos.x < self.matrix.ncols() as isize
            && pos.y >= 0
            && pos.y < self.matrix.nrows() as isize
    }
}
