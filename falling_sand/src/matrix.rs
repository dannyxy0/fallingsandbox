use crate::vector::Vector;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Matrix<T> {
    pub matrix: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        Matrix {
            matrix: vec![default_value; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Vector) -> Result<&T> {
        Ok(&self.matrix[self.index(pos)?])
    }

    pub fn get_mut(&mut self, pos: Vector) -> Result<&mut T> {
        let index = self.index(pos)?;
        Ok(&mut self.matrix[index])
    }

    pub fn set(&mut self, pos: Vector, value: T) -> Result<()> {
        let index = self.index(pos)?;
        self.matrix[index] = value;
        Ok(())
    }

    pub fn in_bounds(&self, pos: Vector) -> bool {
        pos.x >= 0 && pos.x < self.width as isize && pos.y >= 0 && pos.y < self.height as isize
    }

    pub fn swap(&mut self, pos1: Vector, pos2: Vector) -> Result<()> {
        let index1 = self.index(pos1)?;
        let index2 = self.index(pos2)?;
        self.matrix.swap(index1, index2);
        Ok(())
    }

    pub fn fill(&mut self, pos: Vector, size: Vector, value: T) -> Result<()>
    where
        T: Clone,
    {
        self.index(pos + size + Vector::new(-1, -1))?;

        for i in 0..size.x {
            for j in 0..size.y {
                let index = self.index(pos + Vector::new(i, j))?;
                self.matrix[index] = value.clone();
            }
        }

        Ok(())
    }

    fn index(&self, pos: Vector) -> Result<usize> {
        if !self.in_bounds(pos) {
            return Err(anyhow!("pos '{pos}' is out of bounds"));
        }
        Ok(pos.x as usize + pos.y as usize * self.width) // Casting to usize is safe because pos is in bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        let matrix1 = Matrix::new(3, 5, 0);
        assert_eq!(matrix1.width(), 3);
        let matrix2 = Matrix::new(8, 20, 0);
        assert_eq!(matrix2.width(), 8);
    }

    #[test]
    fn test_height() {
        let matrix1 = Matrix::new(3, 5, 0);
        assert_eq!(matrix1.height(), 5);
        let matrix2 = Matrix::new(8, 20, 0);
        assert_eq!(matrix2.height(), 20);
    }

    #[test]
    fn test_in_bounds_valid() {
        let matrix = Matrix::new(3, 5, 0);
        assert!(matrix.in_bounds(Vector::new(0, 0)));
        assert!(matrix.in_bounds(Vector::new(2, 4)));
        assert!(matrix.in_bounds(Vector::new(1, 2)));
    }

    #[test]
    fn test_in_bounds_invalid() {
        let matrix = Matrix::new(3, 5, 0);
        assert!(!matrix.in_bounds(Vector::new(-1, 0)));
        assert!(!matrix.in_bounds(Vector::new(0, -1)));
        assert!(!matrix.in_bounds(Vector::new(3, 0)));
        assert!(!matrix.in_bounds(Vector::new(0, 5)));
        assert!(!matrix.in_bounds(Vector::new(-5, 10)));
    }

    #[test]
    fn test_get_valid() -> Result<()> {
        let matrix = Matrix::new(3, 5, 1);
        assert_eq!(matrix.get(Vector::new(0, 0))?, &1);
        assert_eq!(matrix.get(Vector::new(2, 4))?, &1);
        assert_eq!(matrix.get(Vector::new(1, 2))?, &1);

        Ok(())
    }

    #[test]
    fn test_get_invalid() {
        let matrix = Matrix::new(3, 5, 1);
        assert!(matrix.get(Vector::new(-1, 0)).is_err());
        assert!(matrix.get(Vector::new(0, -1)).is_err());
        assert!(matrix.get(Vector::new(3, 0)).is_err());
        assert!(matrix.get(Vector::new(0, 5)).is_err());
        assert!(matrix.get(Vector::new(-5, 10)).is_err());
    }

    #[test]
    fn test_get_mut_valid() -> Result<()> {
        let mut matrix = Matrix::new(3, 5, 1);
        assert_eq!(matrix.get_mut(Vector::new(0, 0))?, &mut 1);
        assert_eq!(matrix.get_mut(Vector::new(2, 4))?, &mut 1);
        assert_eq!(matrix.get_mut(Vector::new(1, 2))?, &mut 1);

        Ok(())
    }

    #[test]
    fn test_get_mut_invalid() {
        let mut matrix = Matrix::new(3, 5, 1);
        assert!(matrix.get_mut(Vector::new(-1, 0)).is_err());
        assert!(matrix.get_mut(Vector::new(0, -1)).is_err());
        assert!(matrix.get_mut(Vector::new(3, 0)).is_err());
        assert!(matrix.get_mut(Vector::new(0, 5)).is_err());
        assert!(matrix.get_mut(Vector::new(-5, 10)).is_err());
    }

    #[test]
    fn test_set_valid() {
        let mut matrix = Matrix::new(8, 10, 1);

        assert!(matrix.set(Vector::new(0, 0), 3).is_ok());
        assert_eq!(matrix.matrix[0], 3);

        assert!(matrix.set(Vector::new(7, 9), 4).is_ok());
        assert_eq!(matrix.matrix[79], 4);

        assert!(matrix.set(Vector::new(3, 5), 5).is_ok());
        assert_eq!(matrix.matrix[43], 5);
    }

    #[test]
    fn test_set_invalid() {
        let mut matrix = Matrix::new(8, 10, 1);
        let original = matrix.clone();

        assert!(matrix.set(Vector::new(-1, 0), 3).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.set(Vector::new(0, -1), 4).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.set(Vector::new(8, 0), 5).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.set(Vector::new(0, 10), 6).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.set(Vector::new(-5, 12), 7).is_err());
        assert_eq!(matrix.matrix, original.matrix);
    }

    #[test]
    fn test_swap_valid() {
        let mut matrix = Matrix::new(4, 2, 0);
        matrix.matrix[0] = 5;
        matrix.matrix[6] = 7;

        assert!(matrix.swap(Vector::new(0, 0), Vector::new(2, 1)).is_ok());
        assert_eq!(matrix.matrix[0], 7);
        assert_eq!(matrix.matrix[6], 5);

        assert!(matrix.swap(Vector::new(3, 1), Vector::new(0, 0)).is_ok());
        assert_eq!(matrix.matrix[0], 0);
        assert_eq!(matrix.matrix[7], 7);
    }

    #[test]
    fn test_swap_invalid() {
        let mut matrix = Matrix::new(4, 2, 0);
        let original = matrix.clone();

        assert!(matrix.swap(Vector::new(-1, 0), Vector::new(0, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, 0), Vector::new(-1, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, -1), Vector::new(0, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, 0), Vector::new(0, -1)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(8, 0), Vector::new(0, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, 0), Vector::new(8, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, 10), Vector::new(0, 0)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(0, 0), Vector::new(0, 10)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(-5, 12), Vector::new(1, 3)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix.swap(Vector::new(1, 3), Vector::new(-5, 12)).is_err());
        assert_eq!(matrix.matrix, original.matrix);

        assert!(matrix
            .swap(Vector::new(-8, 15), Vector::new(-20, 2))
            .is_err());
        assert_eq!(matrix.matrix, original.matrix);
    }

    #[test]
    fn test_fill_valid() {
        let mut matrix = Matrix::new(10, 10, 0);

        assert!(matrix.fill(Vector::new(5, 5), Vector::new(2, 2), 1).is_ok());
        for i in 5..7 {
            for j in 5..7 {
                assert_eq!(matrix.matrix[i + j * 10], 1);
            }
        }
        assert_eq!(matrix.matrix[54], 0);
        assert_eq!(matrix.matrix[78], 0);

        assert!(matrix.fill(Vector::new(0, 0), Vector::new(6, 6), 2).is_ok());
        for i in 0..6 {
            for j in 0..6 {
                assert_eq!(matrix.matrix[i + j * 10], 2);
            }
        }
        assert_eq!(matrix.matrix[66], 1);
        assert_eq!(matrix.matrix[78], 0);

        assert!(matrix
            .fill(Vector::new(0, 0), Vector::new(10, 10), 5)
            .is_ok());
        matrix.matrix.iter().for_each(|x| assert_eq!(*x, 5))
    }
}
