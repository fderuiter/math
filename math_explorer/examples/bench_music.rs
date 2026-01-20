use math_explorer::physics::medical::radar_gating::super_resolution::MusicEstimator;
use num_complex::Complex;
use std::time::Instant;

fn main() {
    let samples = 64;
    let snapshots = 20;
    let targets = 2;
    let mut estimator = MusicEstimator::new(samples, snapshots, targets);

    // Feed dummy data
    // Signal: Target at index 10 and 20 (frequencies)
    for _ in 0..snapshots {
        let mut data = Vec::with_capacity(samples);
        for i in 0..samples {
            let t = i as f64;
            // Generate some signal
            let val = Complex::new(0.0, 0.1 * t).exp() + Complex::new(0.0, 0.3 * t).exp();
            data.push(val);
        }
        estimator.add_snapshot(&data).unwrap();
    }

    println!("Benchmarking MusicEstimator::compute_spectrum...");

    // Warmup
    for _ in 0..10 {
         let _ = estimator.compute_spectrum(0.0, 10.0, 0.05, 4.0e9, 3.0e8).unwrap();
    }

    let start = Instant::now();
    let iterations = 100;
    // Range 0 to 10m, step 0.001m -> 10,001 points per call.
    // 100 iterations -> 1,000,100 points.
    for _ in 0..iterations {
        let spectrum = estimator.compute_spectrum(0.0, 10.0, 0.001, 4.0e9, 3.0e8).unwrap();
        // Just use the result to make sure it's not optimized away
        if spectrum.len() != 10001 {
             panic!("Unexpected spectrum length");
        }
    }
    let duration = start.elapsed();

    println!("Total time for {} iterations: {:?}", iterations, duration);
    println!("Average time per iteration: {:?}", duration / iterations as u32);
}
