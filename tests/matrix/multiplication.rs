use super::*;
use matops::{Matrix, Vector};

#[test]
fn test_matrix_associativity() {
    let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let b = Matrix::new([[7, 8, 9, 10], [11, 12, 13, 14], [15, 16, 17, 18]]);
    let c = Matrix::new([[19], [20], [21], [22]]);
    assert_eq!(((a * b) * c), (a * (b * c)))
}

#[test]
fn test_product_of_diagonal_matrices() {
    let i = vec![1, 2, 3];
    let j = vec![4, 5, 6];

    let a = Matrix::new([[i[0], 0, 0], [0, i[1], 0], [0, 0, i[2]]]);
    let b = Matrix::new([[j[0], 0, 0], [0, j[1], 0], [0, 0, j[2]]]);
    assert_eq!(
        (a * b),
        Matrix::new([
            [i[0] * j[0], 0, 0],
            [0, i[1] * j[1], 0],
            [0, 0, i[2] * j[2],],
        ])
    )
}

#[test]
fn test_transpose_of_product() {
    let a = Matrix::new([[1, 2], [3, 4]]);
    let b = Matrix::new([[5, 6], [7, 8]]);

    let ab = a * b;
    let ab_t = ab.transpose();

    let a_t = a.transpose();
    let b_t = b.transpose();
    let b_t_a_t = b_t * a_t;

    assert_eq!(ab_t, b_t_a_t);
}

#[test]
fn test_matrix_vector_multiplication() {
    let a: Matrix<i32, 2, 3> = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let v: Vector<i32, 3> = Vector::new([7, 8, 9]);
    let expected: Vector<i32, 2> = Vector::new([50, 122]); // 1*7+2*8+3*9 = 7+16+27 = 50, 4*7+5*8+6*9 = 28+40+54=122
    assert_eq!(a * v, expected);
}

#[test]
fn test_multiplication_by_identity() {
    let a: Matrix<f64, 2, 3> = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let i3: Matrix<f64, 3, 3> = Matrix::identity();
    let i2: Matrix<f64, 2, 2> = Matrix::identity();

    assert_matrix_approx_eq(a * i3, a, 1e-9);
    assert_matrix_approx_eq(i2 * a, a, 1e-9);
}

#[test]
fn test_multiplication_by_zero() {
    let a: Matrix<i32, 2, 3> = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    let zero3x2: Matrix<i32, 3, 2> = Matrix::zeros();
    let zero2x2: Matrix<i32, 2, 2> = Matrix::zeros();
    let zero2x3: Matrix<i32, 2, 3> = Matrix::zeros();

    assert_eq!(a * zero3x2, zero2x2);
    let zero2x2_a: Matrix<i32, 2, 2> = Matrix::zeros(); // Need a different zero matrix for left multiplication
    assert_eq!(zero2x2_a * a, zero2x3);
}
