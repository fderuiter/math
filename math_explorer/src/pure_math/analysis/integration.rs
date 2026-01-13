//! Numerical Integration strategies.
//!
//! This module provides a common interface for numerical integration (quadrature)
//! and concrete implementations of various algorithms.

/// Result of an integration operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegrationResult {
    /// The estimated value of the integral.
    pub value: f64,
    /// The estimated absolute error.
    pub error: f64,
}

/// A trait for numerical integration strategies.
///
/// Computes the definite integral:
/// $$ \int_a^b f(x) \, dx $$
pub trait Integrator {
    /// Integrates the function `f` over the interval `[a, b]`.
    ///
    /// # Arguments
    /// * `f` - The integrand function.
    /// * `a` - Lower bound.
    /// * `b` - Upper bound.
    /// * `tol` - Desired tolerance (absolute error).
    fn integrate<F>(&self, f: F, a: f64, b: f64, tol: f64) -> IntegrationResult
    where
        F: Fn(f64) -> f64;
}

/// Adaptive Clenshaw-Curtis Quadrature.
///
/// Wraps the `quadrature` crate's implementation.
/// This method is generally very efficient for smooth functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClenshawCurtis;

impl Integrator for ClenshawCurtis {
    fn integrate<F>(&self, f: F, a: f64, b: f64, tol: f64) -> IntegrationResult
    where
        F: Fn(f64) -> f64,
    {
        // The quadrature crate uses f64 for tolerance.
        let result = quadrature::clenshaw_curtis::integrate(f, a, b, tol);
        IntegrationResult {
            value: result.integral,
            error: result.error_estimate,
        }
    }
}

/// Adaptive Trapezoidal Rule.
///
/// A robust, dependency-free method using adaptive subdivision.
/// Useful as a fallback or for functions where Clenshaw-Curtis might fail or be overkill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trapezoidal {
    /// Maximum recursion depth for adaptivity.
    pub max_depth: usize,
}

impl Default for Trapezoidal {
    fn default() -> Self {
        Self { max_depth: 20 }
    }
}

impl Trapezoidal {
    /// Creates a new Trapezoidal integrator with specified max depth.
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    fn adaptive_step<F>(
        &self,
        f: &F,
        a: f64,
        b: f64,
        fa: f64,
        fb: f64,
        area: f64,
        tol: f64,
        depth: usize,
    ) -> (f64, f64)
    where
        F: Fn(f64) -> f64,
    {
        if depth == 0 {
            // Max depth reached, return current estimate.
            // Error is unknown but we return 0.0 to stop recursion.
            return (area, 0.0);
        }

        let mid = 0.5 * (a + b);
        let fmid = f(mid);
        let h_half = 0.5 * (b - a);

        let area_left = 0.5 * (fa + fmid) * h_half;
        let area_right = 0.5 * (fmid + fb) * h_half;
        let new_area = area_left + area_right;

        // Simple error estimate: |new - old|
        // A more rigorous one would be |new - old| / 3 for Simpson's.
        let error = (new_area - area).abs();

        if error < tol {
            (new_area, error)
        } else {
            let (l_val, l_err) = self.adaptive_step(f, a, mid, fa, fmid, area_left, tol / 2.0, depth - 1);
            let (r_val, r_err) = self.adaptive_step(f, mid, b, fmid, fb, area_right, tol / 2.0, depth - 1);
            (l_val + r_val, l_err + r_err)
        }
    }
}

impl Integrator for Trapezoidal {
    fn integrate<F>(&self, f: F, a: f64, b: f64, tol: f64) -> IntegrationResult
    where
        F: Fn(f64) -> f64,
    {
        let fa = f(a);
        let fb = f(b);
        let initial_h = b - a;
        let initial_area = 0.5 * (fa + fb) * initial_h;

        let (value, error) = self.adaptive_step(&f, a, b, fa, fb, initial_area, tol, self.max_depth);

        IntegrationResult { value, error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clenshaw_curtis_x_squared() {
        let integrator = ClenshawCurtis;
        // int_0^1 x^2 dx = 1/3
        let res = integrator.integrate(|x| x * x, 0.0, 1.0, 1e-9);
        assert!((res.value - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_trapezoidal_x_squared() {
        let integrator = Trapezoidal::default();
        // int_0^1 x^2 dx = 1/3
        let res = integrator.integrate(|x| x * x, 0.0, 1.0, 1e-6);
        assert!((res.value - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_sin_function() {
        let integrator_cc = ClenshawCurtis;
        let integrator_trap = Trapezoidal::default();

        // int_0^pi sin(x) dx = -cos(pi) - (-cos(0)) = 1 - (-1) = 2
        let pi = std::f64::consts::PI;

        let res_cc = integrator_cc.integrate(|x| x.sin(), 0.0, pi, 1e-9);
        assert!((res_cc.value - 2.0).abs() < 1e-9);

        let res_trap = integrator_trap.integrate(|x| x.sin(), 0.0, pi, 1e-6);
        assert!((res_trap.value - 2.0).abs() < 1e-6);
    }
}
