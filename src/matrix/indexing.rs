use super::core::Matrix;
use core::ops::{Index, IndexMut};

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < M, "Matrix row index out of bounds");
        &self.data[row]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < M, "Matrix row index out of bounds");
        &mut self.data[row]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let m = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(m[0][0], 1);
        assert_eq!(m[1][1], 4);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix::new([[1, 2], [3, 4]]);
        m[0][0] = 10;
        assert_eq!(m[0][0], 10);
    }
}
