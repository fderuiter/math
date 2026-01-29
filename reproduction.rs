use nalgebra::{DMatrix, DVector};

fn main() {
    let m = DMatrix::from_element(2, 2, 1.0);
    let v = DVector::from_element(3, 1.0); // Mismatch!

    // This should panic
    let _res = &m * &v;
}
