//! Kinetics strategy for ion channel gating variables.

/// Defines the voltage-dependent rate constants for gating variables.
///
/// Implement this trait to define custom ion channel dynamics (e.g., for different neuron types).
pub trait GatingKinetics: Send + Sync + std::fmt::Debug {
    /// Rate constant $\alpha_n$ for Potassium activation.
    fn alpha_n(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_n$ for Potassium activation.
    fn beta_n(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\alpha_m$ for Sodium activation.
    fn alpha_m(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_m$ for Sodium activation.
    fn beta_m(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\alpha_h$ for Sodium inactivation.
    fn alpha_h(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_h$ for Sodium inactivation.
    fn beta_h(&self, v: f64, v_rest: f64) -> f64;
}

/// Standard Hodgkin-Huxley kinetics (Squid Giant Axon).
///
/// This implementation uses the original equations from Hodgkin & Huxley (1952).
/// It handles numerical singularities where the denominator approaches zero using
/// L'Hôpital's rule approximations.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardKinetics;

impl GatingKinetics for StandardKinetics {
    /// Potassium activation rate $\alpha_n$.
    ///
    /// $$ \alpha_n = \frac{0.01 (10 - (V - V_{rest}))}{\exp(0.1(10 - (V - V_{rest}))) - 1} $$
    ///
    /// # Numerical Stability
    /// When $V = V_{rest} + 10$, the denominator approaches zero ($e^0 - 1 = 0$).
    /// We use L'Hôpital's rule to evaluate the limit:
    /// $$ \lim_{x \to 0} \frac{0.01 x}{e^{0.1 x} - 1} = \frac{0.01}{0.1} = 0.1 $$
    fn alpha_n(&self, v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_n(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    /// Sodium activation rate $\alpha_m$.
    ///
    /// $$ \alpha_m = \frac{0.1 (25 - (V - V_{rest}))}{\exp(0.1(25 - (V - V_{rest}))) - 1} $$
    ///
    /// # Numerical Stability
    /// When $V = V_{rest} + 25$, the denominator approaches zero.
    /// We use L'Hôpital's rule to evaluate the limit:
    /// $$ \lim_{x \to 0} \frac{0.1 x}{e^{0.1 x} - 1} = \frac{0.1}{0.1} = 1.0 $$
    fn alpha_m(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_m(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    fn alpha_h(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn beta_h(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_n_singularity() {
        let kinetics = StandardKinetics;
        let v_rest = -65.0;
        // Singularity point: v = v_rest + 10.0
        let v_singular = v_rest + 10.0;
        let result = kinetics.alpha_n(v_singular, v_rest);

        // Expected limit: 0.1
        assert!((result - 0.1).abs() < 1e-9, "Expected 0.1, got {}", result);
        assert!(result.is_finite(), "Result should be finite");
    }

    #[test]
    fn test_alpha_m_singularity() {
        let kinetics = StandardKinetics;
        let v_rest = -65.0;
        // Singularity point: v = v_rest + 25.0
        let v_singular = v_rest + 25.0;
        let result = kinetics.alpha_m(v_singular, v_rest);

        // Expected limit: 1.0
        assert!((result - 1.0).abs() < 1e-9, "Expected 1.0, got {}", result);
        assert!(result.is_finite(), "Result should be finite");
    }
}
