use matops::{Matrix, Vector};

#[test]
fn test_associativity() {
    let a = Matrix {
        data: [[1, 2, 3], [4, 5, 6]],
    };
    let b = Matrix {
        data: [[7, 8, 9, 10], [11, 12, 13, 14], [15, 16, 17, 18]],
    };
    let c = Matrix {
        data: [[19], [20], [21], [22]],
    };
    assert_eq!(((a * b) * c), (a * (b * c)))
}

#[test]
fn test_product_of_diagonal_matrices() {
    let i = vec![1, 2, 3];
    let j = vec![4, 5, 6];

    let a = Matrix {
        data: [[i[0], 0, 0], [0, i[1], 0], [0, 0, i[2]]],
    };
    let b = Matrix {
        data: [[j[0], 0, 0], [0, j[1], 0], [0, 0, j[2]]],
    };
    assert_eq!(
        (a * b),
        Matrix {
            data: [
                [i[0] * j[0], 0, 0],
                [0, i[1] * j[1], 0],
                [0, 0, i[2] * j[2],],
            ]
        }
    )
}

#[test]
fn test_transpose_of_product() {
    let a = Matrix {
        data: [[1, 2], [3, 4]],
    };
    let b = Matrix {
        data: [[5, 6], [7, 8]],
    };

    let ab = a * b;
    let ab_t = ab.transpose();

    let a_t = a.transpose();
    let b_t = b.transpose();
    let b_t_a_t = b_t * a_t;

    assert_eq!(ab_t, b_t_a_t);
}

#[test]
fn test_matrix_vector_multiplication() {
    let a: Matrix<i32, 2, 3> = Matrix {
        data: [[1, 2, 3], [4, 5, 6]],
    };
    let v: Vector<i32, 3> = Vector { data: [7, 8, 9] };
    let expected: Vector<i32, 2> = Vector { data: [50, 122] }; // 1*7+2*8+3*9 = 7+16+27 = 50, 4*7+5*8+6*9 = 28+40+54=122
    assert_eq!(a * v, expected);
}

#[test]
fn test_identity() {
    let i: Matrix<f64, 3, 3> = Matrix::identity();
    let expected: Matrix<f64, 3, 3> = Matrix {
        data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    assert_matrix_approx_eq(i, expected, 1e-9);
}

#[test]
fn test_multiplication_by_identity() {
    let a: Matrix<f64, 2, 3> = Matrix {
        data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
    };
    let i3: Matrix<f64, 3, 3> = Matrix::identity();
    let i2: Matrix<f64, 2, 2> = Matrix::identity();

    assert_matrix_approx_eq(a * i3, a, 1e-9);
    assert_matrix_approx_eq(i2 * a, a, 1e-9);
}

#[test]
fn test_determinant_2x2() {
    let a: Matrix<f64, 2, 2> = Matrix {
        data: [[1.0, 2.0], [3.0, 4.0]],
    };
    // det = 1*4 - 2*3 = 4 - 6 = -2
    assert!((a.determinant() - (-2.0)).abs() < 1e-9);
}

#[test]
fn test_determinant_3x3() {
    let a: Matrix<f64, 3, 3> = Matrix {
        data: [[6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]],
    };
    // det = 6*(-2*7 - 5*8) - 1*(4*7 - 5*2) + 1*(4*8 - (-2)*2)
    // det = 6*(-14 - 40) - 1*(28 - 10) + 1*(32 + 4)
    // det = 6*(-54) - 1*(18) + 1*(36)
    // det = -324 - 18 + 36 = -306
    assert!((a.determinant() - (-306.0)).abs() < 1e-9);
}

#[test]
fn test_determinant_singular() {
    let a: Matrix<f64, 2, 2> = Matrix {
        data: [[1.0, 2.0], [2.0, 4.0]], // Row 2 is 2 * Row 1
    };
    assert!(a.determinant().abs() < 1e-9); // Determinant should be close to 0

    let b: Matrix<f64, 3, 3> = Matrix {
        data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]], // Row 3 = 2*Row2 - Row1
    };
    assert!(b.determinant().abs() < 1e-9); // Determinant should be close to 0
}

// Helper for comparing floating point matrices
fn assert_matrix_approx_eq<const M: usize, const N: usize>(
    a: Matrix<f64, M, N>,
    b: Matrix<f64, M, N>,
    tolerance: f64,
) {
    for i in 0..M {
        for j in 0..N {
            assert!(
                (a.data[i][j] - b.data[i][j]).abs() < tolerance,
                "Matrices differ at [{}][{}]: {} != {}",
                i,
                j,
                a.data[i][j],
                b.data[i][j]
            );
        }
    }
}

#[test]
fn test_inverse_2x2() {
    let a: Matrix<f64, 2, 2> = Matrix {
        data: [[4.0, 7.0], [2.0, 6.0]],
    };
    // det = 4*6 - 7*2 = 24 - 14 = 10
    // inv = 1/10 * [[6, -7], [-2, 4]] = [[0.6, -0.7], [-0.2, 0.4]]
    let a_inv = a.inverse().unwrap();
    let expected_inv: Matrix<f64, 2, 2> = Matrix {
        data: [[0.6, -0.7], [-0.2, 0.4]],
    };
    let identity: Matrix<f64, 2, 2> = Matrix::identity();

    assert_matrix_approx_eq(a_inv, expected_inv, 1e-9);
    assert_matrix_approx_eq(a * a_inv, identity, 1e-9);
    assert_matrix_approx_eq(a_inv * a, identity, 1e-9);
}

#[test]
fn test_inverse_3x3() {
    let a: Matrix<f64, 3, 3> = Matrix {
        data: [[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]],
    };
    // det = 1*(1*0 - 4*6) - 2*(0*0 - 4*5) + 3*(0*6 - 1*5)
    // det = 1*(-24) - 2*(-20) + 3*(-5)
    // det = -24 + 40 - 15 = 1
    // Using an online calculator for the inverse:
    let expected_inv: Matrix<f64, 3, 3> = Matrix {
        data: [[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]],
    };
    let identity: Matrix<f64, 3, 3> = Matrix::identity();
    let a_inv = a.inverse().unwrap();

    assert_matrix_approx_eq(a_inv, expected_inv, 1e-9);
    assert_matrix_approx_eq(a * a_inv, identity, 1e-9);
    assert_matrix_approx_eq(a_inv * a, identity, 1e-9);
}

#[test]
fn test_inverse_non_invertible() {
    let a: Matrix<f64, 2, 2> = Matrix {
        data: [[1.0, 2.0], [2.0, 4.0]], // Determinant is 0
    };
    assert!(a.inverse().is_none());
}

#[test]
fn test_transpose_transpose() {
    let a: Matrix<i32, 2, 3> = Matrix {
        data: [[1, 2, 3], [4, 5, 6]],
    };
    let a_t = a.transpose();
    let a_tt = a_t.transpose();
    assert_eq!(a, a_tt);
}

#[test]
fn test_multiplication_by_zero() {
    let a: Matrix<i32, 2, 3> = Matrix {
        data: [[1, 2, 3], [4, 5, 6]],
    };
    let zero3x2: Matrix<i32, 3, 2> = Matrix {
        data: [[0, 0], [0, 0], [0, 0]],
    };
    let zero2x2: Matrix<i32, 2, 2> = Matrix {
        data: [[0, 0], [0, 0]],
    };
    let zero2x3: Matrix<i32, 2, 3> = Matrix {
        data: [[0, 0, 0], [0, 0, 0]],
    };

    assert_eq!(a * zero3x2, zero2x2);
    // Need a 2x2 zero matrix to multiply from the left
    let zero2x2_a: Matrix<i32, 2, 2> = Matrix {
        data: [[0, 0], [0, 0]],
    };
    assert_eq!(zero2x2_a * a, zero2x3);
}

#[test]
fn test_1x1_matrix() {
    let a: Matrix<i32, 1, 1> = Matrix { data: [[5]] };
    let b: Matrix<i32, 1, 1> = Matrix { data: [[3]] };
    let v: Vector<i32, 1> = Vector { data: [2] };

    // Multiplication (i32)
    assert_eq!(a * b, Matrix { data: [[15]] });

    // Transpose (i32)
    assert_eq!(a.transpose(), a);

    // Matrix-Vector Multiplication (i32)
    assert_eq!(a * v, Vector { data: [10] });

    // Determinant (f64)
    let a_f64: Matrix<f64, 1, 1> = Matrix { data: [[5.0]] };
    assert!((a_f64.determinant() - 5.0).abs() < 1e-9);

    // Identity (f64)
    assert_matrix_approx_eq(
        Matrix::<f64, 1, 1>::identity(),
        Matrix { data: [[1.0]] },
        1e-9,
    );

    // Inverse (f64)
    let inv_a = a_f64.inverse().unwrap();
    let expected_inv: Matrix<f64, 1, 1> = Matrix {
        data: [[1.0 / 5.0]],
    };
    assert_matrix_approx_eq(inv_a, expected_inv, 1e-9);
    assert_matrix_approx_eq(a_f64 * inv_a, Matrix::identity(), 1e-9);
}

// Add tests for Add, Sub, Scalar Mul/Div when implemented

#[test]
fn test_addition() {
    let a: Matrix<i32, 2, 2> = Matrix {
        data: [[1, 2], [3, 4]],
    };
    let b: Matrix<i32, 2, 2> = Matrix {
        data: [[5, 6], [7, 8]],
    };
    let expected: Matrix<i32, 2, 2> = Matrix {
        data: [[6, 8], [10, 12]],
    };
    assert_eq!(a + b, expected);
}

#[test]
fn test_subtraction() {
    let a: Matrix<i32, 2, 2> = Matrix {
        data: [[5, 8], [10, 12]],
    };
    let b: Matrix<i32, 2, 2> = Matrix {
        data: [[1, 2], [3, 4]],
    };
    let expected: Matrix<i32, 2, 2> = Matrix {
        data: [[4, 6], [7, 8]],
    };
    assert_eq!(a - b, expected);
}

#[test]
fn test_scalar_multiplication() {
    let a: Matrix<i32, 2, 2> = Matrix {
        data: [[1, 2], [3, 4]],
    };
    let scalar = 3;
    let expected: Matrix<i32, 2, 2> = Matrix {
        data: [[3, 6], [9, 12]],
    };
    assert_eq!(a * scalar, expected);
    // Test multiplication from the left as well if implemented
    // assert_eq!(scalar * a, expected);
}

#[test]
fn test_scalar_division() {
    let a: Matrix<f64, 2, 2> = Matrix {
        data: [[3.0, 6.0], [9.0, 12.0]],
    };
    let scalar = 3.0;
    let expected: Matrix<f64, 2, 2> = Matrix {
        data: [[1.0, 2.0], [3.0, 4.0]],
    };
    assert_matrix_approx_eq(a / scalar, expected, 1e-9);
}

// Add more tests for edge cases if necessary, e.g., different types, dimensions
