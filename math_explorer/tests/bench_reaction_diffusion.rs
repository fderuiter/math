use math_explorer::biology::diffusion::FiniteDifference2D;
use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
use std::time::Instant;

#[test]
fn bench_turing_2d_step() {
    let width = 300;
    let height = 300;
    let diff = FiniteDifference2D::new(width, height, 1.0, 1.0);
    let kinetics = SchnakenbergKinetics::default();

    // Create system with 90k elements
    let mut system = TuringSystem::new_with_kinetics(width * height, 1.0, 40.0, kinetics, diff);

    // Initialize with some values
    for i in 0..width * height {
        system.u_mut()[i] = 1.0 + (i as f64 * 0.01).sin();
        system.v_mut()[i] = 0.5 + (i as f64 * 0.02).cos();
    }

    let iterations = 100;
    let dt = 0.01;

    // Warmup
    for _ in 0..10 {
        system.step(dt).unwrap();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        system.step(dt).unwrap();
    }
    let duration = start.elapsed();

    println!(
        "Turing 2D ({}x{}) - {} iterations: {:?}",
        width, height, iterations, duration
    );
    println!(
        "Average time per iteration: {:?}",
        duration / iterations as u32
    );
}
