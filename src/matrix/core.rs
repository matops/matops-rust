#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data }
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Creates a matrix filled with zeros (default values).
    pub fn zeros() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(m.data, [[1, 2], [3, 4]]);
    }

    #[test]
    fn test_zeros() {
        let m: Matrix<i32, 2, 2> = Matrix::zeros();
        assert_eq!(m.data, [[0, 0], [0, 0]]);
    }
}
