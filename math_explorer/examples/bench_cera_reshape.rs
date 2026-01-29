use math_explorer::climate::cera::{Cera, CeraConfig};
use nalgebra::DMatrix;
use std::time::Instant;

fn main() {
    let num_levels = 50;
    let in_channels = 10;
    let latent_channels = 8;
    let aligned_channels = 8;
    let output_size = 20;
    let batch_size = 2000;

    let config = CeraConfig {
        learning_rate: 0.001,
        lambda_pred: 0.1,
        lambda_emd: 0.01,
        epochs: 1,
        batch_size,
        in_channels,
        latent_channels,
        aligned_channels,
        num_levels,
        output_size,
    };

    let cera = Cera::new(config).expect("Failed to create Cera");

    let total_rows = batch_size * num_levels;
    let inputs = DMatrix::from_fn(total_rows, in_channels, |_, _| rand::random::<f32>());

    println!("Running benchmark with batch_size={}, num_levels={}, channels={}...", batch_size, num_levels, in_channels);

    // Warmup
    for _ in 0..5 {
        let _ = cera.predict(&inputs);
    }

    let iterations = 20;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = cera.predict(&inputs);
    }
    let duration = start.elapsed();

    let avg_time = duration.as_secs_f64() / iterations as f64;
    println!("Total time: {:?}", duration);
    println!("Average time per prediction: {:.6} seconds", avg_time);
}
