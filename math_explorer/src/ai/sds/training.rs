use nalgebra::DMatrix;

/// 5.2 Optimizer Step (Adam)
/// Simplified Adam implementation for a single parameter tensor (e.g., NeRF weights).
/// theta_{t+1} = theta_t - eta * m_t / (sqrt(v_t) + epsilon)
pub struct AdamOptimizer {
    pub learning_rate: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub epsilon: f64,
    pub m: Option<DMatrix<f64>>,
    pub v: Option<DMatrix<f64>>,
    pub t: usize,
}

impl AdamOptimizer {
    pub fn new(learning_rate: f64) -> Self {
        Self {
            learning_rate,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            m: None,
            v: None,
            t: 0,
        }
    }

    pub fn step(&mut self, params: &DMatrix<f64>, grads: &DMatrix<f64>) -> DMatrix<f64> {
        self.t += 1;

        // Initialize state if needed
        if self.m.is_none() {
            self.m = Some(DMatrix::zeros(params.nrows(), params.ncols()));
            self.v = Some(DMatrix::zeros(params.nrows(), params.ncols()));
        }

        let m = self.m.as_mut().unwrap();
        let v = self.v.as_mut().unwrap();

        // Update biased first moment estimate: m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
        *m = &*m * self.beta1 + grads * (1.0 - self.beta1);

        // Update biased second raw moment estimate: v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
        // Element-wise square of gradients
        let grads_sq = grads.map(|x| x * x);
        *v = &*v * self.beta2 + grads_sq * (1.0 - self.beta2);

        // Compute bias-corrected first moment estimate
        let m_hat = &*m / (1.0 - self.beta1.powi(self.t as i32));

        // Compute bias-corrected second raw moment estimate
        let v_hat = &*v / (1.0 - self.beta2.powi(self.t as i32));

        // Update parameters: theta = theta - lr * m_hat / (sqrt(v_hat) + epsilon)
        let update_term = m_hat.component_div(&v_hat.map(|x| x.sqrt() + self.epsilon));

        params - update_term * self.learning_rate
    }
}

#[cfg(test)]
#[path = "tests_training.rs"]
mod tests;
