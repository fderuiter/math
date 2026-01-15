//! Temporal Tracking with Kalman Filter.
//!
//! Implements a **Discrete Kalman Filter** using the **Strategy Pattern** to allow interchangeable
//! physics models (e.g., Constant Velocity, Constant Acceleration).
//!
//! Refactored to use the generic `applied::algorithms::kalman` module.

use crate::applied::algorithms::kalman::KalmanFilter;
pub use crate::applied::algorithms::kalman::KalmanModel;
use nalgebra::{DMatrix, DVector};

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

// Implement the Generic KalmanModel (DMatrix based)
impl KalmanModel for ConstantVelocityModel {
    fn transition_matrix(&self, dt: f64) -> DMatrix<f64> {
        // [1, dt]
        // [0, 1 ]
        DMatrix::from_row_slice(2, 2, &[1.0, dt, 0.0, 1.0])
    }

    fn measurement_matrix(&self) -> DMatrix<f64> {
        // [1, 0] -> Measure position only
        DMatrix::from_row_slice(1, 2, &[1.0, 0.0])
    }

    fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
        // Simplified Q: I * var
        DMatrix::identity(2, 2) * self.process_noise_var
    }

    fn measurement_noise(&self) -> DMatrix<f64> {
        DMatrix::from_element(1, 1, self.measurement_noise_var)
    }
}

/// A Kalman Filter for 1D position and velocity tracking.
///
/// Wraps the generic `KalmanFilter` for the Radar Gating domain.
#[derive(Debug, Clone)]
pub struct TrackingFilter<M: KalmanModel> {
    inner: KalmanFilter<M>,
    dt: f64,
}

impl<M: KalmanModel> TrackingFilter<M> {
    /// Creates a new Kalman Filter with a specific physics model.
    ///
    /// # Arguments
    /// * `initial_amplitude` - Initial position guess.
    /// * `dt` - Time step in seconds.
    /// * `model` - The physics model implementation (e.g., `ConstantVelocityModel`).
    pub fn new(initial_amplitude: f64, dt: f64, model: M) -> Self {
        let initial_state = DVector::from_vec(vec![initial_amplitude, 0.0]);
        let initial_covariance = DMatrix::identity(2, 2);

        let inner = KalmanFilter::new(initial_state, initial_covariance, model);

        Self { inner, dt }
    }

    /// Predicts the next state using the model's transition matrix.
    pub fn predict(&mut self) {
        self.inner.predict(self.dt);
    }

    /// Updates the state with a new measurement.
    pub fn update(&mut self, measured_amplitude: f64) {
        let measurement = DVector::from_element(1, measured_amplitude);
        // We unwrap here to preserve the original API's panic-on-failure behavior
        // (though the original didn't check for singularity).
        self.inner.update(&measurement).expect("Kalman update failed (singular innovation)");
    }

    /// Returns the current estimated amplitude (position).
    pub fn amplitude(&self) -> f64 {
        self.inner.state[0]
    }

    /// Returns the current estimated velocity.
    pub fn velocity(&self) -> f64 {
        self.inner.state[1]
    }

    /// Access the underlying generic filter.
    pub fn inner(&self) -> &KalmanFilter<M> {
        &self.inner
    }
}
