use math_explorer::biology::diffusion::{FiniteDifference2D, SpatialDiffusion};
use std::time::Instant;

fn main() {
    let width = 1000;
    let height = 1000;
    let n = width * height;

    let diff_solver = FiniteDifference2D::new(width, height, 1.0, 1.0);

    let u = vec![1.0; n];
    let v = vec![0.5; n];

    // Warmup
    let mut dummy = 0.0;
    for _ in 0..10 {
        diff_solver.map_diffusion(
            [u.as_slice(), v.as_slice()],
            [0.1, 0.05],
            |_i, _vals, _diffs| {
                dummy += 1.0;
            }
        );
    }

    let iterations = 100;
    let start = Instant::now();

    for _ in 0..iterations {
        diff_solver.map_diffusion(
            [u.as_slice(), v.as_slice()],
            [0.1, 0.05],
            |i, vals, diffs| {
                // Mimic some work to ensure compiler doesn't optimize away everything
                // but keep it minimal so we measure diffusion overhead.
                // We use black_box style if possible, but here we just assign to volatile or similar?
                // Actually, just writing to a buffer is enough.
                unsafe {
                    std::ptr::write_volatile(&mut dummy, vals[0] + diffs[0]);
                }
            }
        );
    }

    let duration = start.elapsed();
    println!("Time for {} iterations on {}x{} grid: {:?}", iterations, width, height, duration);
    let avg_ms = duration.as_millis() as f64 / iterations as f64;
    println!("Average per step: {:.2} ms", avg_ms);
}
