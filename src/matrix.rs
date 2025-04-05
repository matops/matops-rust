use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::vector::Vector;

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub data: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Creates a new matrix from the given 2D data array.
    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data }
    }
}

impl<T: Debug, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("data", &self.data).finish()
    }
}

// Add a zeros constructor
impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn zeros() -> Self {
        Self {
            data: [[T::default(); N]; M],
        }
    }
}

// Matrix multiplication
impl<T, const M: usize, const K: usize, const N: usize> Mul<Matrix<T, K, N>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + Sum,
{
    type Output = Matrix<T, M, N>;
    fn mul(self, rhs: Matrix<T, K, N>) -> Self::Output {
        let mut result = Self::Output::zeros(); // Use zeros constructor
        for i in 0..M {
            for j in 0..N {
                // Use indexers and map/sum for dot product
                result[(i, j)] = (0..K).map(|k| self[(i, k)] * rhs[(k, j)]).sum();
            }
        }
        result
    }
}

// Matrix transpose
impl<T: Copy + Default, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut transposed_data = [[T::default(); M]; N]; // Initialize with default
        for i in 0..M {
            for j in 0..N {
                transposed_data[j][i] = self.data[i][j];
            }
        }
        Matrix {
            data: transposed_data,
        }
    }
}

// Matrix-Vector multiplication
// Assumes Vector<T, K> has a public `data: [T; K]` field and T is Copy
impl<T, const M: usize, const K: usize> Mul<Vector<T, K>> for Matrix<T, M, K>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy + Sum,
{
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, K>) -> Self::Output {
        let mut result_data = [T::default(); M];
        for i in 0..M {
            // Calculate dot product directly using indexers for matrix
            // and direct data access for vector
            result_data[i] = (0..K).map(|k| self[(i, k)] * rhs.data[k]).sum();
        }
        Vector { data: result_data }
    }
}

// Matrix Addition
impl<T, const M: usize, const N: usize> Add<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Self;
    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Self::zeros(); // Use the new constructor
        for i in 0..M {
            for j in 0..N {
                // Use the new indexers
                result[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

// Matrix Subtraction
impl<T, const M: usize, const N: usize> Sub<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Self;
    fn sub(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Self::zeros(); // Use the new constructor
        for i in 0..M {
            for j in 0..N {
                // Use the new indexers
                result[(i, j)] = self[(i, j)] - rhs[(i, j)];
            }
        }
        result
    }
}

// Scalar Multiplication (Matrix * Scalar)
impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N>
where
    T: Mul<Output = T> + Copy + Default,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        let mut result = Self::zeros(); // Use the new constructor
        for i in 0..M {
            for j in 0..N {
                // Use the new indexers
                result[(i, j)] = self[(i, j)] * scalar;
            }
        }
        result
    }
}

// Scalar Division (Matrix / Scalar)
impl<T, const M: usize, const N: usize> Div<T> for Matrix<T, M, N>
where
    T: Div<Output = T> + Copy + Default,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        let mut result = Self::zeros(); // Use the new constructor
        for i in 0..M {
            for j in 0..N {
                // Use the new indexers
                result[(i, j)] = self[(i, j)] / scalar;
            }
        }
        result
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
                if mat[(j, i)].abs() > 1e-9 {
                    // Use indexer
                    pivot = j;
                    break;
                }
            }

            // Check if pivot is effectively zero
            if mat[(pivot, i)].abs() <= 1e-9 {
                // Use indexer
                return 0.0; // Matrix is singular
            }

            // Swap rows if needed
            if pivot != i {
                mat.data.swap(i, pivot); // Keep swapping rows directly on data
                sign = -sign;
            }

            // Eliminate lower rows
            let pivot_val = mat[(i, i)]; // Use indexer
            for j in (i + 1)..M {
                let factor = mat[(j, i)] / pivot_val; // Use indexer
                for k in i..M {
                    let val_ik = mat[(i, k)]; // Use indexer
                    mat[(j, k)] -= factor * val_ik; // Use indexer
                }
            }

            det *= pivot_val; // Use the stored pivot value
        }

        det * sign
    }

    pub fn identity() -> Self {
        let mut result = Self::zeros(); // Use zeros constructor
        for i in 0..M {
            result[(i, i)] = 1.0; // Use indexer
        }
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut mat = *self;
        let mut inv = Self::identity();

        for i in 0..M {
            // Find pivot row
            let mut pivot = i;
            for j in i..M {
                if mat[(j, i)].abs() > 1e-9 {
                    // Use indexer
                    pivot = j;
                    break;
                }
            }
            // Check if pivot is effectively zero
            if mat[(pivot, i)].abs() <= 1e-9 {
                // Use indexer
                return None; // Matrix is singular
            }

            // Swap rows
            if pivot != i {
                mat.data.swap(i, pivot); // Keep swapping rows directly
                inv.data.swap(i, pivot); // Keep swapping rows directly
            }

            // Normalize pivot row
            let pivot_val = mat[(i, i)]; // Use indexer
            let factor = 1.0 / pivot_val;
            for k in 0..M {
                mat[(i, k)] *= factor; // Use indexer
                inv[(i, k)] *= factor; // Use indexer
            }

            // Eliminate other rows
            for j in 0..M {
                if j != i {
                    let factor = mat[(j, i)]; // Use indexer
                    for k in 0..M {
                        let val_ik_mat = mat[(i, k)]; // Use indexer
                        let val_ik_inv = inv[(i, k)]; // Use indexer
                        mat[(j, k)] -= factor * val_ik_mat; // Use indexer
                        inv[(j, k)] -= factor * val_ik_inv; // Use indexer
                    }
                }
            }
        }

        Some(inv)
    }
}

// Indexing (Read-only) - Use (row, col) tuple for indexing
impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < M && index.1 < N, "Matrix index out of bounds");
        &self.data[index.0][index.1]
    }
}

// Indexing (Mutable) - Use (row, col) tuple for indexing
impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < M && index.1 < N, "Matrix index out of bounds");
        &mut self.data[index.0][index.1]
    }
}

// Matrix Negation
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
