use math_explorer::physics::medical::radar_gating::tracking::TrackingFilter;
use math_explorer::applied::algorithms::kalman::{KalmanFilter, KalmanModel};
use nalgebra::{DMatrix, DVector};
use approx::assert_relative_eq;

/// Constant Acceleration Model (Pos, Vel, Acc)
struct ConstantAccelerationModel {
    noise: f64,
}

impl KalmanModel for ConstantAccelerationModel {
    fn transition_matrix(&self, dt: f64) -> DMatrix<f64> {
        // [1, dt, 0.5*dt^2]
        // [0, 1,  dt      ]
        // [0, 0,  1       ]
        DMatrix::from_row_slice(3, 3, &[
            1.0, dt, 0.5 * dt * dt,
            0.0, 1.0, dt,
            0.0, 0.0, 1.0
        ])
    }

    fn measurement_matrix(&self) -> DMatrix<f64> {
        // [1, 0, 0]
        DMatrix::from_row_slice(1, 3, &[1.0, 0.0, 0.0])
    }

    fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
        DMatrix::identity(3, 3) * self.noise
    }

    fn measurement_noise(&self) -> DMatrix<f64> {
        DMatrix::from_element(1, 1, self.noise)
    }
}

#[test]
fn test_tracking_filter_with_constant_acceleration() {
    let dt = 0.1;
    let model = ConstantAccelerationModel { noise: 0.01 };

    // Initial State: Pos=0, Vel=10, Acc=1
    let initial_state = DVector::from_vec(vec![0.0, 10.0, 1.0]);
    let initial_cov = DMatrix::identity(3, 3);

    // Create Generic Kalman Filter
    let kf = KalmanFilter::new(initial_state, initial_cov, model, dt);

    // Inject into TrackingFilter using new_from_filter
    let mut tracker = TrackingFilter::new_from_filter(kf);

    // Predict 1 step
    // Pos = 0 + 10*0.1 + 0.5*1*0.01 = 1 + 0.005 = 1.005
    // Vel = 10 + 1*0.1 = 10.1
    // Acc = 1
    tracker.predict();

    assert_relative_eq!(tracker.amplitude(), 1.005, epsilon=1e-6);
    assert_relative_eq!(tracker.velocity(), 10.1, epsilon=1e-6);

    // Test update
    // Measure 1.1 (slightly off)
    tracker.update(1.1);

    // Just verify it doesn't panic and updates state
    let new_pos = tracker.amplitude();
    assert!(new_pos > 1.005 && new_pos < 1.15); // pulled towards measurement
}
