use math_explorer::applied::algorithms::kalman::KalmanModel;
use math_explorer::applied::algorithms::{AlgorithmError, kalman::KalmanFilter};
use math_explorer::applied::favoritism::{
    FavoritismError, FavoritismInputs, try_calculate_favoritism_score,
};
use math_explorer::applied::lorahub::{LoraEnsemble, LoraHubError, LoraStateDict};
use nalgebra::{DMatrix, DVector};

#[test]
fn test_favoritism_validation_negative_time() {
    let mut inputs = FavoritismInputs::default();
    inputs.time.t = -1.0;
    let result = inputs.validate();
    assert!(matches!(result, Err(FavoritismError::InvalidInput(_))));

    let result = try_calculate_favoritism_score(&inputs);
    assert!(matches!(result, Err(FavoritismError::InvalidInput(_))));
}

#[test]
fn test_favoritism_validation_negative_sibling_distance() {
    let mut inputs = FavoritismInputs::default();
    inputs.family.sibling_distances = vec![100.0, -10.0];
    let result = inputs.validate();
    assert!(matches!(result, Err(FavoritismError::InvalidInput(_))));
}

#[test]
fn test_lorahub_empty_ensemble_error() {
    let modules = vec![];
    let ensemble = LoraEnsemble::new(modules);
    let weights = vec![0.5];
    let result = ensemble.combine(&weights);
    assert_eq!(result.unwrap_err(), LoraHubError::EmptyEnsemble);
}

#[test]
fn test_lorahub_length_mismatch_error() {
    let mut lora_1 = LoraStateDict::new();
    lora_1.insert("layer1".to_string(), DMatrix::from_element(1, 1, 1.0));
    let modules = vec![lora_1];
    let ensemble = LoraEnsemble::new(modules);
    let weights = vec![0.5, 0.5]; // Mismatch
    let result = ensemble.combine(&weights);
    assert_eq!(result.unwrap_err(), LoraHubError::LengthMismatch);
}

// Mock Model for testing Kalman error
struct MockModel;
impl KalmanModel for MockModel {
    fn transition_matrix(&self, _dt: f64) -> DMatrix<f64> {
        DMatrix::identity(1, 1)
    }
    fn measurement_matrix(&self) -> DMatrix<f64> {
        DMatrix::identity(1, 1)
    }
    fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
        DMatrix::identity(1, 1)
    }
    fn measurement_noise(&self) -> DMatrix<f64> {
        DMatrix::zeros(1, 1)
    } // Zero noise might cause singularity if covariance is also zero/singular
}

#[test]
fn test_kalman_singular_error() {
    let model = MockModel;
    let x_init = DVector::from_element(1, 0.0);
    // Zero covariance and zero measurement noise -> Innovation Covariance S = HPH^T + R = 0.
    // Inverting 0 will fail.
    let p_init = DMatrix::zeros(1, 1);

    let mut kf = KalmanFilter::new(x_init, p_init, model, 1.0);
    let measurement = DVector::from_element(1, 1.0);

    let result = kf.update(&measurement);
    assert_eq!(result.unwrap_err(), AlgorithmError::SingularMatrix);
}
