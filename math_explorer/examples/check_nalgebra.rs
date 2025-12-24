use nalgebra::DVector;
use math_explorer::pure_math::analysis::ode::VectorOperations;

fn require_vector_ops<T: VectorOperations>(_: T) {}

fn main() {
    let v1 = DVector::from_vec(vec![1.0, 2.0]);
    require_vector_ops(v1);
    println!("Success");
}
