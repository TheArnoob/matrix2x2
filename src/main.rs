use matrix::Matrix2x2;

mod matrix;

fn main() {
    let mat_a = Matrix2x2::new(6., 4., 3., 5.);
    let mat_b = Matrix2x2::new(4., 2., 1., 3.);
    println!("{}", mat_a.get(0, 0).unwrap());
    println!("{mat_a}");
    println!("{}", mat_a.add(&mat_b));
    println!("{}", mat_a.sub(&mat_b));
    println!("{}", mat_a.mul(&mat_b));
}
