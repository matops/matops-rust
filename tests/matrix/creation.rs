use super::*;
use matops::Matrix;

#[test]
fn test_identity() {
    let i: Matrix<f64, 3, 3> = Matrix::identity();
    let expected: Matrix<f64, 3, 3> =
        Matrix::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert_matrix_approx_eq(i, expected, 1e-9);
}

#[test]
fn test_zeros() {
    // Implicitly tested in test_multiplication_by_zero,
    // but an explicit test might be good.
    let zero2x3: Matrix<i32, 2, 3> = Matrix::zeros();
    let expected: Matrix<i32, 2, 3> = Matrix::new([[0, 0, 0], [0, 0, 0]]);
    assert_eq!(zero2x3, expected);

    let zero1x1: Matrix<f64, 1, 1> = Matrix::zeros();
    let expected_f64: Matrix<f64, 1, 1> = Matrix::new([[0.0]]);
    assert_matrix_approx_eq(zero1x1, expected_f64, 1e-9);
}
