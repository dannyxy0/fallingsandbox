pub struct Matrix<T> {
    matrix: Vec<T>,
    width: usize,
    height: usize
}

impl<'a, T> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        Matrix {
            matrix: Vec::with_capacity(width * height),
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
}
