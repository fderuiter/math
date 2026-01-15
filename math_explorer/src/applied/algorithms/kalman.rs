//! Generic Kalman Filter Implementation.
//!
//! A robust implementation of the Discrete Kalman Filter using dynamic matrices (`DMatrix`).
//!
//! # Architecture
//!
//! This module uses the **Strategy Pattern** via the `KalmanModel` trait.
//! The `KalmanFilter` struct is the context that executes the algorithm, while
//! the `KalmanModel` defines the physics of the system.
//!
//! # Usage
//!
//! ```rust
//! use math_explorer::applied::algorithms::kalman::{KalmanFilter, KalmanModel};
//! use nalgebra::{DMatrix, DVector};
//!
//! struct ConstantVelocity {
//!     noise: f64,
//! }
//!
//! impl KalmanModel for ConstantVelocity {
//!     fn transition_matrix(&self, dt: f64) -> DMatrix<f64> {
//!         // [1, dt], [0, 1]
//!         DMatrix::from_row_slice(2, 2, &[1.0, dt, 0.0, 1.0])
//!     }
//!     fn measurement_matrix(&self) -> DMatrix<f64> {
//!         // [1, 0]
//!         DMatrix::from_row_slice(1, 2, &[1.0, 0.0])
//!     }
//!     fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
//!         DMatrix::identity(2, 2) * 0.1
//!     }
//!     fn measurement_noise(&self) -> DMatrix<f64> {
//!         DMatrix::from_element(1, 1, self.noise)
//!     }
//! }
//!
//! let model = ConstantVelocity { noise: 1.0 };
//! let init_state = DVector::from_element(2, 0.0);
//! let init_cov = DMatrix::identity(2, 2);
//! let mut kf = KalmanFilter::new(init_state, init_cov, model);
//! ```

use nalgebra::{DMatrix, DVector};

/// Defines the physics and noise characteristics of a linear system.
pub trait KalmanModel {
    /// Returns the State Transition Matrix ($F_k$).
    ///
    /// Defines how the state evolves from $k-1$ to $k$ without control input.
    fn transition_matrix(&self, dt: f64) -> DMatrix<f64>;

    /// Returns the Measurement Matrix ($H_k$).
    ///
    /// Maps the state space into the measurement space.
    fn measurement_matrix(&self) -> DMatrix<f64>;

    /// Returns the Process Noise Covariance ($Q_k$).
    ///
    /// Represents uncertainty in the model (e.g., wind gusts, friction).
    fn process_noise(&self, dt: f64) -> DMatrix<f64>;

    /// Returns the Measurement Noise Covariance ($R_k$).
    ///
    /// Represents sensor noise.
    fn measurement_noise(&self) -> DMatrix<f64>;
}

/// A Generic Discrete Kalman Filter.
///
/// Stores the current state estimate and error covariance.
#[derive(Debug, Clone)]
pub struct KalmanFilter<M: KalmanModel> {
    /// The current state estimate $\hat{x}_{k|k}$.
    pub state: DVector<f64>,
    /// The current error covariance matrix $P_{k|k}$.
    pub covariance: DMatrix<f64>,
    /// The physics model strategy.
    pub model: M,
}

impl<M: KalmanModel> KalmanFilter<M> {
    /// Creates a new Kalman Filter.
    ///
    /// # Arguments
    ///
    /// * `initial_state` - Initial guess for $\hat{x}$.
    /// * `initial_covariance` - Initial uncertainty $P$.
    /// * `model` - The system model.
    pub fn new(
        initial_state: DVector<f64>,
        initial_covariance: DMatrix<f64>,
        model: M,
    ) -> Self {
        Self {
            state: initial_state,
            covariance: initial_covariance,
            model,
        }
    }

    /// Performs the **Prediction** step.
    ///
    /// Projects the state and covariance forward in time.
    ///
    /// $$ \hat{x}_{k|k-1} = F_k \hat{x}_{k-1|k-1} $$
    /// $$ P_{k|k-1} = F_k P_{k-1|k-1} F_k^T + Q_k $$
    pub fn predict(&mut self, dt: f64) {
        let f = self.model.transition_matrix(dt);
        let q = self.model.process_noise(dt);

        // x = F * x
        self.state = &f * &self.state;

        // P = F * P * F^T + Q
        self.covariance = &f * &self.covariance * f.transpose() + q;
    }

    /// Performs the **Update** (Correction) step.
    ///
    /// Corrects the predicted state using a new measurement.
    ///
    /// $$ y_k = z_k - H_k \hat{x}_{k|k-1} $$
    /// $$ S_k = H_k P_{k|k-1} H_k^T + R_k $$
    /// $$ K_k = P_{k|k-1} H_k^T S_k^{-1} $$
    /// $$ \hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k y_k $$
    /// $$ P_{k|k} = (I - K_k H_k) P_{k|k-1} $$
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement vector $z_k$.
    pub fn update(&mut self, measurement: &DVector<f64>) -> Result<(), String> {
        let h = self.model.measurement_matrix();
        let r = self.model.measurement_noise();

        // Innovation y = z - Hx
        let prediction = &h * &self.state;
        let innovation = measurement - prediction;

        // Innovation Covariance S = HPH^T + R
        let s = &h * &self.covariance * h.transpose() + r;

        // Kalman Gain K = PH^T S^-1
        // We use Cholesky decomposition for stability if S is symmetric positive definite (it should be).
        // Fallback to LU or pseudo-inverse if needed.
        let s_inv = match s.clone().cholesky() {
            Some(cholesky) => cholesky.inverse(),
            None => s.clone().try_inverse().ok_or("Singular innovation matrix")?,
        };

        let k = &self.covariance * h.transpose() * s_inv;

        // Update State x = x + Ky
        self.state += &k * innovation;

        // Update Covariance P = (I - KH)P
        let identity = DMatrix::identity(self.state.len(), self.state.len());
        self.covariance = (identity - &k * h) * &self.covariance;

        Ok(())
    }
}
