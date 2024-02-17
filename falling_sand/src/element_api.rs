use crate::elements::element::Element;
use crate::simulation::ElementMatrix;
use crate::vector::Vector;
use rand_core::RngCore;
use rand_xoshiro::SplitMix64;

/// A wrapper around an ElementMatrix which provides helper methods for element implementations
pub struct ElementApi<'a> {
    pub matrix: &'a mut ElementMatrix,
    pub rng: &'a mut SplitMix64,
    pub position: Vector,
}

impl<'a> ElementApi<'a> {
    /// # Arguments
    ///
    /// * `matrix` - The ElementMatrix
    /// * `rng` - The random number generator
    /// * `position` - The absolute position to the current element. Needs to be a valid position containing an element
    pub fn new(matrix: &'a mut ElementMatrix, rng: &'a mut SplitMix64, position: Vector) -> Self {
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
            .get_mut(self.position)
            .expect("ElementApi expects valid position")
            .as_mut()
            .expect("ElementApi expects position containing element")
    }

    /// Returns the element at `pos`
    ///
    /// # Arguments
    ///
    /// * `rel_pos` - Relative position to the element
    pub fn other_element(&mut self, rel_pos: Vector) -> Option<&mut Element> {
        self.matrix
            .get_mut(self.position + rel_pos)
            .map_or(None, |x| x.as_mut())
    }

    /// Swaps `self.position` with `other_pos` if possible.
    /// Returns true if the swap was successful
    ///
    /// # Arguments
    ///
    /// * `other_pos` - Relative position to the element to swap with
    pub fn swap(&mut self, other_pos: Vector) -> bool {
        let swap_priority = self.element().properties.swap_priority();
        if self
            .other_element(other_pos)
            .is_some_and(|other| other.properties.swap_priority() >= swap_priority)
        {
            return false;
        }

        let swapped = self
            .matrix
            .swap(self.position, self.position + other_pos)
            .is_ok();
        if swapped {
            self.position += other_pos;
        }
        swapped
    }

    /// Returns -1 or 1 using `self.rng`
    pub fn rand_dir(&mut self) -> i32 {
        (self.rng.next_u32() as i32 % 2).abs() * 2 - 1
    }
}
