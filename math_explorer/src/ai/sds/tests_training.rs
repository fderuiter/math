use crate::ai::sds::training::{AdamOptimizer, Optimizer, SgdOptimizer};
use approx::assert_relative_eq;
use nalgebra::DMatrix;

#[test]
fn test_adam_step() {
    let rows = 2;
    let cols = 2;
    let mut optimizer = AdamOptimizer::new(0.1);

    let params = DMatrix::from_element(rows, cols, 0.5);
    let grads = DMatrix::from_element(rows, cols, 0.1);

    // First step
    // m = 0.1 * (1-0.9) = 0.01
    // v = 0.01 * (1-0.999) = 0.00001
    // m_hat = 0.01 / (1-0.9) = 0.1
    // v_hat = 0.00001 / (1-0.999) = 0.01
    // update = 0.1 * 0.1 / (sqrt(0.01) + eps) = 0.01 / 0.1 = 0.1
    // new_params = 0.5 - 0.1 = 0.4

    let new_params_result = optimizer.step(&params, &grads);
    assert!(new_params_result.is_ok());
    let new_params = new_params_result.unwrap();

    // Exact values might differ slightly due to float precision and epsilon
    assert!(new_params[(0, 0)] < 0.5);
    assert_relative_eq!(new_params[(0, 0)], 0.4, epsilon = 1e-4);
}

#[test]
fn test_sgd_step() {
    let rows = 2;
    let cols = 2;
    // SGD with momentum 0.9, lr 0.1
    let mut optimizer = SgdOptimizer::new(0.1, 0.9);

    let params = DMatrix::from_element(rows, cols, 0.5);
    let grads = DMatrix::from_element(rows, cols, 0.1);

    // First step
    // v = 0.9 * 0 + 0.1 = 0.1
    // theta = 0.5 - 0.1 * 0.1 = 0.49

    let new_params_result = optimizer.step(&params, &grads);
    assert!(new_params_result.is_ok());
    let new_params = new_params_result.unwrap();

    assert_relative_eq!(new_params[(0, 0)], 0.49, epsilon = 1e-6);

    // Second step
    // grads = 0.1
    // v = 0.9 * 0.1 + 0.1 = 0.19
    // theta = 0.49 - 0.1 * 0.19 = 0.49 - 0.019 = 0.471

    let new_params_result_2 = optimizer.step(&new_params, &grads);
    assert!(new_params_result_2.is_ok());
    let new_params_2 = new_params_result_2.unwrap();

    assert_relative_eq!(new_params_2[(0, 0)], 0.471, epsilon = 1e-6);
}
