// tests/matrix/indexing.rs
use matops::Matrix;

#[test]
fn test_indexing_read_write() {
    let mut a: Matrix<i32, 2, 3> = Matrix::new([[1, 2, 3], [4, 5, 6]]);

    // Test Index trait (read)
    assert_eq!(a[(0, 0)], 1);
    assert_eq!(a[(0, 1)], 2);
    assert_eq!(a[(0, 2)], 3);
    assert_eq!(a[(1, 0)], 4);
    assert_eq!(a[(1, 1)], 5);
    assert_eq!(a[(1, 2)], 6);

    // Test IndexMut trait (write)
    a[(1, 1)] = 99;
    assert_eq!(a[(1, 1)], 99);

    // Verify the rest of the matrix is unchanged
    let expected: Matrix<i32, 2, 3> = Matrix::new([[1, 2, 3], [4, 99, 6]]);
    assert_eq!(a, expected);
}

#[test]
#[should_panic]
fn test_indexing_out_of_bounds_row() {
    let a: Matrix<i32, 2, 2> = Matrix::zeros();
    let _ = a[(2, 0)]; // Access row index 2 which is out of bounds
}

#[test]
#[should_panic]
fn test_indexing_out_of_bounds_col() {
    let a: Matrix<i32, 2, 2> = Matrix::zeros();
    let _ = a[(0, 2)]; // Access column index 2 which is out of bounds
}
