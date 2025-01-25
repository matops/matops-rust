use matops::{Matrix, Vector};

fn main() {
    let v = Vector { data: [2.0, 3.0] };
    let composition = Matrix {
        data: [[2.0, 0.0], [1.0, -2.0]],
    };
    let rotation = Matrix {
        data: [[1.0, -2.0], [1.0, 0.0]],
    };
    let sheer = Matrix {
        data: [[0.0, 2.0], [1.0, 0.0]],
    };
    println!("{:?}", composition * v);
    println!("{:?}", sheer * (rotation * v));
}
