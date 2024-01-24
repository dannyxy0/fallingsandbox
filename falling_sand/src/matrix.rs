pub struct Matrix<T> {
    matrix: Vec<T>,
    width: usize,
    height: usize
}

impl<'a, T> Matrix<T> where T: Clone + Default
{
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        Matrix {
            matrix: vec![T::default(); width * height],
            width,
            height
        }
    }

    pub fn get(&'a self, x: usize, y: usize) -> &'a T {
        &self.matrix[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.matrix[x + y * self.width] = value;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, x: usize, y: usize, width: usize, height: usize, value: &T) {
        for i in 0..width {
            for j in 0..height {
                self.set(i + x, j + y, value.clone());
            }
        }
    }
}
