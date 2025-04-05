use matops::Matrix;

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
