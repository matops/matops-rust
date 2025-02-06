use matops::Vector;

fn main() {
    let c = 1.0;
    let d = 2.0;
    let v = Vector { data: [1.0, 2.0] };
    let w = Vector { data: [3.0, 4.0] };
    println!("c*(d*v) = {})", (v * d) * c);
    println!("(c*d)*v) = {})", v * (d * c));
    println!("(c+d)*v) = {})", v * (d + c));
    println!("c*v + d*v) = {})", v * c + v * d);
    println!("c(v+w) = {})", (v + w) * c);
    println!("c*v + c*w) = {})", v * c + w * c);
}
