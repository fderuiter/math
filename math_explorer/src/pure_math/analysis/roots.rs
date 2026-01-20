//! Root finding algorithms.
//!
//! This module provides a strategy pattern for finding roots of functions ($f(x) = 0$).

/// Errors that can occur during root finding.
#[derive(Debug, Clone)]
pub enum AnalysisError {
    /// The algorithm failed to converge within the maximum number of iterations.
    ConvergenceError,
    /// The root is not bracketed by the given bounds (i.e., f(min) and f(max) have the same sign).
    RootBracketingError,
    /// Invalid parameters (e.g., min > max).
    InvalidParameters(String),
}

/// A strategy for finding the root of a continuous function.
pub trait RootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    ///
    /// # Arguments
    /// * `f` - The function to find the root of.
    /// * `min` - Lower bound of the interval.
    /// * `max` - Upper bound of the interval.
    ///
    /// # Returns
    /// The estimated root `x` such that `f(x) \approx 0`.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Bisection method implementation.
///
/// A robust but slow method that iteratively bisects the interval.
/// Requires the root to be bracketed (signs of f(min) and f(max) must be different).
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Tolerance for the root (x-domain absolute error).
    pub tolerance: f64,
    /// Tolerance for the function value (y-domain absolute error).
    pub y_tolerance: f64,
}

impl Bisection {
    /// Creates a new Bisection solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
            y_tolerance: 1e-9, // Default epsilon
        }
    }

    /// Creates a new Bisection solver with custom y-tolerance.
    pub fn new_with_y_tol(max_iterations: usize, tolerance: f64, y_tolerance: f64) -> Self {
         Self {
            max_iterations,
            tolerance,
            y_tolerance,
        }
    }
}

impl Default for Bisection {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
            y_tolerance: 1e-9,
        }
    }
}

impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        if min >= max {
            return Err(AnalysisError::InvalidParameters(
                "min must be less than max".to_string(),
            ));
        }

        let mut low = min;
        let mut high = max;
        let mut f_low = f(low);
        let f_high = f(high);

        // Strict bracketing check
        if f_low.signum() == f_high.signum() {
             return Err(AnalysisError::RootBracketingError);
        }

        let mut mid = low;

        for _ in 0..self.max_iterations {
            mid = (low + high) / 2.0;
            if (high - low).abs() < self.tolerance {
                return Ok(mid);
            }

            let val = f(mid);

            if val.abs() < self.y_tolerance {
                return Ok(mid); // Found exact root
            }

            // Standard bisection checks signs
            if val.signum() == f_low.signum() {
                low = mid;
                f_low = val; // Optimization: update f_low
            } else {
                high = mid;
            }
        }

        // Return best guess after iterations
        Ok(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_simple() {
        let solver = Bisection::default();
        // Root of x^2 - 4 at x=2
        let root = solver.find_root(|x| x * x - 4.0, 0.0, 5.0).unwrap();
        assert!((root - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_bisection_bracketing_error() {
        let solver = Bisection::default();
        // x^2 + 1 has no real roots, so [0, 5] is unbracketed (both positive)
        let result = solver.find_root(|x| x * x + 1.0, 0.0, 5.0);
        assert!(matches!(result, Err(AnalysisError::RootBracketingError)));
    }
}
