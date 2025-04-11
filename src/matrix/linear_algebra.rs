use super::core::Matrix;

impl<T: Copy + Default, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut transposed_data = [[T::default(); M]; N];
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

impl<const M: usize> Matrix<f64, M, M> {
    pub fn determinant(&self) -> f64 {
        let mut mat = *self;
        let mut det = 1.0;
        let mut sign = 1.0;

        for i in 0..M {
            let mut pivot = i;
            for j in i..M {
                if mat[j][i].abs() > 1e-9 {
                    pivot = j;
                    break;
                }
            }
            if mat[pivot][i].abs() <= 1e-9 {
                return 0.0;
            }
            if pivot != i {
                mat.data.swap(i, pivot);
                sign = -sign;
            }
            let pivot_val = mat[i][i];
            for j in (i + 1)..M {
                let factor = mat[j][i] / pivot_val;
                for k in i..M {
                    let val_ik = mat[i][k];
                    mat[j][k] -= factor * val_ik;
                }
            }
            det *= pivot_val;
        }
        det * sign
    }

    pub fn identity() -> Self {
        let mut result = Self::zeros();
        for i in 0..M {
            result[i][i] = 1.0;
        }
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut mat = *self;
        let mut inv = Self::identity();
        for i in 0..M {
            let mut pivot = i;
            for j in i..M {
                if mat[j][i].abs() > 1e-9 {
                    pivot = j;
                    break;
                }
            }
            if mat[pivot][i].abs() <= 1e-9 {
                return None;
            }
            if pivot != i {
                mat.data.swap(i, pivot);
                inv.data.swap(i, pivot);
            }
            let pivot_val = mat[i][i];
            let factor = 1.0 / pivot_val;
            for k in 0..M {
                mat[i][k] *= factor;
                inv[i][k] *= factor;
            }
            for j in 0..M {
                if j != i {
                    let factor = mat[j][i];
                    for k in 0..M {
                        let val_ik_mat = mat[i][k];
                        let val_ik_inv = inv[i][k];
                        mat[j][k] -= factor * val_ik_mat;
                        inv[j][k] -= factor * val_ik_inv;
                    }
                }
            }
        }
        Some(inv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let t = m.transpose();
        assert_eq!(t.data, [[1.0, 3.0], [2.0, 4.0]]);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        assert!((m.determinant() - -2.0).abs() < 1e-9);
    }
}
