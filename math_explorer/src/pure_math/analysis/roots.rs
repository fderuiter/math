//! Root finding algorithms.

use std::fmt;

/// Errors that can occur during numerical analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalysisError {
    /// The root is not bracketed by the initial interval.
    RootBracketingError,
    /// The algorithm failed to converge within the maximum number of iterations.
    ConvergenceError,
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalysisError::RootBracketingError => write!(f, "Root is not bracketed by the interval"),
            AnalysisError::ConvergenceError => write!(f, "Algorithm failed to converge"),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// A strategy for finding the root of a function.
pub trait RootFinder {
    /// Finds a root of the function $f(x) = 0$ within the interval $[min, max]$.
    fn find_root(
        &self,
        f: impl Fn(f64) -> f64,
        min: f64,
        max: f64,
    ) -> Result<f64, AnalysisError>;
}

/// The Bisection method.
///
/// A robust root-finding method that repeatedly bisects an interval and selects
/// a sub-interval in which a root must lie for the function $f(x)$.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// The maximum number of iterations.
    pub max_iterations: usize,
    /// The tolerance for the root value (x-tolerance).
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
    /// Creates a new Bisection solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }
}

impl RootFinder for Bisection {
    fn find_root(
        &self,
        f: impl Fn(f64) -> f64,
        min: f64,
        max: f64,
    ) -> Result<f64, AnalysisError> {
        let mut a = min;
        let mut b = max;
        let mut fa = f(a);
        let fb = f(b);

        // Check if endpoints are roots
        if fa.abs() < 1e-9 { return Ok(a); } // Use a small epsilon for zero check
        if fb.abs() < 1e-9 { return Ok(b); }

        if fa * fb > 0.0 {
            return Err(AnalysisError::RootBracketingError);
        }

        for _ in 0..self.max_iterations {
            let mid = (a + b) / 2.0;
            // Convergence check on interval size
            if (b - a).abs() / 2.0 < self.tolerance {
                return Ok(mid);
            }

            let fmid = f(mid);

            // Convergence check on function value (optional but good)
            if fmid.abs() < 1e-9 {
                return Ok(mid);
            }

            if fa * fmid < 0.0 {
                b = mid;
                // fb = fmid; // Not strictly needed
            } else {
                a = mid;
                fa = fmid;
            }
        }

        // Return best estimate even if max iterations reached?
        // Usually strictly returning error is better for "Systems Core".
        // But for "Robustness" maybe return Ok?
        // Prompt: "The compiler is your co-pilot... Enemy is brittleness".
        // Returning Ok((a+b)/2.0) is safer than failing if we are close enough but ran out of steps.
        // But I defined ConvergenceError. I should use it.
        Err(AnalysisError::ConvergenceError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_simple_root() {
        let solver = Bisection::default();
        // Root of x^2 - 4 is 2 (in [0, 3])
        let root = solver.find_root(|x| x * x - 4.0, 0.0, 3.0).unwrap();
        assert!((root - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_bisection_not_bracketed() {
        let solver = Bisection::default();
        // x^2 + 1 has no real roots. [0, 10] -> f(0)=1, f(10)=101. Positive * Positive > 0.
        let result = solver.find_root(|x| x * x + 1.0, 0.0, 10.0);
        assert!(matches!(result, Err(AnalysisError::RootBracketingError)));
    }
}
