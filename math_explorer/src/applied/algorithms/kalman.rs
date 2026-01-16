//! Generic Kalman Filter Implementation.
//!
//! A dimension-agnostic implementation of the Discrete Kalman Filter using `nalgebra::DMatrix`.
//! This module allows for state estimation of linear systems with arbitrary state and measurement dimensions.

use nalgebra::{DMatrix, DVector};

/// Defines the physics/dynamics model for the Kalman Filter.
///
/// Implementors provide the system matrices that define how the state evolves and how it is measured.
///
/// # Mathematical Model
///
/// State Evolution: $x_k = F_k x_{k-1} + w_k$
/// Measurement: $z_k = H_k x_k + v_k$
///
/// Where:
/// - $F_k$: Transition Matrix
/// - $H_k$: Measurement Matrix
/// - $w_k \sim N(0, Q_k)$: Process Noise
/// - $v_k \sim N(0, R_k)$: Measurement Noise
pub trait KalmanModel {
    /// Returns the State Transition Matrix ($F_k$) of size $(n \times n)$.
    fn transition_matrix(&self, dt: f64) -> DMatrix<f64>;

    /// Returns the Measurement Matrix ($H_k$) of size $(m \times n)$.
    fn measurement_matrix(&self) -> DMatrix<f64>;

    /// Returns the Process Noise Covariance ($Q_k$) of size $(n \times n)$.
    fn process_noise(&self, dt: f64) -> DMatrix<f64>;

    /// Returns the Measurement Noise Covariance ($R_k$) of size $(m \times m)$.
    fn measurement_noise(&self) -> DMatrix<f64>;
}

/// A generic Discrete Kalman Filter.
///
/// Uses the **Strategy Pattern** via the `KalmanModel` trait to decouple the estimation algorithm
/// from the specific system dynamics.
#[derive(Debug, Clone)]
pub struct KalmanFilter<M: KalmanModel> {
    /// State vector estimate ($\hat{x}$).
    pub state: DVector<f64>,
    /// State covariance matrix ($P$).
    pub covariance: DMatrix<f64>,
    /// The physics/system model strategy.
    pub model: M,
    /// Time step for the filter (if fixed).
    pub dt: f64,
}

impl<M: KalmanModel> KalmanFilter<M> {
    /// Creates a new Kalman Filter.
    ///
    /// # Arguments
    ///
    /// * `initial_state` - Initial estimate of the state vector.
    /// * `initial_covariance` - Initial uncertainty covariance matrix.
    /// * `model` - The system model implementation.
    /// * `dt` - Default time step.
    pub fn new(
        initial_state: DVector<f64>,
        initial_covariance: DMatrix<f64>,
        model: M,
        dt: f64,
    ) -> Self {
        Self {
            state: initial_state,
            covariance: initial_covariance,
            model,
            dt,
        }
    }

    /// Performs the **Prediction Step**.
    ///
    /// Projects the current state estimate and covariance forward in time.
    ///
    /// $$ \hat{x}_{k|k-1} = F_k \hat{x}_{k-1|k-1} $$
    /// $$ P_{k|k-1} = F_k P_{k-1|k-1} F_k^T + Q_k $$
    pub fn predict(&mut self) {
        let f = self.model.transition_matrix(self.dt);
        let q = self.model.process_noise(self.dt);

        self.state = &f * &self.state;
        self.covariance = &f * &self.covariance * f.transpose() + q;
    }

    /// Performs the **Update Step** with a new measurement.
    ///
    /// Incorporates the new observation $z_k$ to refine the state estimate.
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

        // Innovation
        let y = measurement - &h * &self.state;

        // Innovation Covariance S = H P H^T + R
        let s = &h * &self.covariance * h.transpose() + r;

        // Invert S.
        // For 1D measurements, this is trivial. For nD, we need matrix inversion.
        // Kalman Filter requires S to be invertible (positive definite).
        let s_inv = s.try_inverse().ok_or("Failed to invert innovation covariance matrix (singular)")?;

        // Kalman Gain K = P H^T S^-1
        let k = &self.covariance * h.transpose() * s_inv;

        // Update State
        self.state = &self.state + &k * y;

        // Update Covariance P = (I - K H) P
        let identity = DMatrix::identity(self.state.len(), self.state.len());
        self.covariance = (identity - &k * &h) * &self.covariance;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock Model for testing 1D constant velocity
    struct MockCvModel {
        process_noise: f64,
        measurement_noise: f64,
    }

    impl KalmanModel for MockCvModel {
        fn transition_matrix(&self, dt: f64) -> DMatrix<f64> {
            // [1, dt]
            // [0, 1]
            DMatrix::from_row_slice(2, 2, &[1.0, dt, 0.0, 1.0])
        }
        fn measurement_matrix(&self) -> DMatrix<f64> {
            // [1, 0]
            DMatrix::from_row_slice(1, 2, &[1.0, 0.0])
        }
        fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
            DMatrix::identity(2, 2) * self.process_noise
        }
        fn measurement_noise(&self) -> DMatrix<f64> {
            DMatrix::from_element(1, 1, self.measurement_noise)
        }
    }

    #[test]
    fn test_kalman_predict_logic() {
        let dt = 1.0;
        let model = MockCvModel {
            process_noise: 0.0,
            measurement_noise: 1.0,
        };
        let x_init = DVector::from_vec(vec![0.0, 10.0]); // Pos=0, Vel=10
        let p_init = DMatrix::identity(2, 2);

        let mut kf = KalmanFilter::new(x_init, p_init, model, dt);
        kf.predict();

        // New Pos = 0 + 10*1 = 10
        // New Vel = 10
        assert!((kf.state[0] - 10.0).abs() < 1e-6);
        assert!((kf.state[1] - 10.0).abs() < 1e-6);
    }
}
