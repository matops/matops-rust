pub mod arithmetic;
pub mod creation;
pub mod edge_cases;
pub mod indexing;
pub mod linear_algebra;
pub mod multiplication;

use matops::Matrix;

pub fn assert_matrix_approx_eq<const M: usize, const N: usize>(
    a: Matrix<f64, M, N>,
    b: Matrix<f64, M, N>,
    tolerance: f64,
) {
    for i in 0..M {
        for j in 0..N {
            assert!(
                (a[i][j] - b[i][j]).abs() < tolerance,
                "Matrices differ at [{}][{}]: {} != {} (difference: {})",
                i,
                j,
                a[i][j],
                b[i][j],
                (a[i][j] - b[i][j]).abs()
            );
        }
    }
}
