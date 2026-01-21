use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    ConvergenceError,
    InvalidParameters(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::ConvergenceError => write!(f, "Algorithm failed to converge"),
            AnalysisError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
        }
    }
}

impl Error for AnalysisError {}

/// A trait for finding roots of a function f(x) = 0.
pub trait RootFinder {
    /// Finds a root of the function `f` within the given bounds.
    ///
    /// # Arguments
    /// * `f` - The function to find the root of.
    /// * `min` - The lower bound of the search interval.
    /// * `max` - The upper bound of the search interval.
    ///
    /// # Returns
    /// The estimated root `x` such that `f(x) \approx 0`, or an error.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Bisection method implementation.
///
/// robust but slow method that requires the root to be bracketed.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    pub max_iterations: usize,
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

impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        let mut low = min;
        let mut high = max;

        let mut f_low = f(low);
        if f_low.abs() < self.tolerance {
            return Ok(low);
        }

        // We don't strictly require the root to be bracketed if the function behavior is known (like monotonic)
        // but for a general solver, we should be careful.
        // However, standard bisection assumes bracket.
        // If the user provides a range where f(low) * f(high) > 0, it might just shrink to one side.

        for _ in 0..self.max_iterations {
            let mid = (low + high) / 2.0;
            let f_mid = f(mid);

            if f_mid.abs() < self.tolerance || (high - low).abs() < self.tolerance {
                return Ok(mid);
            }

            // Standard Bisection Logic:
            // If f(low) * f(mid) < 0, the root is in [low, mid].
            // Otherwise, it is in [mid, high].
            // If the root is not bracketed, this logic will push towards the side with the sign change
            // or fail if there are no sign changes.
            if f_low.signum() != f_mid.signum() {
                high = mid;
                // f_high would be f_mid, but we don't track f_high explicitly in the loop
            } else {
                low = mid;
                f_low = f_mid; // Update cached value
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
        // f(x) = x^2 - 4. Roots at 2, -2.
        // Search in [0, 3]. f(0)=-4, f(3)=5. Sign change ok.
        let root = solver.find_root(|x| x * x - 4.0, 0.0, 3.0).unwrap();
        assert!((root - 2.0).abs() < 1e-5);
    }
}
