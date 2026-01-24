//! Optimizers for training AI models.
//!
//! This module defines the `Optimizer` trait and implements common optimization algorithms
//! like SGD and Adam, supporting both `f32` and `f64`.

use crate::ai::AIError;
use nalgebra::{DMatrix, RealField};

/// A trait for optimization algorithms.
///
/// Optimizers update model parameters based on computed gradients.
/// The interface operates on mutable slices, allowing it to work with
/// any contiguous tensor storage (e.g., `DMatrix`, `DVector`, `Vec`).
pub trait Optimizer<T: RealField + Copy> {
    /// Updates the parameters in-place using the given gradients.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters to update (as a mutable slice).
    /// * `grads` - The gradients with respect to the parameters (as a slice).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the update was successful.
    /// * `Err(AIError)` if there was a length mismatch.
    fn update(&mut self, params: &mut [T], grads: &[T]) -> Result<(), AIError>;

    /// Performs a single optimization step and returns the updated parameters.
    /// This is a convenience wrapper that clones the parameters.
    /// Note: This assumes the input is a DMatrix for convenience in existing code.
    fn step(&mut self, params: &DMatrix<T>, grads: &DMatrix<T>) -> Result<DMatrix<T>, AIError> {
        let mut new_params = params.clone();
        self.update(new_params.as_mut_slice(), grads.as_slice())?;
        Ok(new_params)
    }
}

/// Stochastic Gradient Descent (SGD) optimizer.
///
/// Updates parameters using the rule: `params = params - learning_rate * grads`.
pub struct SgdOptimizer<T: RealField + Copy> {
    /// The learning rate.
    pub learning_rate: T,
}

impl<T: RealField + Copy> SgdOptimizer<T> {
    /// Creates a new SGD optimizer.
    pub fn new(learning_rate: T) -> Self {
        Self { learning_rate }
    }
}

impl<T: RealField + Copy> Optimizer<T> for SgdOptimizer<T> {
    fn update(&mut self, params: &mut [T], grads: &[T]) -> Result<(), AIError> {
        if params.len() != grads.len() {
            return Err(AIError::DimensionMismatch {
                expected: format!("len {}", params.len()),
                got: format!("len {}", grads.len()),
            });
        }

        for (p, g) in params.iter_mut().zip(grads.iter()) {
            *p -= *g * self.learning_rate;
        }
        Ok(())
    }
}

/// Adam optimizer (Adaptive Moment Estimation).
///
/// Computes individual adaptive learning rates for different parameters
/// from estimates of first and second moments of the gradients.
pub struct AdamOptimizer<T: RealField + Copy> {
    pub learning_rate: T,
    pub beta1: T,
    pub beta2: T,
    pub epsilon: T,
    pub m: Option<Vec<T>>,
    pub v: Option<Vec<T>>,
    pub t: usize,
}

impl<T: RealField + Copy> AdamOptimizer<T> {
    /// Creates a new Adam optimizer with default betas (0.9, 0.999) and epsilon (1e-8).
    pub fn new(learning_rate: T) -> Self {
        Self {
            learning_rate,
            beta1: T::from_f64(0.9).unwrap(),
            beta2: T::from_f64(0.999).unwrap(),
            epsilon: T::from_f64(1e-8).unwrap(),
            m: None,
            v: None,
            t: 0,
        }
    }
}

impl<T: RealField + Copy> Optimizer<T> for AdamOptimizer<T> {
    fn update(&mut self, params: &mut [T], grads: &[T]) -> Result<(), AIError> {
        if params.len() != grads.len() {
            return Err(AIError::DimensionMismatch {
                expected: format!("len {}", params.len()),
                got: format!("len {}", grads.len()),
            });
        }

        self.t += 1;

        // Initialize state if needed
        if self.m.is_none() {
            self.m = Some(vec![T::zero(); params.len()]);
            self.v = Some(vec![T::zero(); params.len()]);
        }

        let m = self
            .m
            .as_mut()
            .expect("Optimizer state m should be initialized");
        let v = self
            .v
            .as_mut()
            .expect("Optimizer state v should be initialized");

        if m.len() != params.len() {
             return Err(AIError::DimensionMismatch {
                expected: format!("state len {}", m.len()),
                got: format!("params len {}", params.len()),
            });
        }

        let one = T::one();
        let t_i32 = self.t as i32;
        let beta1_t = self.beta1.powi(t_i32);
        let beta2_t = self.beta2.powi(t_i32);

        // Loop over elements
        for i in 0..params.len() {
            let g = grads[i];

            // Update m
            m[i] = m[i] * self.beta1 + g * (one - self.beta1);

            // Update v
            v[i] = v[i] * self.beta2 + g * g * (one - self.beta2);

            // Bias correction
            let m_hat = m[i] / (one - beta1_t);
            let v_hat = v[i] / (one - beta2_t);

            // Update param
            params[i] -= (m_hat / (v_hat.sqrt() + self.epsilon)) * self.learning_rate;
        }

        Ok(())
    }
}
