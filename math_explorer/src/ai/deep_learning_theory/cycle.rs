use super::calculus::{linear_backward, relu, relu_prime};
use super::linear_algebra::{DenseLayer, Vector};
use super::optimization::{Optimizer, ParamType, cross_entropy_softmax_prime};
use super::probability::softmax;

/// The Deep Learning Cycle: Forward -> Loss -> Backward -> Update
///
/// This struct demonstrates a simple neural network training loop.
/// Architecture: Input -> Dense -> ReLU -> Dense -> Softmax -> Output
pub struct TrainingLoop {
    pub layer1: DenseLayer,
    pub layer2: DenseLayer,
    pub optimizer: Box<dyn Optimizer>,
}

impl TrainingLoop {
    pub fn new(
        input_dim: usize,
        hidden_dim: usize,
        output_dim: usize,
        optimizer: Box<dyn Optimizer>,
    ) -> Self {
        Self {
            layer1: DenseLayer::new(input_dim, hidden_dim),
            layer2: DenseLayer::new(hidden_dim, output_dim),
            optimizer,
        }
    }

    /// Performs one iteration of training (Forward, Backward, Update).
    ///
    /// # Arguments
    /// * `x` - Input vector.
    /// * `y_true` - One-hot encoded target vector.
    ///
    /// # Returns
    /// The scalar loss value.
    pub fn train_step(&mut self, x: &Vector, y_true: &Vector) -> f64 {
        // --- 1. Forward Pass (Linear Algebra) ---
        // z1 = W1*x + b1
        let z1 = self.layer1.forward(x);
        // a1 = ReLU(z1)
        let a1 = relu(&z1);
        // z2 = W2*a1 + b2
        let z2 = self.layer2.forward(&a1);
        // a2 = Softmax(z2) (Prediction)
        let y_pred = softmax(&z2);

        // --- 2. Loss Calculation (Probability & Statistics) ---
        // Using Cross-Entropy. We calculate it just for reporting,
        // but for gradients we use the simplified analytical form combined with Softmax.
        let epsilon = 1e-15;
        let loss = -(y_true.dot(&y_pred.map(|v| (v + epsilon).ln())));

        // --- 3. Backward Pass (Calculus) ---
        // Gradient of Loss w.r.t z2 (logits)
        // dL/dz2 = y_pred - y_true
        let d_z2 = cross_entropy_softmax_prime(&z2, y_true);

        // Backprop through Layer 2
        // dL/dW2, dL/db2, dL/da1
        let (d_a1, d_w2, d_b2) = linear_backward(&d_z2, &a1, &self.layer2.weights);

        // Backprop through ReLU
        // dL/dz1 = dL/da1 * ReLU'(z1)
        let d_z1 = d_a1.component_mul(&relu_prime(&z1));

        // Backprop through Layer 1
        // dL/dW1, dL/db1, dL/dx
        let (_, d_w1, d_b1) = linear_backward(&d_z1, x, &self.layer1.weights);

        // --- 4. Update (Optimization) ---
        // Using layer indices to allow stateful optimizers (like Adam) to track momentum correctly.

        // Layer 2 update
        self.optimizer
            .update_matrix(1, ParamType::Weight, &mut self.layer2.weights, &d_w2);
        self.optimizer
            .update_vector(1, ParamType::Bias, &mut self.layer2.bias, &d_b2);

        // Layer 1 update
        self.optimizer
            .update_matrix(0, ParamType::Weight, &mut self.layer1.weights, &d_w1);
        self.optimizer
            .update_vector(0, ParamType::Bias, &mut self.layer1.bias, &d_b1);

        loss
    }

    /// Predicts the class for a given input.
    pub fn predict(&self, x: &Vector) -> Vector {
        let z1 = self.layer1.forward(x);
        let a1 = relu(&z1);
        let z2 = self.layer2.forward(&a1);
        softmax(&z2)
    }
}
