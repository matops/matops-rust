use super::core::Matrix;
use core::fmt::Debug;

impl<T: Debug, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Matrix").field("data", &self.data).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let m = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(format!("{:?}", m), "Matrix { data: [[1, 2], [3, 4]] }");
    }
}
