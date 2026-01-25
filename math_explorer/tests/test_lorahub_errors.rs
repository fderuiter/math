use math_explorer::applied::lorahub::{LoraEnsemble, LoraError, LoraStateDict};
use nalgebra::DMatrix;

#[test]
fn test_empty_ensemble_error() {
    let ensemble = LoraEnsemble::new(vec![]);
    let result = ensemble.combine(&[1.0]);
    assert_eq!(result.unwrap_err(), LoraError::EmptyEnsemble);
}

#[test]
fn test_empty_weights_error() {
    let lora = LoraStateDict::new();
    let ensemble = LoraEnsemble::new(vec![lora]);
    let result = ensemble.combine(&[]);
    assert_eq!(result.unwrap_err(), LoraError::EmptyWeights);
}

#[test]
fn test_weight_count_mismatch() {
    let lora = LoraStateDict::new();
    let ensemble = LoraEnsemble::new(vec![lora]);
    let result = ensemble.combine(&[1.0, 2.0]);
    match result.unwrap_err() {
        LoraError::WeightCountMismatch { weights, modules } => {
            assert_eq!(weights, 2);
            assert_eq!(modules, 1);
        }
        _ => panic!("Expected WeightCountMismatch"),
    }
}

#[test]
fn test_key_mismatch() {
    let mut lora1 = LoraStateDict::new();
    lora1.insert("A".to_string(), DMatrix::from_element(1, 1, 1.0));

    let mut lora2 = LoraStateDict::new();
    lora2.insert("B".to_string(), DMatrix::from_element(1, 1, 1.0));

    let ensemble = LoraEnsemble::new(vec![lora1, lora2]);
    let result = ensemble.combine(&[0.5, 0.5]);

    // Logic: iterates over lora1 keys. "A" is not in lora2.
    match result.unwrap_err() {
        LoraError::KeyMismatch { key } => assert_eq!(key, "A"),
        _ => panic!("Expected KeyMismatch"),
    }
}

#[test]
fn test_shape_mismatch() {
    let mut lora1 = LoraStateDict::new();
    lora1.insert("A".to_string(), DMatrix::from_element(1, 1, 1.0));

    let mut lora2 = LoraStateDict::new();
    lora2.insert("A".to_string(), DMatrix::from_element(2, 2, 1.0));

    let ensemble = LoraEnsemble::new(vec![lora1, lora2]);
    let result = ensemble.combine(&[0.5, 0.5]);

    match result.unwrap_err() {
        LoraError::ShapeMismatch { key, expected, actual } => {
            assert_eq!(key, "A");
            assert_eq!(expected, (1, 1));
            assert_eq!(actual, (2, 2));
        }
        _ => panic!("Expected ShapeMismatch"),
    }
}
