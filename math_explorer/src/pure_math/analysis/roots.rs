//! Root finding algorithms.
//!
//! This module implements the Strategy Pattern for finding roots of functions.
//! $$ f(x) = 0 $$

use std::fmt;

/// Errors related to root finding.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The root is not bracketed by the given interval.
    RootBracketingError { min: f64, max: f64, f_min: f64, f_max: f64 },
    /// The algorithm failed to converge within the maximum number of iterations.
    ConvergenceError,
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootBracketingError { min, max, f_min, f_max } => write!(
                f,
                "Root not bracketed in [{:.4}, {:.4}]. f(min)={:.4}, f(max)={:.4}",
                min, max, f_min, f_max
            ),
            Self::ConvergenceError => write!(f, "Algorithm failed to converge"),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// A strategy for finding a root of a function.
pub trait RootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Bisection Method.
///
/// A robust root-finding method that repeatedly bisects an interval and selects
/// a sub-interval in which a root must lie for further processing.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Tolerance for convergence (absolute error).
    pub tolerance: f64,
}

impl Bisection {
    /// Creates a new Bisection solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self { max_iterations, tolerance }
    }
}

impl Default for Bisection {
    fn default() -> Self {
        Self { max_iterations: 100, tolerance: 1e-6 }
    }
}

impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        let mut low = min;
        let mut high = max;
        let f_low = f(low);
        let f_high = f(high);

        if f_low * f_high > 0.0 {
             return Err(AnalysisError::RootBracketingError {
                 min: low,
                 max: high,
                 f_min: f_low,
                 f_max: f_high
             });
        }

        // If one of them is already 0, return it
        if f_low == 0.0 { return Ok(low); }
        if f_high == 0.0 { return Ok(high); }

        // We cache f(low) sign to compare against f(mid)
        let f_low_sign = f_low.signum();

        for _ in 0..self.max_iterations {
            let mid = (low + high) / 2.0;

            // Check convergence on domain
            if (high - low).abs() < self.tolerance {
                return Ok(mid);
            }

            let f_mid = f(mid);

            if f_mid == 0.0 {
                return Ok(mid);
            }

            // If f_mid has same sign as f_low, root is in [mid, high]
            if f_mid.signum() == f_low_sign {
                low = mid;
                // f_low effectively becomes f_mid, so the sign remains f_low_sign
            } else {
                high = mid;
            }
        }

        Err(AnalysisError::ConvergenceError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_simple_root() {
        let solver = Bisection::default();
        // f(x) = x^2 - 4, root at 2. bracket [0, 3]
        let root = solver.find_root(|x| x * x - 4.0, 0.0, 3.0).unwrap();
        assert!((root - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_bisection_convergence() {
        let solver = Bisection::new(100, 1e-10);
        // f(x) = x - cos(x), root approx 0.739085
        let root = solver.find_root(|x| x - x.cos(), 0.0, 1.0).unwrap();
        assert!((root - 0.7390851332).abs() < 1e-9);
    }

    #[test]
    fn test_bracketing_error() {
        let solver = Bisection::default();
        // f(x) = x^2 + 1, no real roots. bracket [-1, 1] (both positive)
        let result = solver.find_root(|x| x * x + 1.0, -1.0, 1.0);
        match result {
            Err(AnalysisError::RootBracketingError { .. }) => (),
            _ => panic!("Expected RootBracketingError"),
        }
    }
}
