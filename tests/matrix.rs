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
