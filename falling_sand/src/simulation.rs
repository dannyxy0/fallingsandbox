use crate::cell::Cell;
use crate::elements::element::Element;
use crate::matrix::Matrix;
use crate::position::Position;

pub struct Simulation<'a> {
    pub matrix: Matrix<Cell<'a>>
}

impl Simulation<'_> {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            matrix: Matrix::new(width, height, &Cell::default())
        }
    }

    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.matrix.width() as i32 &&
        pos.y >= 0 && pos.y < self.matrix.height() as i32
    }

    pub fn get(&self, pos: Position) -> Option<&dyn Element> {
        if !self.in_bounds(pos) {
            return None;
        }

        match self.matrix.get(pos.x as usize, pos.y as usize) {
            Cell::Empty => None,
            Cell::Element(element) => Some(*element)
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.matrix.width() {
            for j in 0..self.matrix.height() {
                let cell = self.matrix.get(i, j);
                match cell {
                    Cell::Empty => (),
                    Cell::Element(element) => {
                        let _ = element.tick(Position::new_usize(i, j), self);
                    }
                }
            }
        }
    }

    pub fn move_and_swap(&mut self, _pos1: Position, _pos2: Position) {
        !todo!()
    }
}
