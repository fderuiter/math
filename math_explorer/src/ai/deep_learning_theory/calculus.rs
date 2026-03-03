use super::linear_algebra::{Matrix, Vector};
use std::f64::consts::E;

/// ReLU (Rectified Linear Unit) activation function: f(z) = max(0, z).
/// Applied element-wise.
pub fn relu(z: &Vector) -> Vector {
    z.map(|v| if v > 0.0 { v } else { 0.0 })
}

/// Derivative of ReLU.
/// f'(z) = 1 if z > 0, else 0.
pub fn relu_prime(z: &Vector) -> Vector {
    z.map(|v| if v > 0.0 { 1.0 } else { 0.0 })
}

/// Sigmoid activation function: \sigma(z) = 1 / (1 + e^{-z}).
pub fn sigmoid(z: &Vector) -> Vector {
    z.map(|v| 1.0 / (1.0 + E.powf(-v)))
}

/// Derivative of Sigmoid: \sigma'(z) = \sigma(z) * (1 - \sigma(z)).
pub fn sigmoid_prime(z: &Vector) -> Vector {
    let s = sigmoid(z);
    s.map(|v| v * (1.0 - v))
}

/// Computes the gradients for a linear layer during backpropagation.
///
/// Given the gradient of the loss with respect to the output z (\frac{\partial L}{\partial z}),
/// compute the gradients with respect to inputs x, weights W, and bias b.
///
/// Returns tuple: (grad_x, grad_W, grad_b)
///
/// Derivations:
/// z = Wx + b
/// \frac{\partial L}{\partial x} = W^T \cdot \frac{\partial L}{\partial z}
/// \frac{\partial L}{\partial W} = \frac{\partial L}{\partial z} \cdot x^T
/// \frac{\partial L}{\partial b} = \frac{\partial L}{\partial z}
pub fn linear_backward(grad_z: &Vector, x: &Vector, w: &Matrix) -> (Vector, Matrix, Vector) {
    // grad_z is (output_dim)
    // x is (input_dim)
    // W is (output_dim, input_dim)

    // dL/dx = W^T * dL/dz
    let grad_x = w.transpose() * grad_z;

    // dL/dW = dL/dz * x^T (outer product)
    // In nalgebra, we can do this via gemm or explicit multiplication
    // grad_z is (out, 1), x is (in, 1). We want (out, in).
    // grad_z * x^T
    let grad_w = grad_z * x.transpose();

    // dL/db = dL/dz
    let grad_b = grad_z.clone();

    (grad_x, grad_w, grad_b)
}

/// Applies the chain rule for element-wise activation functions.
///
/// Given \frac{\partial L}{\partial a} (gradient w.r.t activation output),
/// and the pre-activation input z, computes \frac{\partial L}{\partial z}.
///
/// \frac{\partial L}{\partial z} = \frac{\partial L}{\partial a} \odot f'(z)
pub fn activation_backward<F>(grad_a: &Vector, z: &Vector, prime_fn: F) -> Vector
where
    F: Fn(&Vector) -> Vector,
{
    let dz = prime_fn(z);
    grad_a.component_mul(&dz) // Element-wise multiplication (Hadamard product)
}

/// Tanh activation function: f(z) = tanh(z).
pub fn tanh(z: &Vector) -> Vector {
    z.map(|v| v.tanh())
}

/// Derivative of Tanh: f'(z) = 1 - tanh^2(z).
pub fn tanh_prime(z: &Vector) -> Vector {
    let t = tanh(z);
    t.map(|v| 1.0 - v * v)
}

/// GELU (Gaussian Error Linear Unit) activation function: f(z) = 0.5 * z * (1 + tanh(sqrt(2/pi) * (z + 0.044715 * z^3))).
/// Using the approximate formulation.
pub fn gelu(z: &Vector) -> Vector {
    let sqrt_2_over_pi = (2.0f64 / std::f64::consts::PI).sqrt();
    z.map(|v| 0.5 * v * (1.0 + (sqrt_2_over_pi * (v + 0.044715 * v.powi(3))).tanh()))
}

/// Derivative of GELU (approximate).
pub fn gelu_prime(z: &Vector) -> Vector {
    let sqrt_2_over_pi = (2.0f64 / std::f64::consts::PI).sqrt();
    z.map(|v| {
        let x = sqrt_2_over_pi * (v + 0.044715 * v.powi(3));
        let tanh_x = x.tanh();
        let sech2_x = 1.0 - tanh_x * tanh_x;
        0.5 * (1.0 + tanh_x)
            + 0.5 * v * sech2_x * sqrt_2_over_pi * (1.0 + 3.0 * 0.044715 * v.powi(2))
    })
}
