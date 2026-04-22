#![allow(warnings)]
use math_explorer::ai::deep_learning_theory::cycle::TrainingLoop;
use math_explorer::ai::deep_learning_theory::model::TwoLayerMLP;
use math_explorer::ai::optimization::SGD;
use nalgebra::DVector;

#[test]
fn test_deep_learning_cycle() {
    // Xor problem-ish (non-linear)
    // 2 inputs, 2 hidden units, 2 output classes
    // Use the new Strategy Pattern with SGD
    let mut network = TrainingLoop::new(2, 4, 2, Box::new(SGD::new(0.1)));

    // Dummy data: Input [1, 0] -> Class 0 ([1, 0])
    let x = DVector::from_vec(vec![1.0, 0.0]);
    let y_true = DVector::from_vec(vec![1.0, 0.0]);

    // Initial prediction should be close to uniform or random
    let _initial_pred = network.predict(&x);
    let initial_loss = network.train_step(&x, &y_true).unwrap();

    // Train for a few steps
    let mut final_loss = 0.0;
    for _ in 0..100 {
        final_loss = network.train_step(&x, &y_true).unwrap();
    }

    // Prediction should improve (loss should decrease)
    println!("Initial Loss: {}, Final Loss: {}", initial_loss, final_loss);
    assert!(final_loss < initial_loss, "Loss did not decrease!");

    let final_pred = network.predict(&x);
    // Probability of class 0 should be high
    assert!(
        final_pred[0] > final_pred[1],
        "Did not learn to predict class 0"
    );
    assert!(final_pred[0] > 0.8, "Prediction confidence too low");
}

#[test]
fn test_explicit_model_construction() {
    // Explicitly construct TwoLayerMLP and inject it
    let model = TwoLayerMLP::new(2, 4, 2);
    let network = TrainingLoop::new_with_model(model, Box::new(SGD::new(0.1)));

    let x = DVector::from_vec(vec![1.0, 0.0]);
    let pred = network.predict(&x);
    assert_eq!(pred.len(), 2);
}

#[test]
fn test_backward_compatibility_layer_access() {
    // Verify we can still access layer1/layer2 like before (via Deref)
    let network = TrainingLoop::new(2, 4, 2, Box::new(SGD::new(0.1)));

    // This access works because of Deref<Target=TwoLayerMLP>
    let _w1 = &network.layer1.weights;
    let _b2 = &network.layer2.bias;

    assert_eq!(network.layer1.weights.ncols(), 2);
}

#[test]
fn test_linear_algebra_basics() {
    use math_explorer::ai::deep_learning_theory::linear_algebra::*;

    let v1 = DVector::from_vec(vec![1.0, 2.0, 3.0]);
    let v2 = DVector::from_vec(vec![4.0, 5.0, 6.0]);

    // Dot product: 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    assert_eq!(dot_product(&v1, &v2), 32.0);
}
