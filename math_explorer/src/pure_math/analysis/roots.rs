//! Root finding algorithms.
//!
//! This module provides a generic `RootFinder` trait and concrete implementations
//! like `Bisection` for finding roots of continuous functions.

use std::fmt;

/// Errors that can occur during root finding.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The function values at the interval endpoints do not have opposite signs.
    RootBracketingError(String),
    /// The algorithm failed to converge within the maximum number of iterations.
    ConvergenceError(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::RootBracketingError(msg) => write!(f, "Root bracketing error: {}", msg),
            AnalysisError::ConvergenceError(msg) => write!(f, "Convergence error: {}", msg),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// A strategy for finding roots of a function $f(x) = 0$.
pub trait RootFinder {
    /// Finds a root of the function `f` in the interval `[min, max]`.
    ///
    /// # Arguments
    /// * `f` - The function to find the root of.
    /// * `min` - The lower bound of the interval.
    /// * `max` - The upper bound of the interval.
    ///
    /// # Returns
    /// The approximate value of $x$ such that $f(x) \approx 0$.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Bisection method for root finding.
///
/// A robust but slow method that iteratively bisects an interval ensuring the root
/// remains bracketed.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Tolerance for convergence (absolute error).
    pub tolerance: f64,
}

impl Default for Bisection {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

impl Bisection {
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }
}

impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        let mut low = min;
        let mut high = max;
        let mut f_low = f(low);
        let f_high = f(high);

        if f_low.abs() < self.tolerance {
            return Ok(low);
        }
        if f_high.abs() < self.tolerance {
            return Ok(high);
        }

        if f_low.signum() == f_high.signum() {
            return Err(AnalysisError::RootBracketingError(format!(
                "Function must have opposite signs at bounds: f({})={}, f({})={}",
                min, f_low, max, f_high
            )));
        }

        for _ in 0..self.max_iterations {
            let mid = (low + high) / 2.0;
            if (high - low).abs() < self.tolerance {
                return Ok(mid);
            }

            let f_mid = f(mid);
            if f_mid.abs() < self.tolerance {
                return Ok(mid);
            }

            // If f(low) and f(mid) have different signs, the root is in [low, mid].
            if f_low.signum() != f_mid.signum() {
                high = mid;
                // f_high = f_mid; // Not strictly used
            } else {
                // Otherwise, the root is in [mid, high].
                low = mid;
                f_low = f_mid;
            }
        }

        Err(AnalysisError::ConvergenceError(format!(
            "Bisection failed to converge after {} iterations. Last interval: [{}, {}]",
            self.max_iterations, low, high
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_square_root() {
        // Find sqrt(2) by solving x^2 - 2 = 0
        let solver = Bisection::default();
        let root = solver
            .find_root(|x| x * x - 2.0, 1.0, 2.0)
            .expect("Should converge");
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_bracketing_error() {
        let solver = Bisection::default();
        let result = solver.find_root(|x| x * x + 1.0, -1.0, 1.0); // No real root
        assert!(matches!(result, Err(AnalysisError::RootBracketingError(_))));
    }
}
