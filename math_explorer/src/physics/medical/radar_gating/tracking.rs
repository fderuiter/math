//! Temporal Tracking with Kalman Filter.
//!
//! Implements a **Discrete Kalman Filter** using the **Strategy Pattern** to allow interchangeable
//! physics models (e.g., Constant Velocity, Constant Acceleration).

use nalgebra::{Matrix2, RowVector2, Vector1, Vector2};

/// Defines the physics model for the Kalman Filter.
///
/// This trait decouples the filter algorithm (Predict/Update) from the specific physics
/// (Transition Matrix, Measurement Matrix).
pub trait KalmanModel {
    /// Returns the State Transition Matrix ($F_k$).
    fn transition_matrix(&self, dt: f64) -> Matrix2<f64>;

    /// Returns the Measurement Matrix ($H_k$).
    fn measurement_matrix(&self) -> RowVector2<f64>;

    /// Returns the Process Noise Covariance ($Q_k$).
    fn process_noise(&self, dt: f64) -> Matrix2<f64>;

    /// Returns the Measurement Noise Covariance ($R_k$).
    fn measurement_noise(&self) -> f64;
}

/// Constant Velocity (CV) Model.
/// State: [Position, Velocity]
#[derive(Debug, Clone)]
pub struct ConstantVelocityModel {
    pub process_noise_var: f64,
    pub measurement_noise_var: f64,
}

impl ConstantVelocityModel {
    pub fn new(process_noise_var: f64, measurement_noise_var: f64) -> Self {
        Self {
            process_noise_var,
            measurement_noise_var,
        }
    }
}

impl KalmanModel for ConstantVelocityModel {
    fn transition_matrix(&self, dt: f64) -> Matrix2<f64> {
        // [1, dt]
        // [0, 1 ]
        Matrix2::new(1.0, dt, 0.0, 1.0)
    }

    fn measurement_matrix(&self) -> RowVector2<f64> {
        // [1, 0] -> Measure position only
        RowVector2::new(1.0, 0.0)
    }

    fn process_noise(&self, _dt: f64) -> Matrix2<f64> {
        // Simplified Q: I * var
        Matrix2::identity() * self.process_noise_var
    }

    fn measurement_noise(&self) -> f64 {
        self.measurement_noise_var
    }
}

/// A Kalman Filter for 1D position and velocity tracking.
#[derive(Debug, Clone)]
pub struct TrackingFilter<M: KalmanModel> {
    /// State vector $x_k$.
    pub state: Vector2<f64>,
    /// State Covariance Matrix $P_k$.
    pub covariance: Matrix2<f64>,
    /// Physics Model Strategy.
    pub model: M,
    /// Time step $\Delta t$.
    pub dt: f64,
}

impl<M: KalmanModel> TrackingFilter<M> {
    /// Creates a new Kalman Filter with a specific physics model.
    ///
    /// # Arguments
    /// * `initial_amplitude` - Initial position guess.
    /// * `dt` - Time step in seconds.
    /// * `model` - The physics model implementation (e.g., `ConstantVelocityModel`).
    pub fn new(initial_amplitude: f64, dt: f64, model: M) -> Self {
        Self {
            state: Vector2::new(initial_amplitude, 0.0), // Assume 0 velocity initially
            covariance: Matrix2::identity(),             // High initial uncertainty
            model,
            dt,
        }
    }

    /// Predicts the next state using the model's transition matrix.
    ///
    /// $$ x_{k|k-1} = F x_{k-1|k-1} $$
    /// $$ P_{k|k-1} = F P_{k-1|k-1} F^T + Q $$
    pub fn predict(&mut self) {
        let f_mat = self.model.transition_matrix(self.dt);
        let q_mat = self.model.process_noise(self.dt);

        // Predict State
        self.state = f_mat * self.state;

        // Predict Covariance
        self.covariance = f_mat * self.covariance * f_mat.transpose() + q_mat;
    }

    /// Updates the state with a new measurement.
    ///
    /// $$ y_k = z_k - H x_{k|k-1} $$
    /// $$ S_k = H P H^T + R $$
    /// $$ K_k = P H^T S^{-1} $$
    /// $$ x_{k|k} = x_{k|k-1} + K y $$
    /// $$ P_{k|k} = (I - K H) P_{k|k-1} $$
    pub fn update(&mut self, measured_amplitude: f64) {
        let h_mat = self.model.measurement_matrix();
        let r_val = self.model.measurement_noise();

        // Innovation y
        let measurement = Vector1::new(measured_amplitude);
        let prediction = h_mat * self.state;
        let y = measurement - prediction;

        // Innovation Covariance S
        // H (1x2) * P (2x2) * H^T (2x1) -> (1x1) scalar
        let s = (h_mat * self.covariance * h_mat.transpose())[0] + r_val;

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
