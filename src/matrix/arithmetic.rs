use super::core::Matrix;
use crate::vector::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<T, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Self;
    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Self::zeros();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Self;
    fn sub(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Self::zeros();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] - rhs[(i, j)];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
where
    T: Mul<Output = T> + Copy + Default,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        let mut result = Self::zeros();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] * scalar;
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Div<T> for Matrix<T, M, N>
where
    T: Div<Output = T> + Copy + Default,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        let mut result = Self::zeros();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] / scalar;
            }
        }
        result
    }
}

impl<T, const M: usize, const K: usize, const N: usize> Mul<Matrix<T, K, N>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + std::iter::Sum,
{
    type Output = Matrix<T, M, N>;
    fn mul(self, rhs: Matrix<T, K, N>) -> Self::Output {
        let mut result = Self::Output::zeros();
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = (0..K).map(|k| self[(i, k)] * rhs[(k, j)]).sum();
            }
        }
        result
    }
}

impl<T, const M: usize, const K: usize> Mul<Vector<T, K>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + std::iter::Sum,
{
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, K>) -> Self::Output {
        let mut result_data = [T::default(); M];
        for i in 0..M {
            result_data[i] = (0..K).map(|k| self[(i, k)] * rhs.data[k]).sum();
        }
        Vector { data: result_data }
    }
}

impl<T, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = -self.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);
        let result = a + b;
        assert_eq!(result.data, [[6.0, 8.0], [10.0, 12.0]]);
    }

    #[test]
    fn test_scalar_mul() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let result = a * 2.0;
        assert_eq!(result.data, [[2.0, 4.0], [6.0, 8.0]]);
    }
}
