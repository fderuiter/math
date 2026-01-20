//! Temporal Tracking with Kalman Filter.
//!
//! Implements a **Discrete Kalman Filter** using the **Strategy Pattern** to allow interchangeable
//! physics models (e.g., Constant Velocity, Constant Acceleration).
//!
//! **Update (2026-06-XX):** This module now uses the generic `KalmanFilter` implementation from
//! `applied::algorithms::kalman`, adapted for the specific domain of Radar Gating.

use crate::applied::algorithms::kalman::{KalmanFilter, KalmanModel};
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
        // R is 1x1 matrix for scalar measurement
        DMatrix::from_row_slice(1, 1, &[self.measurement_noise_var])
    }
}

/// A wrapper around the generic Kalman Filter for 1D position and velocity tracking.
///
/// This maintains the original API of the radar gating module while using the unified core algorithm.
#[derive(Debug, Clone)]
pub struct TrackingFilter<M: KalmanModel = ConstantVelocityModel> {
    inner: KalmanFilter<M>,
}

impl TrackingFilter<ConstantVelocityModel> {
    /// Creates a new Kalman Filter with a specific physics model.
    ///
    /// # Arguments
    /// * `initial_amplitude` - Initial position guess.
    /// * `dt` - Time step in seconds.
    /// * `model` - The physics model implementation (e.g., `ConstantVelocityModel`).
    pub fn new(initial_amplitude: f64, dt: f64, model: ConstantVelocityModel) -> Self {
        let initial_state = DVector::from_vec(vec![initial_amplitude, 0.0]);
        let initial_covariance = DMatrix::identity(2, 2);

        Self {
            inner: KalmanFilter::new(initial_state, initial_covariance, model, dt),
        }
    }
}

impl<M: KalmanModel> TrackingFilter<M> {
    /// Creates a new TrackingFilter from an existing generic KalmanFilter.
    /// This allows injecting any physics model (Dependency Injection).
    pub fn new_from_filter(filter: KalmanFilter<M>) -> Self {
        Self { inner: filter }
    }

    /// Predicts the next state.
    pub fn predict(&mut self) {
        self.inner.predict();
    }

    /// Updates the state with a new measurement.
    pub fn update(&mut self, measured_amplitude: f64) {
        // Optimization: Use from_column_slice to avoid extra allocation if vector was larger,
        // though here we just need a 1-element vector. from_element is cleaner.
        let measurement = DVector::from_element(1, measured_amplitude);
        // Unwrap logic: In this controlled domain (scalar update), inversion should rarely fail given R > 0.
        // If it fails, we simply skip the update (robustness).
        let _ = self.inner.update(&measurement);
    }

    /// Returns the current estimated amplitude (position).
    /// Assumes state index 0 is position.
    pub fn amplitude(&self) -> f64 {
        // Safety: We assume the model has at least 1 state variable.
        self.inner.state.get(0).copied().unwrap_or(0.0)
    }

    /// Returns the current estimated velocity.
    /// Assumes state index 1 is velocity.
    pub fn velocity(&self) -> f64 {
        // Safety: We assume the model has at least 2 state variables.
        self.inner.state.get(1).copied().unwrap_or(0.0)
    }

    /// Returns the current state covariance matrix.
    ///
    /// Useful for visualizing uncertainty or debugging filter convergence.
    pub fn covariance(&self) -> &DMatrix<f64> {
        &self.inner.covariance
    }
}
