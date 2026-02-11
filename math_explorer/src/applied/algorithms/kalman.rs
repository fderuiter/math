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
    /// Writes the State Transition Matrix ($F_k$) of size $(n \times n)$ into `out`.
    fn transition_matrix(&self, dt: f64, out: &mut DMatrix<f64>);

    /// Writes the Measurement Matrix ($H_k$) of size $(m \times n)$ into `out`.
    fn measurement_matrix(&self, out: &mut DMatrix<f64>);

    /// Writes the Process Noise Covariance ($Q_k$) of size $(n \times n)$ into `out`.
    fn process_noise(&self, dt: f64, out: &mut DMatrix<f64>);

    /// Writes the Measurement Noise Covariance ($R_k$) of size $(m \times m)$ into `out`.
    fn measurement_noise(&self, out: &mut DMatrix<f64>);
}

/// Defines a generalized Kalman System (Linear or Non-Linear).
///
/// This trait supports both Standard Kalman Filters (Linear) and Extended Kalman Filters (EKF).
///
/// # Mathematical Model
///
/// State Evolution: $x_k = f(x_{k-1}) + w_k$
/// Measurement: $z_k = h(x_k) + v_k$
///
/// Where:
/// - $f(x)$: State Transition Function
/// - $h(x)$: Measurement Function
/// - $F_k = \frac{\partial f}{\partial x}$: Jacobian of State Transition
/// - $H_k = \frac{\partial h}{\partial x}$: Jacobian of Measurement
pub trait KalmanSystem {
    /// Predicts the next state $x_k$ and returns the transition Jacobian $F_k$.
    ///
    /// # Arguments
    /// * `state` - The current state estimate $\hat{x}_{k-1|k-1}$.
    /// * `dt` - The time step $\Delta t$.
    /// * `out_state` - Buffer to write the predicted state $\hat{x}_{k|k-1}$.
    /// * `out_jacobian` - Buffer to write the transition Jacobian $F_k$.
    fn predict_state(&self, state: &DVector<f64>, out_state: &mut DVector<f64>, out_jacobian: &mut DMatrix<f64>, dt: f64);

    /// Predicts the measurement $z_k$ and returns the measurement Jacobian $H_k$.
    ///
    /// # Arguments
    /// * `state` - The predicted state estimate $\hat{x}_{k|k-1}$.
    /// * `out_measurement` - Buffer to write the predicted measurement $z_k$.
    /// * `out_jacobian` - Buffer to write the measurement Jacobian $H_k$.
    fn predict_measurement(&self, state: &DVector<f64>, out_measurement: &mut DVector<f64>, out_jacobian: &mut DMatrix<f64>);

    /// Writes the Process Noise Covariance $Q_k$ into `out_noise`.
    fn process_noise(&self, dt: f64, out_noise: &mut DMatrix<f64>);

    /// Writes the Measurement Noise Covariance $R_k$ into `out_noise`.
    fn measurement_noise(&self, out_noise: &mut DMatrix<f64>);
}

impl<T: KalmanModel> KalmanSystem for T {
    fn predict_state(&self, state: &DVector<f64>, out_state: &mut DVector<f64>, out_jacobian: &mut DMatrix<f64>, dt: f64) {
        // F = transition_matrix(dt)
        self.transition_matrix(dt, out_jacobian);

        // x_pred = F * state
        // Optimize: use mul_to to avoid allocation
        out_jacobian.mul_to(state, out_state);
    }

    fn predict_measurement(&self, state: &DVector<f64>, out_measurement: &mut DVector<f64>, out_jacobian: &mut DMatrix<f64>) {
        // H = measurement_matrix()
        self.measurement_matrix(out_jacobian);

        // z_pred = H * state
        out_jacobian.mul_to(state, out_measurement);
    }

    fn process_noise(&self, dt: f64, out_noise: &mut DMatrix<f64>) {
        KalmanModel::process_noise(self, dt, out_noise);
    }

    fn measurement_noise(&self, out_noise: &mut DMatrix<f64>) {
        KalmanModel::measurement_noise(self, out_noise);
    }
}

/// Internal buffers for State Prediction (size n).
#[derive(Debug, Clone)]
struct KalmanStateBuffers {
    x_pred: DVector<f64>,
    f: DMatrix<f64>,
    q: DMatrix<f64>,
    temp_nn_1: DMatrix<f64>, // For F*P, I-KH
    temp_nn_2: DMatrix<f64>, // For F*P*F^T, (I-KH)*P
}

/// Internal buffers for Measurement Update (size m).
#[derive(Debug, Clone)]
struct KalmanMeasurementBuffers {
    z_pred: DVector<f64>,
    h: DMatrix<f64>,
    r: DMatrix<f64>,
    y: DVector<f64>,
    s: DMatrix<f64>,
    k: DMatrix<f64>,
    temp_mn: DMatrix<f64>, // For H*P
    temp_nm: DMatrix<f64>, // For P*H^T
}

/// A generic Discrete Kalman Filter.
///
/// Uses the **Strategy Pattern** via the `KalmanSystem` trait to decouple the estimation algorithm
/// from the specific system dynamics.
#[derive(Debug, Clone)]
pub struct KalmanFilter<M: KalmanSystem> {
    /// State vector estimate ($\hat{x}$).
    pub state: DVector<f64>,
    /// State covariance matrix ($P$).
    pub covariance: DMatrix<f64>,
    /// The physics/system model strategy.
    pub model: M,
    /// Time step for the filter (if fixed).
    pub dt: f64,

    // Internal Buffers
    buffers: KalmanStateBuffers,
    meas_buffers: Option<KalmanMeasurementBuffers>,
}

impl<M: KalmanSystem> KalmanFilter<M> {
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
        let n = initial_state.len();
        let buffers = KalmanStateBuffers {
            x_pred: DVector::zeros(n),
            f: DMatrix::zeros(n, n),
            q: DMatrix::zeros(n, n),
            temp_nn_1: DMatrix::zeros(n, n),
            temp_nn_2: DMatrix::zeros(n, n),
        };

        Self {
            state: initial_state,
            covariance: initial_covariance,
            model,
            dt,
            buffers,
            meas_buffers: None,
        }
    }

    /// Performs the **Prediction Step**.
    ///
    /// Projects the current state estimate and covariance forward in time.
    ///
    /// $$ \hat{x}_{k|k-1} = f(\hat{x}_{k-1|k-1}) $$
    /// $$ P_{k|k-1} = F_k P_{k-1|k-1} F_k^T + Q_k $$
    pub fn predict(&mut self) {
        let b = &mut self.buffers;

        // 1. Predict State x_{k|k-1} and Jacobian F_k
        // Writes to b.x_pred and b.f
        self.model.predict_state(&self.state, &mut b.x_pred, &mut b.f, self.dt);

        // 2. Get Process Noise Q_k
        // Writes to b.q
        self.model.process_noise(self.dt, &mut b.q);

        // 3. Update State Estimate
        // x = x_pred
        self.state.copy_from(&b.x_pred);

        // 4. Update Covariance P = F P F^T + Q

        // temp1 = F * P
        b.f.mul_to(&self.covariance, &mut b.temp_nn_1);

        // temp2 = temp1 * F^T = (F * P) * F^T
        b.temp_nn_1.mul_to(&b.f.transpose(), &mut b.temp_nn_2);

        // P = temp2 + Q
        // We can do this by copying temp2 to P and adding Q
        self.covariance.copy_from(&b.temp_nn_2);
        self.covariance += &b.q;
    }

    /// Performs the **Update Step** with a new measurement.
    ///
    /// Incorporates the new observation $z_k$ to refine the state estimate.
    ///
    /// $$ y_k = z_k - h(\hat{x}_{k|k-1}) $$
    /// $$ S_k = H_k P_{k|k-1} H_k^T + R_k $$
    /// $$ K_k = P_{k|k-1} H_k^T S_k^{-1} $$
    /// $$ \hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k y_k $$
    /// $$ P_{k|k} = (I - K_k H_k) P_{k|k-1} $$
    ///
    /// # Arguments
    ///
    /// * `measurement` - The measurement vector $z_k$.
    pub fn update(&mut self, measurement: &DVector<f64>) -> Result<(), String> {
        let m = measurement.len();
        let n = self.state.len();

        // Ensure measurement buffers are initialized and correct size
        if self.meas_buffers.is_none() || self.meas_buffers.as_ref().unwrap().z_pred.len() != m {
             self.meas_buffers = Some(KalmanMeasurementBuffers {
                z_pred: DVector::zeros(m),
                h: DMatrix::zeros(m, n),
                r: DMatrix::zeros(m, m),
                y: DVector::zeros(m),
                s: DMatrix::zeros(m, m),
                k: DMatrix::zeros(n, m),
                temp_mn: DMatrix::zeros(m, n),
                temp_nm: DMatrix::zeros(n, m),
            });
        }

        // Unwrap is safe because we just set it
        let mb = self.meas_buffers.as_mut().unwrap();
        let sb = &mut self.buffers;

        // 1. Predict Measurement z_pred and Jacobian H
        self.model.predict_measurement(&self.state, &mut mb.z_pred, &mut mb.h);

        // 2. Get Measurement Noise R
        self.model.measurement_noise(&mut mb.r);

        // 3. Innovation y = z - z_pred
        // y = measurement - z_pred
        measurement.sub_to(&mb.z_pred, &mut mb.y);

        // 4. Innovation Covariance S = H P H^T + R

        // temp_mn = H * P
        mb.h.mul_to(&self.covariance, &mut mb.temp_mn);

        // S = temp_mn * H^T = (H * P) * H^T
        mb.temp_mn.mul_to(&mb.h.transpose(), &mut mb.s);

        // S += R
        mb.s += &mb.r;

        // 5. Invert S
        // In-place inversion to avoid allocation
        if !mb.s.try_inverse_mut() {
             return Err("Failed to invert innovation covariance matrix (singular)".to_string());
        }
        // Now mb.s contains S^-1

        // 6. Kalman Gain K = P H^T S^-1

        // temp_nm = P * H^T = (H * P)^T = temp_mn^T
        // We already computed temp_mn = H * P.
        mb.temp_mn.transpose_to(&mut mb.temp_nm);

        // K = temp_nm * s_inv (where s_inv is now stored in mb.s)
        mb.temp_nm.mul_to(&mb.s, &mut mb.k);

        // 7. Update State x = x + K * y
        // reuse sb.x_pred as temporary buffer for K * y (size n)
        mb.k.mul_to(&mb.y, &mut sb.x_pred);
        self.state += &sb.x_pred;

        // 8. Update Covariance P = (I - K H) P
        // P = P - K H P
        // We have K. We have H. We have P.
        // K * H is (n x n).
        // (K * H) * P is (n x n).

        // Let's calculate K * (H * P). We have H * P in temp_mn.
        // temp_nn_1 = K * temp_mn
        mb.k.mul_to(&mb.temp_mn, &mut sb.temp_nn_1);

        // P = P - temp_nn_1
        self.covariance -= &sb.temp_nn_1;

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
        fn transition_matrix(&self, dt: f64, out: &mut DMatrix<f64>) {
            // [1, dt]
            // [0, 1]
            out[(0, 0)] = 1.0;
            out[(0, 1)] = dt;
            out[(1, 0)] = 0.0;
            out[(1, 1)] = 1.0;
        }
        fn measurement_matrix(&self, out: &mut DMatrix<f64>) {
            // [1, 0]
            out[(0, 0)] = 1.0;
            out[(0, 1)] = 0.0;
        }
        fn process_noise(&self, _dt: f64, out: &mut DMatrix<f64>) {
            out.fill_diagonal(self.process_noise);
        }
        fn measurement_noise(&self, out: &mut DMatrix<f64>) {
            out[(0, 0)] = self.measurement_noise;
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

    #[test]
    fn test_extended_kalman_filter() {
        // Define a non-linear model: x_{k} = sqrt(x_{k-1})
        // Measurement: z_k = x_k^2
        struct NonLinearModel;

        impl KalmanSystem for NonLinearModel {
            fn predict_state(
                &self,
                state: &DVector<f64>,
                out_state: &mut DVector<f64>,
                out_jacobian: &mut DMatrix<f64>,
                _dt: f64,
            ) {
                let x = state[0];
                let x_pred = x.sqrt();
                // Derivative of sqrt(x) is 1 / (2 * sqrt(x))
                let f_jacobian = 0.5 / x.sqrt();

                out_state[0] = x_pred;
                out_jacobian[(0, 0)] = f_jacobian;
            }

            fn predict_measurement(&self, state: &DVector<f64>, out_measurement: &mut DVector<f64>, out_jacobian: &mut DMatrix<f64>) {
                let x = state[0];
                let z_pred = x.powi(2);
                // Derivative of x^2 is 2x
                let h_jacobian = 2.0 * x;

                out_measurement[0] = z_pred;
                out_jacobian[(0, 0)] = h_jacobian;
            }

            fn process_noise(&self, _dt: f64, out_noise: &mut DMatrix<f64>) {
                out_noise[(0, 0)] = 0.1;
            }

            fn measurement_noise(&self, out_noise: &mut DMatrix<f64>) {
                out_noise[(0, 0)] = 0.1;
            }
        }

        let model = NonLinearModel;
        // Start at x=100.
        // Predict -> sqrt(100) = 10.
        // Update with z=100 (which corresponds to x=10).
        let initial_state = DVector::from_element(1, 100.0);
        let initial_covariance = DMatrix::from_element(1, 1, 1.0);

        let mut kf = KalmanFilter::new(initial_state, initial_covariance, model, 1.0);

        kf.predict();
        assert!((kf.state[0] - 10.0).abs() < 1e-6, "Prediction step failed");

        // Measurement z=100 (perfect measurement for x=10)
        let measurement = DVector::from_element(1, 100.0);
        kf.update(&measurement).unwrap();

        // Should stay close to 10
        assert!(
            (kf.state[0] - 10.0).abs() < 0.5,
            "Update step diverged: {}",
            kf.state[0]
        );
    }
}
