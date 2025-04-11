use super::*;
use matops::{Matrix, Vector};

#[test]
fn test_1x1_matrix() {
    let a: Matrix<i32, 1, 1> = Matrix::new([[5]]);
    let b: Matrix<i32, 1, 1> = Matrix::new([[3]]);
    let v: Vector<i32, 1> = Vector::new([2]);

    // Multiplication (i32)
    assert_eq!(a * b, Matrix::new([[15]]));

    // Transpose (i32)
    assert_eq!(a.transpose(), a);

    // Matrix-Vector Multiplication (i32)
    assert_eq!(a * v, Vector::new([10]));

    // Determinant (f64)
    let a_f64: Matrix<f64, 1, 1> = Matrix::new([[5.0]]);
    assert!((a_f64.determinant() - 5.0).abs() < 1e-9);

    // Identity (f64)
    assert_matrix_approx_eq(Matrix::<f64, 1, 1>::identity(), Matrix::new([[1.0]]), 1e-9);

    // Inverse (f64)
    let inv_a = a_f64.inverse().unwrap();
    let expected_inv: Matrix<f64, 1, 1> = Matrix::new([[1.0 / 5.0]]);
    assert_matrix_approx_eq(inv_a, expected_inv, 1e-9);
    assert_matrix_approx_eq(a_f64 * inv_a, Matrix::identity(), 1e-9);
}

// Add more tests for edge cases if necessary, e.g., different types, dimensions
