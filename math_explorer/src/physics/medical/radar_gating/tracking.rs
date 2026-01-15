//! Temporal Tracking with Kalman Filter.
//!
//! Implements a **Discrete Kalman Filter** with a **Constant Velocity (CV)** model to smooth
//! the breathing signal and handle system latency.

use nalgebra::{Matrix2, Vector1, Vector2};

/// A Kalman Filter for 1D position and velocity tracking (Constant Velocity model).
#[derive(Debug, Clone)]
pub struct TrackingFilter {
    /// State vector $x_k = [A_k, \dot{A}_k]^T$ (Position, Velocity).
    pub state: Vector2<f64>,
    /// State Covariance Matrix $P_k$.
    pub covariance: Matrix2<f64>,
    /// Process Noise Covariance $Q$.
    pub process_noise: Matrix2<f64>,
    /// Measurement Noise Covariance $R$ (scalar for 1D measurement).
    pub measurement_noise: f64,
    /// Time step $\Delta t$.
    pub dt: f64,
}

impl TrackingFilter {
    /// Creates a new Kalman Filter.
    ///
    /// # Arguments
    /// * `initial_amplitude` - Initial position guess.
    /// * `dt` - Time step in seconds (e.g., 0.05).
    /// * `process_noise_var` - Variance for process noise (trust in model).
    /// * `measurement_noise_var` - Variance for measurement noise (sensor uncertainty).
    pub fn new(
        initial_amplitude: f64,
        dt: f64,
        process_noise_var: f64,
        measurement_noise_var: f64,
    ) -> Self {
        Self {
            state: Vector2::new(initial_amplitude, 0.0), // Assume 0 velocity initially
            covariance: Matrix2::identity(),             // High initial uncertainty
            process_noise: Matrix2::identity() * process_noise_var, // Simplified Q
            measurement_noise: measurement_noise_var,
            dt,
        }
    }

    /// Predicts the next state.
    ///
    /// $$ x_{k|k-1} = F x_{k-1|k-1} $$
    /// $$ P_{k|k-1} = F P_{k-1|k-1} F^T + Q $$
    pub fn predict(&mut self) {
        let dt = self.dt;
        // State Transition Matrix F
        // [1, dt]
        // [0, 1 ]
        let f_mat = Matrix2::new(1.0, dt, 0.0, 1.0);

        // Predict State
        self.state = f_mat * self.state;

        // Predict Covariance
        self.covariance = f_mat * self.covariance * f_mat.transpose() + self.process_noise;
    }

    /// Updates the state with a new measurement.
    ///
    /// $$ y_k = z_k - H x_{k|k-1} $$
    /// $$ S_k = H P H^T + R $$
    /// $$ K_k = P H^T S^{-1} $$
    /// $$ x_{k|k} = x_{k|k-1} + K y $$
    /// $$ P_{k|k} = (I - K H) P_{k|k-1} $$
    ///
    /// # Arguments
    /// * `measured_amplitude` - The measured position $z_k$.
    pub fn update(&mut self, measured_amplitude: f64) {
        // Measurement Matrix H = [1, 0]
        // We only measure position.
        let h_mat = nalgebra::RowVector2::new(1.0, 0.0);

        // Innovation y
        let measurement = Vector1::new(measured_amplitude);
        let prediction = h_mat * self.state;
        let y = measurement - prediction;

        // Innovation Covariance S
        // H (1x2) * P (2x2) * H^T (2x1) -> (1x1) scalar
        let s = (h_mat * self.covariance * h_mat.transpose())[0] + self.measurement_noise;

        // Kalman Gain K = P H^T S^-1
        // (2x2) * (2x1) * scalar -> (2x1)
        let k_gain = (self.covariance * h_mat.transpose()) / s;

        // Update State
        self.state += k_gain * y;

        // Update Covariance
        // P = (I - K H) P
        let identity = Matrix2::identity();
        self.covariance = (identity - k_gain * h_mat) * self.covariance;
    }

    /// Returns the current estimated amplitude (position).
    pub fn amplitude(&self) -> f64 {
        self.state[0]
    }

    /// Returns the current estimated velocity.
    pub fn velocity(&self) -> f64 {
        self.state[1]
    }
}
