use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}

impl<T: Debug, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("data", &self.data).finish()
    }
}
// Matrix multiplication
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

// Matrix transpose
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

// Matrix-Vector multiplication
impl<T, const M: usize, const K: usize> Mul<Vector<T, K>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, K>) -> Self::Output {
        let mut data = [T::default(); M];
        for (i, row) in self.data.iter().enumerate() {
            let row_vector = Vector { data: *row };
            data[i] = row_vector.dot(&rhs);
        }
        Vector { data }
    }
}

impl<T, const M: usize> Matrix<T, M, M>
where
    T: Copy
        + Default
        + PartialEq
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + From<i32>
        + Div<Output = T>,
{
    pub fn determinant(&self) -> T {
        let mut mat = *self;
        let mut det = T::from(1);
        let mut sign = T::from(1);

        for i in 0..M {
            // Find pivot row
            let mut pivot = i;
            for j in i..M {
                if mat.data[j][i] != T::default() {
                    pivot = j;
                    break;
                }
            }

            if mat.data[pivot][i] == T::default() {
                return T::default();
            }

            // Swap rows if needed
            if pivot != i {
                mat.data.swap(i, pivot);
                sign = -sign;
            }

            // Eliminate lower rows
            for j in (i + 1)..M {
                let factor = mat.data[j][i] / mat.data[i][i];
                for k in i..M {
                    mat.data[j][k] = mat.data[j][k] - factor * mat.data[i][k];
                }
            }

            det = det * mat.data[i][i];
        }

        det * sign
    }
}
