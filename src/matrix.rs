#[derive(Debug, PartialEq)]
pub struct Matrix2x2 {
    /// Contains the matrix elements
    /// a00 -> data[0]
    /// a01 -> data[1]
    /// a10 -> data[2]
    /// a11 -> data[3]
    data: [f64; 4],
}

impl Matrix2x2 {
    pub fn new(a00: f64, a01: f64, a10: f64, a11: f64) -> Matrix2x2 {
        Matrix2x2 {
            data: [a00, a01, a10, a11],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<f64> {
        if x > 1 {
            return None;
        }
        if y > 1 {
            return None;
        }
        Some(self.data[2 * x + y])
    }

    pub fn add(&self, other: &Matrix2x2) -> Matrix2x2 {
        let added_matrix = Matrix2x2::new(
            other.data[0] + self.data[0],
            other.data[1] + self.data[1],
            other.data[2] + self.data[2],
            other.data[3] + self.data[3],
        );
        added_matrix
    }
    pub fn sub(&self, other: &Matrix2x2) -> Matrix2x2 {
        let sub_matrix = Matrix2x2::new(
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2],
            self.data[3] - other.data[3],
        );
        sub_matrix
    }
    pub fn mul(&self, other: &Matrix2x2) -> Matrix2x2 {
        let c00 = self.data[0] * other.data[0] + self.data[1] * other.data[2];
        let c01 = self.data[0] * other.data[1] + self.data[1] * other.data[3];
        let c10 = self.data[2] * other.data[0] + self.data[3] * other.data[2];
        let c11 = self.data[2] * other.data[1] + self.data[3] * other.data[3];
        let mul_matrix = Matrix2x2::new(c00, c01, c10, c11);
        mul_matrix
    }

    pub fn det(&self) -> f64 {
        let determinent = self.data[0] * self.data[3] - self.data[1] * self.data[2];
        determinent
    }

    pub fn inv(&self) -> Matrix2x2 {
        let inverse: f64 = 1. / self.det();
        let inv_matrix = Matrix2x2::new(
            inverse * self.data[3],
            inverse * -self.data[1],
            inverse * -self.data[2],
            inverse * self.data[0],
        );
        inv_matrix
    }
}

impl std::fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_be_printed = format!(
            "{}\t{}\n{}\t{}",
            self.data[0], self.data[1], self.data[2], self.data[3]
        );
        to_be_printed.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn matrix_add() {
        use crate::Matrix2x2;
        let mat_a = Matrix2x2::new(5., 5., 0., 3.);
        let mat_b = Matrix2x2::new(4., 5., 6., 7.);
        let mat_c = Matrix2x2::new(9., 10., 6., 10.);
        assert_eq!(mat_a.add(&mat_b), mat_c)
    }

    #[test]
    fn matrix_sub() {
        use crate::Matrix2x2;
        let mat_a = Matrix2x2::new(5., 5., 0., 3.);
        let mat_b = Matrix2x2::new(4., 5., 6., 7.);
        let mat_c = Matrix2x2::new(1., 0., -6., -4.);
        assert_eq!(mat_a.sub(&mat_b), mat_c)
    }
    #[test]
    fn matrix_mul() {
        use crate::Matrix2x2;
        let mat_a = Matrix2x2::new(5., 5., 0., 3.);
        let mat_b = Matrix2x2::new(4., 5., 6., 7.);
        let mat_c = Matrix2x2::new(50., 60., 18., 21.);
        assert_eq!(mat_a.mul(&mat_b), mat_c)
    }
    #[test]
    fn matrix_det() {
        use crate::Matrix2x2;
        let mat_a = Matrix2x2::new(5., 5., 0., 3.);
        let mat_b = Matrix2x2::new(4., 5., 6., 7.);
        let mat_c = Matrix2x2::new(9., 11., 9., 12.);
        assert_eq!(mat_a.det(), 15.);
        assert_eq!(mat_b.det(), -2.);
        assert_eq!(mat_c.det(), 9.);
    }
}
