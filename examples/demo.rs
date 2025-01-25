use matops::{Matrix, Vector};

fn main() {
    // Temporary test code
    let m = Matrix {
        data: [[1, 2], [3, 4]],
    };
    let v = Vector { data: [5, 6] };
    println!("{:?}", m);
    println!("{:?}", v.dot(&v));
}
