use math_explorer::ai::deep_learning_theory::cycle::TrainingLoop;
use math_explorer::ai::deep_learning_theory::linear_algebra::Vector;
use math_explorer::ai::deep_learning_theory::optimization::Adam;

#[test]
fn test_training_loop_sgd() {
    let input_dim = 2;
    let hidden_dim = 3;
    let output_dim = 2;
    let lr = 0.01;

    let mut loop_instance = TrainingLoop::new(input_dim, hidden_dim, output_dim, lr);

    // Dummy data
    let x = Vector::from_vec(vec![0.5, -0.5]);
    let y_true = Vector::from_vec(vec![1.0, 0.0]); // Class 0

    let initial_loss = loop_instance.train_step(&x, &y_true);

    // Run a few steps
    for _ in 0..10 {
        loop_instance.train_step(&x, &y_true);
    }

    let final_loss = loop_instance.train_step(&x, &y_true);

    // Loss should decrease
    assert!(final_loss < initial_loss, "Loss did not decrease with SGD");
}

#[test]
fn test_training_loop_adam() {
    let input_dim = 2;
    let hidden_dim = 3;
    let output_dim = 2;
    let lr = 0.01;

    let adam = Box::new(Adam::new(lr));
    let mut loop_instance = TrainingLoop::with_optimizer(input_dim, hidden_dim, output_dim, adam);

    // Dummy data
    let x = Vector::from_vec(vec![0.5, -0.5]);
    let y_true = Vector::from_vec(vec![1.0, 0.0]); // Class 0

    let initial_loss = loop_instance.train_step(&x, &y_true);

    // Run a few steps
    for _ in 0..10 {
        loop_instance.train_step(&x, &y_true);
    }

    let final_loss = loop_instance.train_step(&x, &y_true);

    // Loss should decrease
    assert!(final_loss < initial_loss, "Loss did not decrease with Adam");
}
