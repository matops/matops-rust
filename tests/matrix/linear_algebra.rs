use super::*;
use matops::Matrix;

#[test]
fn test_determinant_2x2() {
    let a: Matrix<f64, 2, 2> = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
    // det = 1*4 - 2*3 = 4 - 6 = -2
    assert!((a.determinant() - (-2.0)).abs() < 1e-9);
}

#[test]
fn test_determinant_3x3() {
    let a: Matrix<f64, 3, 3> = Matrix::new([[6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]]);
    // det = 6*(-2*7 - 5*8) - 1*(4*7 - 5*2) + 1*(4*8 - (-2)*2)
    // det = 6*(-14 - 40) - 1*(28 - 10) + 1*(32 + 4)
    // det = 6*(-54) - 1*(18) + 1*(36)
    // det = -324 - 18 + 36 = -306
    assert!((a.determinant() - (-306.0)).abs() < 1e-9);
}

#[test]
fn test_determinant_singular() {
    let a: Matrix<f64, 2, 2> = Matrix::new([[1.0, 2.0], [2.0, 4.0]]); // Row 2 is 2 * Row 1
    assert!(a.determinant().abs() < 1e-9); // Determinant should be close to 0

    let b: Matrix<f64, 3, 3> = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]); // Row 3 = 2*Row2 - Row1
    assert!(b.determinant().abs() < 1e-9); // Determinant should be close to 0
}

#[test]
fn test_inverse_2x2() {
    let a: Matrix<f64, 2, 2> = Matrix::new([[4.0, 7.0], [2.0, 6.0]]);
    // det = 4*6 - 7*2 = 24 - 14 = 10
    // inv = 1/10 * [[6, -7], [-2, 4]] = [[0.6, -0.7], [-0.2, 0.4]]
    let a_inv = a.inverse().unwrap();
    let expected_inv: Matrix<f64, 2, 2> = Matrix::new([[0.6, -0.7], [-0.2, 0.4]]);
    let identity: Matrix<f64, 2, 2> = Matrix::identity();

    assert_matrix_approx_eq(a_inv, expected_inv, 1e-9);
    assert_matrix_approx_eq(a * a_inv, identity, 1e-9);
    assert_matrix_approx_eq(a_inv * a, identity, 1e-9);
}

#[test]
fn test_inverse_3x3() {
    let a: Matrix<f64, 3, 3> = Matrix::new([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);
    // det = 1*(1*0 - 4*6) - 2*(0*0 - 4*5) + 3*(0*6 - 1*5)
    // det = 1*(-24) - 2*(-20) + 3*(-5)
    // det = -24 + 40 - 15 = 1
    // Using an online calculator for the inverse:
    let expected_inv: Matrix<f64, 3, 3> =
        Matrix::new([[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]]);
    let identity: Matrix<f64, 3, 3> = Matrix::identity();
    let a_inv = a.inverse().unwrap();

    assert_matrix_approx_eq(a_inv, expected_inv, 1e-9);
    assert_matrix_approx_eq(a * a_inv, identity, 1e-9);
    assert_matrix_approx_eq(a_inv * a, identity, 1e-9);
}

#[test]
fn test_inverse_non_invertible() {
    let a: Matrix<f64, 2, 2> = Matrix::new([[1.0, 2.0], [2.0, 4.0]]); // Determinant is 0
    assert!(a.inverse().is_none());
}

#[test]
fn test_transpose_transpose() {
    let a: Matrix<i32, 2, 3> = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let a_t = a.transpose();
    let a_tt = a_t.transpose();
    assert_eq!(a, a_tt);
}

// Note: test_transpose_of_product is in multiplication.rs
