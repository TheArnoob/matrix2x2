use matrix::Matrix2x2;
mod matrix;

fn main(){
    let mat_a = Matrix2x2::new(6., 4., 3., 5.);
    let mat_b = Matrix2x2::new(4., 2., 1., 3.);
    let mat_c = Matrix2x2::new(5., 3., 4., 6.);
    let mat_d = Matrix2x2::new(3., 1., 2., 4.);
    println!("{mat_a}");
    println!("{}", mat_a.add(&mat_b));
    println!("{}", mat_a.sub(&mat_b));
    println!("{}", mat_a.mul(&mat_b));
    println!("{}", mat_a.get(1, 1).unwrap());
    println!("Determinant of matrix a: {}", mat_a.det());
    println!("Inverse of matrix b: {}", mat_b.inv());
    println!("Determinant of matrix c: {}", mat_c.det());
    println!("Inverse of matrix d: {}", mat_d.inv());
}
