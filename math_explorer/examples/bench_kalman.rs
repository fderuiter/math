use math_explorer::applied::algorithms::kalman::{KalmanFilter, KalmanModel};
use nalgebra::{DMatrix, DVector};
use std::time::Instant;

struct BenchmarkModel {
    process_noise: f64,
    measurement_noise: f64,
}

impl KalmanModel for BenchmarkModel {
    fn transition_matrix(&self, dt: f64, out: &mut DMatrix<f64>) {
        // 4x4 Constant Velocity Model (2D Pos, 2D Vel)
        // [1 0 dt 0]
        // [0 1 0 dt]
        // [0 0 1  0]
        // [0 0 0  1]
        out.fill(0.0);
        out.fill_diagonal(1.0);
        out[(0, 2)] = dt;
        out[(1, 3)] = dt;
    }

    fn measurement_matrix(&self, out: &mut DMatrix<f64>) {
        // Measure position only (2D)
        // [1 0 0 0]
        // [0 1 0 0]
        out.fill(0.0);
        out[(0, 0)] = 1.0;
        out[(1, 1)] = 1.0;
    }

    fn process_noise(&self, _dt: f64, out: &mut DMatrix<f64>) {
        out.fill_diagonal(self.process_noise);
    }

    fn measurement_noise(&self, out: &mut DMatrix<f64>) {
        out.fill_diagonal(self.measurement_noise);
    }
}

fn main() {
    let dt = 0.1;
    let model = BenchmarkModel {
        process_noise: 0.1,
        measurement_noise: 1.0,
    };

    let initial_state = DVector::from_vec(vec![0.0, 0.0, 10.0, 10.0]);
    let initial_covariance = DMatrix::identity(4, 4);

    let mut kf = KalmanFilter::new(initial_state, initial_covariance, model, dt);

    // Warmup
    for _ in 0..100 {
        kf.predict();
        let z = DVector::from_vec(vec![1.0, 1.0]);
        let _ = kf.update(&z);
    }

    let iterations = 100_000;
    let start = Instant::now();

    for i in 0..iterations {
        kf.predict();
        let z = DVector::from_vec(vec![i as f64 * dt * 10.0, i as f64 * dt * 10.0]);
        let _ = kf.update(&z);
    }

    let duration = start.elapsed();
    println!("Time for {} iterations: {:?}", iterations, duration);
    println!("Average time per iteration: {:?}", duration / iterations as u32);
}
