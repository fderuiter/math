use std::error::Error;
use std::fmt;

/// Errors that can occur during numerical analysis algorithms.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The root is not bracketed by the given interval (signs must be opposite).
    RootBracketingError {
        min: f64,
        max: f64,
        f_min: f64,
        f_max: f64,
    },
    /// The algorithm failed to converge within the maximum number of iterations.
    ConvergenceError { iterations: usize },
    /// The interval is invalid (e.g., min > max).
    InvalidInterval { min: f64, max: f64 },
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootBracketingError {
                min,
                max,
                f_min,
                f_max,
            } => write!(
                f,
                "Root not bracketed in [{:.4}, {:.4}]: f(min)={:.4}, f(max)={:.4}",
                min, max, f_min, f_max
            ),
            Self::ConvergenceError { iterations } => write!(
                f,
                "Algorithm failed to converge after {} iterations",
                iterations
            ),
            Self::InvalidInterval { min, max } => {
                write!(f, "Invalid interval: [{:.4}, {:.4}]", min, max)
            }
        }
    }
}

impl Error for AnalysisError {}

/// Strategy trait for finding roots of a continuous function $f(x) = 0$.
pub trait RootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    ///
    /// # Arguments
    /// * `f` - The function to find the root of.
    /// * `min` - The lower bound of the search interval.
    /// * `max` - The upper bound of the search interval.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Implements the Bisection method (binary search) for root finding.
///
/// This method is robust but converges linearly. It requires that the function
/// has opposite signs at the interval endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// The absolute error tolerance for the root.
    pub tolerance: f64,
    /// The maximum number of iterations allowed.
    pub max_iterations: usize,
}

impl Bisection {
    /// Creates a new Bisection solver with custom parameters.
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }
}

impl Default for Bisection {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 100,
        }
    }
}

impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        if min >= max {
            return Err(AnalysisError::InvalidInterval { min, max });
        }

        let mut low = min;
        let mut high = max;
        let mut f_low = f(low);
        let f_high = f(high);

        // Check for bracketing
        if f_low.signum() == f_high.signum() {
            // Check if one of them is exactly zero
            if f_low.abs() < f64::EPSILON {
                return Ok(low);
            }
            if f_high.abs() < f64::EPSILON {
                return Ok(high);
            }
            return Err(AnalysisError::RootBracketingError {
                min: low,
                max: high,
                f_min: f_low,
                f_max: f_high,
            });
        }

        for _ in 0..self.max_iterations {
            let mid = (low + high) / 2.0;
            let f_mid = f(mid);

            if (high - low).abs() < self.tolerance || f_mid.abs() < f64::EPSILON {
                return Ok(mid);
            }

            // If f_mid has same sign as f_low, root is in [mid, high]
            if f_mid.signum() == f_low.signum() {
                low = mid;
                f_low = f_mid;
            } else {
                high = mid;
                // f_high stays the same
            }
        }

        Err(AnalysisError::ConvergenceError {
            iterations: self.max_iterations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_simple_root() {
        let solver = Bisection::default();
        // f(x) = x^2 - 4. Root at 2.
        let result = solver.find_root(|x| x * x - 4.0, 0.0, 3.0).unwrap();
        assert!((result - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_bisection_no_bracket() {
        let solver = Bisection::default();
        // f(x) = x^2 + 1. Positive everywhere.
        let result = solver.find_root(|x| x * x + 1.0, 0.0, 3.0);
        assert!(matches!(result, Err(AnalysisError::RootBracketingError { .. })));
    }
}
