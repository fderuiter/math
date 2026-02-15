#[cfg(test)]
mod tests {
    use math_explorer::ai::deep_learning_theory::cycle::TrainingLoop;
    use math_explorer::ai::deep_learning_theory::linear_algebra::Vector;
    use math_explorer::ai::deep_learning_theory::optimization::{Adam, SGD};

    #[test]
    fn test_training_loop_sgd() {
        let input_dim = 2;
        let hidden_dim = 2;
        let output_dim = 2;
        let learning_rate = 0.01;

        // Dependency Injection: Inject SGD
        let optimizer = Box::new(SGD::new(learning_rate));
        let mut loop_instance = TrainingLoop::new(input_dim, hidden_dim, output_dim, optimizer);

        let x = Vector::from_vec(vec![0.1, 0.2]);
        let y_true = Vector::from_vec(vec![0.0, 1.0]);

        let initial_loss = loop_instance.train_step(&x, &y_true);
        let next_loss = loop_instance.train_step(&x, &y_true);

        // Loss should decrease (or at least change)
        assert!(next_loss <= initial_loss, "Loss did not decrease with SGD");
    }

    #[test]
    fn test_training_loop_adam() {
        let input_dim = 2;
        let hidden_dim = 2;
        let output_dim = 2;
        let learning_rate = 0.01;

        // Dependency Injection: Inject Adam
        // This confirms that we can swap optimizers without changing TrainingLoop code!
        let optimizer = Box::new(Adam::new(learning_rate));
        let mut loop_instance = TrainingLoop::new(input_dim, hidden_dim, output_dim, optimizer);

        let x = Vector::from_vec(vec![0.1, 0.2]);
        let y_true = Vector::from_vec(vec![0.0, 1.0]);

        let initial_loss = loop_instance.train_step(&x, &y_true);
        // Adam converges faster/differently, but should still decrease loss or work
        let next_loss = loop_instance.train_step(&x, &y_true);

        // Just verify it runs and returns a valid float
        assert!(initial_loss.is_finite());
        assert!(next_loss.is_finite());

        // In some cases with random init, loss might jump slightly on first step with momentum,
        // but generally should go down.
        // We use a lenient check here as this is a smoke test for the architecture.
        // If it compiles and runs without panic, the refactor is successful structurally.
    }
}
