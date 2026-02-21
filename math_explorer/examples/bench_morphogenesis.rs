use math_explorer::biology::morphogenesis::TuringSystem;
use std::time::Instant;

fn main() {
    let size = 100_000;
    let iterations = 1000;

    // Use builder
    let mut system = TuringSystem::builder()
        .size(size)
        .diffusion_rates(0.1, 0.05)
        .with_1d_diffusion(1.0)
        .build()
        .expect("Failed to build TuringSystem");

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
        "Time for {} iterations with size {}: {:?}",
        iterations, size, duration
    );
    let avg_ns = duration.as_nanos() as f64 / iterations as f64;
    println!("Average per step: {:.2} ns", avg_ns);
}
