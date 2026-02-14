use math_explorer::biology::diffusion::FiniteDifference1D;
use math_explorer::biology::morphogenesis::SchnakenbergKinetics;
use math_explorer::biology::reaction_diffusion::ReactionDiffusionSystem;
use std::time::Instant;

fn main() {
    let size = 100_000;
    let iterations = 1000;

    // Setup generic system
    let dx = 1.0;
    let d_u = 0.1;
    let d_v = 0.05; // Matching bench_morphogenesis values

    // Note: bench_morphogenesis used: new(size, 0.1, 0.05, 1.0) -> d_u=0.1, d_v=0.05, dx=1.0?
    // Let's check bench_morphogenesis.rs content again.
    // let mut system = TuringSystem::new(size, 0.1, 0.05, 1.0);
    // TuringSystem::new(size, d_u, d_v, dx)
    // So d_u=0.1, d_v=0.05, dx=1.0. Correct.

    let kinetics = SchnakenbergKinetics::default();
    let diffusion = FiniteDifference1D::new(dx);
    let diffusion_coeffs = vec![d_u, d_v];

    let mut system = ReactionDiffusionSystem::new(2, size, kinetics, diffusion, diffusion_coeffs);

    // Initialize (just to be safe, though 0 is valid)
    for i in 0..size {
        system.state.concentrations[0][i] = 1.0;
        system.state.concentrations[1][i] = 0.5;
    }

    // Warmup
    for _ in 0..100 {
        system.step(0.01);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        system.step(0.01);
    }
    let duration = start.elapsed();

    println!(
        "Generic RD Time for {} iterations with size {}: {:?}",
        iterations, size, duration
    );
    let avg_ns = duration.as_nanos() as f64 / iterations as f64;
    println!("Average per step: {:.2} ns", avg_ns);
}
