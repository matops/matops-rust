// tests/matrix/basic_arithmetic.rs
use super::common::*;

#[test]
fn test_addition() {
    let a: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
    let b: Matrix<i32, 2, 2> = Matrix::new([[5, 6], [7, 8]]);
    let expected: Matrix<i32, 2, 2> = Matrix::new([[6, 8], [10, 12]]);
    assert_eq!(a + b, expected);
}

#[test]
fn test_subtraction() {
    let a: Matrix<i32, 2, 2> = Matrix::new([[5, 8], [10, 12]]);
    let b: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
    let expected: Matrix<i32, 2, 2> = Matrix::new([[4, 6], [7, 8]]);
    assert_eq!(a - b, expected);
}

#[test]
fn test_scalar_multiplication() {
    let a: Matrix<i32, 2, 2> = Matrix::new([[1, 2], [3, 4]]);
    let scalar = 3;
    let expected: Matrix<i32, 2, 2> = Matrix::new([[3, 6], [9, 12]]);
    assert_eq!(a * scalar, expected);
}

#[test]
fn test_scalar_division() {
    let a: Matrix<f64, 2, 2> = Matrix::new([[3.0, 6.0], [9.0, 12.0]]);
    let scalar = 3.0;
    let expected: Matrix<f64, 2, 2> = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    assert_matrix_approx_eq(a / scalar, expected, 1e-9);
}

#[test]
fn test_negation() {
    let a: Matrix<i32, 2, 2> = Matrix::new([[1, -2], [-3, 4]]);
    let expected: Matrix<i32, 2, 2> = Matrix::new([[-1, 2], [3, -4]]);
    assert_eq!(-a, expected);

    // Test with f64
    let b: Matrix<f64, 1, 2> = Matrix::new([[5.5, -6.6]]);
    let expected_b: Matrix<f64, 1, 2> = Matrix::new([[-5.5, 6.6]]);
    assert_matrix_approx_eq(-b, expected_b, 1e-9);

    // Test negation of zero matrix
    let zero: Matrix<i32, 2, 2> = Matrix::zeros();
    assert_eq!(-zero, zero);
}
