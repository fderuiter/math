use super::calculus::{linear_backward, relu, relu_prime};
use super::linear_algebra::{DenseLayer, Vector};
use crate::ai::optimization::Optimizer;

/// Defines a Trainable Model that can perform forward passes and handle backpropagation.
pub trait Trainable {
    /// Performs a forward pass through the network.
    /// Returns the logits (pre-softmax outputs).
    fn forward(&self, x: &Vector) -> Vector;

    /// Performs the backward pass and updates parameters.
    ///
    /// # Arguments
    /// * `x` - Input vector.
    /// * `loss_grad` - Gradient of the loss with respect to the output logits (dJ/dz).
    /// * `optimizer` - The optimizer strategy to update parameters.
    fn backward_update(&mut self, x: &Vector, loss_grad: &Vector, optimizer: &mut dyn Optimizer<f64>);
}

/// A standard Two-Layer Multi-Layer Perceptron (MLP).
/// Architecture: Input -> Dense -> ReLU -> Dense -> Softmax (implicit in loss)
pub struct TwoLayerMLP {
    pub layer1: DenseLayer,
    pub layer2: DenseLayer,
}

impl TwoLayerMLP {
    pub fn new(input_dim: usize, hidden_dim: usize, output_dim: usize) -> Self {
        Self {
            layer1: DenseLayer::new(input_dim, hidden_dim),
            layer2: DenseLayer::new(hidden_dim, output_dim),
        }
    }
}

impl Trainable for TwoLayerMLP {
    fn forward(&self, x: &Vector) -> Vector {
        let z1 = self.layer1.forward(x);
        let a1 = relu(&z1);
        let z2 = self.layer2.forward(&a1);
        z2
    }

    fn backward_update(&mut self, x: &Vector, loss_grad: &Vector, optimizer: &mut dyn Optimizer<f64>) {
        // --- Forward Re-computation (needed for gradients) ---
        // Ideally, we'd cache these, but re-computing is simpler for this educational example.
        let z1 = self.layer1.forward(x);
        let a1 = relu(&z1);
        // z2 is not needed explicitly since we have loss_grad (dL/dz2) passed in.

        // --- Backward Pass ---
        // Backprop through Layer 2
        // dL/dW2, dL/db2, dL/da1
        // loss_grad corresponds to d_z2
        let (d_a1, d_w2, d_b2) = linear_backward(loss_grad, &a1, &self.layer2.weights);

        // Backprop through ReLU
        // dL/dz1 = dL/da1 * ReLU'(z1)
        let d_z1 = d_a1.component_mul(&relu_prime(&z1));

        // Backprop through Layer 1
        // dL/dW1, dL/db1, dL/dx
        let (_, d_w1, d_b1) = linear_backward(&d_z1, x, &self.layer1.weights);

        // --- Update ---
        // Layer 2
        optimizer.update_matrix(2, &mut self.layer2.weights, &d_w2);
        optimizer.update_vector(2, &mut self.layer2.bias, &d_b2);

        // Layer 1
        optimizer.update_matrix(1, &mut self.layer1.weights, &d_w1);
        optimizer.update_vector(1, &mut self.layer1.bias, &d_b1);
    }
}
