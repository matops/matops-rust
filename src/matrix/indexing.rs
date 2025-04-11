use super::core::Matrix;
use core::ops::{Index, IndexMut};

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < M && index.1 < N, "Matrix index out of bounds");
        &self.data[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < M && index.1 < N, "Matrix index out of bounds");
        &mut self.data[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let m = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(1, 1)], 4);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix::new([[1, 2], [3, 4]]);
        m[(0, 0)] = 10;
        assert_eq!(m[(0, 0)], 10);
    }
}
