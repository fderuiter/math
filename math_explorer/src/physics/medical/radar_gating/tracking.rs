//! Temporal Tracking with Kalman Filter.
//!
//! Implements a **Discrete Kalman Filter** using the **Strategy Pattern** to separate the
//! mathematical engine from the physical model (e.g., Constant Velocity).

use std::marker::PhantomData;
use nalgebra::{Matrix2, Vector2, Vector1, Dim, RealField, DefaultAllocator, OMatrix, OVector};
use nalgebra::allocator::Allocator;

/// Defines the physics model for the Kalman Filter.
///
/// This trait provides the System Matrices ($F$, $Q$, $H$, $R$) required for the prediction
/// and update steps.
///
/// # Generics
/// * `T` - Field type (e.g., `f64`).
/// * `D` - State Dimension (e.g., 2 for Position/Velocity).
/// * `M` - Measurement Dimension (e.g., 1 for Position only).
pub trait KalmanModel<T: RealField, D: Dim, M: Dim>
where
    DefaultAllocator: Allocator<T, D, D> + Allocator<T, M, D> + Allocator<T, M, M> + Allocator<T, D, M>,
{
    /// State Transition Matrix ($F$).
    fn f(&self, dt: T) -> OMatrix<T, D, D>;

    /// Process Noise Covariance Matrix ($Q$).
    fn q(&self, dt: T) -> OMatrix<T, D, D>;

    /// Measurement Matrix ($H$).
    fn h(&self) -> OMatrix<T, M, D>;

    /// Measurement Noise Covariance Matrix ($R$).
    fn r(&self) -> OMatrix<T, M, M>;
}

/// A Generic Discrete Kalman Filter.
///
/// The filter estimates the state of a linear dynamic system.
#[derive(Debug, Clone)]
pub struct KalmanFilter<T, D, M, Model>
where
    T: RealField,
    D: Dim,
    M: Dim,
    Model: KalmanModel<T, D, M>,
    DefaultAllocator: Allocator<T, D> + Allocator<T, D, D> + Allocator<T, M, D> + Allocator<T, M, M> + Allocator<T, M> + Allocator<T, D, M>,
{
    /// State vector $\hat{x}_{k}$.
    pub state: OVector<T, D>,
    /// State Covariance Matrix $P_k$.
    pub covariance: OMatrix<T, D, D>,
    /// The physics model providing system matrices.
    model: Model,
    /// PhantomData to hold unused type parameters `M`.
    _marker: PhantomData<M>,
}

impl<T, D, M, Model> KalmanFilter<T, D, M, Model>
where
    T: RealField,
    D: Dim,
    M: Dim,
    Model: KalmanModel<T, D, M>,
    DefaultAllocator: Allocator<T, D> + Allocator<T, D, D> + Allocator<T, M, D> + Allocator<T, M, M> + Allocator<T, M> + Allocator<T, D, M>,
{
    /// Creates a new Kalman Filter with the given initial state and model.
    pub fn new(initial_state: OVector<T, D>, initial_covariance: OMatrix<T, D, D>, model: Model) -> Self {
        Self {
            state: initial_state,
            covariance: initial_covariance,
            model,
            _marker: PhantomData,
        }
    }

    /// Predicts the next state using the time step `dt`.
    pub fn predict(&mut self, dt: T) {
        let f = self.model.f(dt.clone());
        let q = self.model.q(dt);

        // x = F * x
        self.state = &f * &self.state;

        // P = F * P * F^T + Q
        // Note: f.transpose() allocates a new matrix, so ownership is fine.
        // We clone self.covariance because we need it for multiplication before updating it.
        // Actually, &f * &self.covariance is fine as a reference.
        // The issue is if we assign to self.covariance while using it.
        // But the RHS is evaluated fully before assignment.
        // The issue in the update step was moving out of self.covariance. Here it should be fine.
        self.covariance = &f * &self.covariance * f.transpose() + q;
    }

    /// Updates the state with a new measurement vector.
    pub fn update(&mut self, measurement: OVector<T, M>) {
        let h = self.model.h();
        let r = self.model.r();

        // Innovation y = z - H * x
        let y = &measurement - &h * &self.state;

        // Innovation Covariance S = H * P * H^T + R
        let s = &h * &self.covariance * h.transpose() + r;

        // Kalman Gain K = P * H^T * S^-1
        // We use Cholesky decomposition for stability, or simple inverse if S is small.
        // For generality here, we try inverse. In production, consider robust solvers.
        if let Some(s_inv) = s.try_inverse() {
            let k = &self.covariance * h.transpose() * s_inv;

            // x = x + K * y
            self.state = &self.state + &k * y;

            // P = (I - K * H) * P
            let identity = OMatrix::<T, D, D>::identity_generic(self.covariance.shape_generic().0, self.covariance.shape_generic().1);
            // We use .clone() on self.covariance because we are modifying it in place essentially.
            self.covariance = (identity - k * h) * self.covariance.clone();
        } else {
             // If S is singular, we skip update or handle error.
             // For now, we assume invertibility as per standard KF assumptions.
        }
    }

    /// Returns a reference to the current state.
    pub fn state(&self) -> &OVector<T, D> {
        &self.state
    }
}

// --- Implementations for Backward Compatibility ---

use nalgebra::Const;

/// A Constant Velocity (CV) Model for 1D tracking.
/// State: [Position, Velocity]
/// Measurement: [Position]
#[derive(Debug, Clone, Copy)]
pub struct ConstantVelocityModel {
    pub process_noise_var: f64,
    pub measurement_noise_var: f64,
}

impl KalmanModel<f64, Const<2>, Const<1>> for ConstantVelocityModel {
    fn f(&self, dt: f64) -> Matrix2<f64> {
        Matrix2::new(1.0, dt, 0.0, 1.0)
    }

    fn q(&self, _dt: f64) -> Matrix2<f64> {
        // Simplified Q: discrete noise injected primarily into velocity or both.
        // The original implementation used Identity * var, which implies noise on position and velocity.
        Matrix2::identity() * self.process_noise_var
    }

    fn h(&self) -> nalgebra::Matrix<f64, Const<1>, Const<2>, nalgebra::ArrayStorage<f64, 1, 2>> {
        // Measure position only: [1, 0]
        nalgebra::Matrix1x2::new(1.0, 0.0)
    }

    fn r(&self) -> nalgebra::Matrix1<f64> {
         nalgebra::Matrix1::new(self.measurement_noise_var)
    }
}

/// A Kalman Filter for 1D position and velocity tracking (Constant Velocity model).
///
/// This struct wraps the generic `KalmanFilter` to maintain the original API.
#[derive(Debug, Clone)]
pub struct TrackingFilter {
    inner: KalmanFilter<f64, Const<2>, Const<1>, ConstantVelocityModel>,
    dt: f64,
}

impl TrackingFilter {
    /// Creates a new Kalman Filter.
    pub fn new(initial_amplitude: f64, dt: f64, process_noise_var: f64, measurement_noise_var: f64) -> Self {
        let model = ConstantVelocityModel {
            process_noise_var,
            measurement_noise_var,
        };

        let initial_state = Vector2::new(initial_amplitude, 0.0);
        let initial_covariance = Matrix2::identity();

        let inner = KalmanFilter::new(initial_state, initial_covariance, model);

        Self { inner, dt }
    }

    pub fn predict(&mut self) {
        self.inner.predict(self.dt);
    }

    pub fn update(&mut self, measured_amplitude: f64) {
        let measurement = Vector1::new(measured_amplitude);
        self.inner.update(measurement);
    }

    pub fn amplitude(&self) -> f64 {
        self.inner.state[0]
    }

    pub fn velocity(&self) -> f64 {
        self.inner.state[1]
    }
}
