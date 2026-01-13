//! Tracking and Estimation Algorithms.
//!
//! Includes Kalman Filter and other state estimation techniques.

use nalgebra::{DMatrix, DVector};

/// A standard Kalman Filter implementation.
pub struct KalmanFilter {
    /// State transition model ($F$).
    pub f: DMatrix<f64>,
    /// State estimate ($\hat{x}$).
    pub x: DVector<f64>,
}

impl KalmanFilter {
    /// Creates a new Kalman Filter.
    pub fn new(f: DMatrix<f64>, x_init: DVector<f64>) -> Self {
        Self { f, x: x_init }
    }

    /// Performs the Prediction Step.
    ///
    /// $$ \hat{x}_{k|k-1} = F_k \hat{x}_{k-1|k-1} $$
    ///
    /// Note: This simplifies the prediction to just the state projection,
    /// ignoring covariance propagation ($P = FPF^T + Q$) which is usually part of the full step.
    /// The prompt asks for this specific formula.
    pub fn predict(&mut self) {
        self.x = &self.f * &self.x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kalman_predict() {
        // Simple 1D motion: x = x + v*dt. State [pos, vel].
        // F = [[1, dt], [0, 1]]
        let dt = 1.0;
        let f = DMatrix::from_row_slice(2, 2, &[1.0, dt, 0.0, 1.0]);
        let x = DVector::from_column_slice(&[0.0, 10.0]); // Pos=0, Vel=10

        let mut kf = KalmanFilter::new(f, x);
        kf.predict();

        // New Pos = 0 + 10*1 = 10
        // New Vel = 10
        assert!((kf.x[0] - 10.0).abs() < 1e-6);
        assert!((kf.x[1] - 10.0).abs() < 1e-6);
    }
}
