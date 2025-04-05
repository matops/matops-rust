use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::vector::Vector;

#[derive(Clone, Copy, PartialEq)]
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

// Matrix Addition
impl<T, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

// Matrix Subtraction
impl<T, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        Matrix { data: result }
    }
}

// Scalar Multiplication (Matrix * Scalar)
impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] * scalar;
            }
        }
        Matrix { data: result }
    }
}

// Scalar Division (Matrix / Scalar)
impl<T, const M: usize, const N: usize> Div<T> for Matrix<T, M, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        let mut result = self.data;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.data[i][j] / scalar;
            }
        }
        Matrix { data: result }
    }
}

// Only implement determinant, inverse, identity for f64 square matrices
impl<const M: usize> Matrix<f64, M, M> {
    pub fn determinant(&self) -> f64 {
        let mut mat = *self;
        let mut det = 1.0;
        let mut sign = 1.0;

        for i in 0..M {
            // Find pivot row
            let mut pivot = i;
            for j in i..M {
                // Use a small tolerance for float comparison
                if (mat.data[j][i]).abs() > 1e-9 {
                    pivot = j;
                    break;
                }
            }

            // Check if pivot is effectively zero
            if (mat.data[pivot][i]).abs() <= 1e-9 {
                return 0.0; // Matrix is singular
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
    pub fn identity() -> Self {
        let mut data = [[0.0; M]; M];
        for i in 0..M {
            data[i][i] = 1.0;
        }
        Self { data }
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut mat = *self;
        let mut inv = Self::identity();

        for i in 0..M {
            // Find pivot row
            let mut pivot = i;
            for j in i..M {
                // Use a small tolerance for float comparison
                if (mat.data[j][i]).abs() > 1e-9 {
                    pivot = j;
                    break;
                }
            }
            // Check if pivot is effectively zero
            if (mat.data[pivot][i]).abs() <= 1e-9 {
                return None; // Matrix is singular
            }

            // Swap rows
            if pivot != i {
                mat.data.swap(i, pivot);
                inv.data.swap(i, pivot);
            }

            // Normalize pivot row
            let factor = 1.0 / mat.data[i][i];
            for k in 0..M {
                mat.data[i][k] = mat.data[i][k] * factor;
                inv.data[i][k] = inv.data[i][k] * factor;
            }

            // Eliminate other rows
            for j in 0..M {
                if j != i {
                    let factor = mat.data[j][i];
                    for k in 0..M {
                        mat.data[j][k] = mat.data[j][k] - factor * mat.data[i][k];
                        inv.data[j][k] = inv.data[j][k] - factor * inv.data[i][k];
                    }
                }
            }
        }

        Some(inv)
    }
}
