

use domain_biology::biology::diffusion::FiniteDifference2D;
use domain_biology::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
use std::time::Instant;

#[test]
fn bench_turing_2d_step() {
    let width = 300;
    let height = 300;
    let diff = FiniteDifference2D::new(
        math_commons::math_kernel::types::Dimension(width),
        math_commons::math_kernel::types::Dimension(height),
        math_commons::math_kernel::types::StepSize(1.0),
        math_commons::math_kernel::types::StepSize(1.0),
    );
    let kinetics = SchnakenbergKinetics::default();

    // Create system with 90k elements
    let mut system = TuringSystem::new_with_kinetics(
        math_commons::math_kernel::types::Dimension(width * height),
        domain_biology::biology::morphogenesis::DiffusionCoeff(1.0),
        domain_biology::biology::morphogenesis::DiffusionCoeff(40.0),
        kinetics,
        diff,
    );

    // Initialize with some values
    for i in 0..width * height {
        system.u_mut()[i] = 1.0 + (i as f64 * 0.01).sin();
        system.v_mut()[i] = 0.5 + (i as f64 * 0.02).cos();
    }

    let iterations = 100;
    let dt = 0.01;

    // Warmup
    for _ in 0..10 {
        system.step(dt);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        system.step(dt);
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
