use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Mul};

pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}

impl<T, const M: usize, const K: usize, const N: usize> Mul<Matrix<T, K, N>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + Sum,
{
    type Output = Matrix<T, M, N>;
    fn mul(self, rhs: Matrix<T, K, N>) -> Self::Output {
        let mut result = [[T::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                result[i][j] = (0..K).map(|k| self.data[i][k] * rhs.data[k][j]).sum();
            }
        }
        Matrix { data: result }
    }
}

impl<T: Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut data = [[self.data[0][0]; M]; N];
        for i in 0..M {
            for j in 0..N {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix { data }
    }
}

impl<T: Debug, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("data", &self.data).finish()
    }
}
