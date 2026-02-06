use math_explorer::biology::morphogenesis::TuringSystem;
use std::time::Instant;

fn main() {
    let size = 10_000;
    let iterations = 10_000;

    println!("Benchmarking TuringSystem with size {} for {} iterations...", size, iterations);

    // Setup
    // d_u = 1.0, d_v = 0.5, dx = 1.0
    let mut system = TuringSystem::new(size, 1.0, 0.5, 1.0);

    // Initialize with noise to ensure non-trivial behavior
    for i in 0..size {
         system.u_mut()[i] = 1.0 + (i as f64 % 100.0) * 0.001;
         system.v_mut()[i] = 0.5 - (i as f64 % 100.0) * 0.001;
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

    println!("Total time: {:?}", duration);
    println!("Time per step: {:?}", duration / iterations as u32);
}
