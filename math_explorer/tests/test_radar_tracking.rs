use math_explorer::physics::medical::radar_gating::tracking::{KalmanFilter, KalmanModel, ConstantVelocityModel, TrackingFilter};
use nalgebra::{Matrix3, Vector3, Matrix1x3, Matrix1, Vector1, Const};
use approx::assert_relative_eq;

// --- Test 1: Verify the Wrapper (Legacy API) ---
#[test]
fn test_legacy_tracking_filter() {
    let mut filter = TrackingFilter::new(0.0, 0.1, 0.01, 0.1);

    // Initial State
    assert_eq!(filter.amplitude(), 0.0);
    assert_eq!(filter.velocity(), 0.0);

    // Predict & Update cycle
    filter.predict();
    filter.update(1.0);

    let amp = filter.amplitude();
    // After one update with measurement 1.0, estimate should move towards 1.0
    assert!(amp > 0.0 && amp < 1.0);
}

// --- Test 2: Verify Generic Capability (Constant Acceleration) ---

// Define a new model: Constant Acceleration
// State: [Position, Velocity, Acceleration]
struct ConstantAccelerationModel {
    dt: f64,
    process_noise: f64,
    measurement_noise: f64,
}

impl KalmanModel<f64, Const<3>, Const<1>> for ConstantAccelerationModel {
    fn f(&self, _dt: f64) -> Matrix3<f64> {
        let t = self.dt;
        let t2 = 0.5 * t * t;
        Matrix3::new(
            1.0, t, t2,
            0.0, 1.0, t,
            0.0, 0.0, 1.0
        )
    }

    fn q(&self, _dt: f64) -> Matrix3<f64> {
        Matrix3::identity() * self.process_noise
    }

    fn h(&self) -> Matrix1x3<f64> {
        Matrix1x3::new(1.0, 0.0, 0.0)
    }

    fn r(&self) -> Matrix1<f64> {
        Matrix1::new(self.measurement_noise)
    }
}

#[test]
fn test_generic_extension_constant_acceleration() {
    let model = ConstantAccelerationModel {
        dt: 0.1,
        process_noise: 0.001,
        measurement_noise: 0.1,
    };

    let initial_state = Vector3::new(0.0, 1.0, 0.1); // Initial velocity 1.0, Accel 0.1
    let initial_cov = Matrix3::identity();

    let mut kf = KalmanFilter::new(initial_state, initial_cov, model);

    // Predict step
    kf.predict(0.1);

    // Expected position after 0.1s: p + v*t + 0.5*a*t^2
    // 0 + 1*0.1 + 0.5*0.1*0.01 = 0.1 + 0.0005 = 0.1005
    let predicted_pos = kf.state()[0];
    assert_relative_eq!(predicted_pos, 0.1005, epsilon = 1e-6);

    // Update step
    let measurement = Vector1::new(0.12); // Measured slightly ahead
    kf.update(measurement);

    // State should adjust
    let new_pos = kf.state()[0];
    assert!(new_pos > 0.1005); // Should be pulled towards measurement
}
