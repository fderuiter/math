use math_explorer::pure_math::analysis::ode::VecState;
use std::time::Instant;

#[test]
fn bench_vecstate_allocation() {
    let size = 100_000;
    let iterations = 100;

    // Create large vectors
    let v1 = VecState(vec![1.0; size]);
    let v2 = VecState(vec![2.0; size]);

    // Measure Add
    let start = Instant::now();
    let mut result = v1.clone();
    for _ in 0..iterations {
        // We want to simulate the solver loop: state = state + delta
        // Here we just do repeated additions.
        // We clone v2 because in the solver we usually add different things,
        // but `Add` consumes self, so `result` is consumed and a new one returned.
        result = result + v2.clone();
    }
    let duration = start.elapsed();
    println!("VecState Add (100k elements, {} iters): {:?}", iterations, duration);

    // Measure Mul
    let start_mul = Instant::now();
    let mut result_mul = v1.clone();
    for _ in 0..iterations {
        result_mul = result_mul * 1.01;
    }
    let duration_mul = start_mul.elapsed();
    println!("VecState Mul (100k elements, {} iters): {:?}", iterations, duration_mul);
}
