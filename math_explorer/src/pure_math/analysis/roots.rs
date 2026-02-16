//! Root finding algorithms.
//!
//! This module implements the **Strategy Pattern** for root finding, allowing
//! different algorithms (Bisection, Newton-Raphson, Brent's) to be used interchangeably.

use std::fmt;

/// Configuration for iterative solvers.
#[derive(Debug, Clone, Copy)]
pub struct IterativeParams {
    /// Maximum number of iterations before giving up.
    pub max_iterations: usize,
    /// Absolute tolerance for the root value (convergence criteria).
    pub tolerance: f64,
}

impl Default for IterativeParams {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

impl IterativeParams {
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }
}

/// Errors that can occur during numerical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The algorithm failed to converge within the maximum number of iterations.
    /// Contains the best guess so far.
    ConvergenceError(f64),
    /// Invalid parameters were provided.
    InvalidParameters(String),
    /// Root is not bracketed within the interval.
    RootNotBracketed {
        min: f64,
        max: f64,
        f_min: f64,
        f_max: f64,
    },
    /// Derivative became too small during iteration.
    DerivativeTooSmall { x: f64, derivative: f64 },
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvergenceError(guess) => {
                write!(f, "Algorithm failed to converge. Best guess: {}", guess)
            }
            Self::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            Self::RootNotBracketed {
                min,
                max,
                f_min,
                f_max,
            } => {
                write!(
                    f,
                    "Root not bracketed in [{}, {}]: f({})={}, f({})={}",
                    min, max, min, f_min, max, f_max
                )
            }
            Self::DerivativeTooSmall { x, derivative } => {
                write!(f, "Derivative too small at x={}: {}", x, derivative)
            }
        }
    }
}

impl std::error::Error for AnalysisError {}

/// Strategy for finding roots of a function $f(x) = 0$.
pub trait RootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    ///
    /// # Arguments
    /// * `f` - The objective function.
    /// * `min` - Lower bound of the search interval.
    /// * `max` - Upper bound of the search interval.
    ///
    /// # Returns
    /// * `Ok(f64)` - The approximate root.
    /// * `Err(AnalysisError)` - If the root cannot be found or parameters are invalid.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Strategy for finding roots of a function $f(x) = 0$ given an initial guess.
pub trait OpenRootFinder {
    /// Finds a root of the function `f` starting from `guess`.
    fn find_root_open<F>(&self, f: F, guess: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Strategy for finding roots using a derivative $f'(x)$.
pub trait DifferentiableRootFinder {
    /// Finds a root of the function `f` with derivative `f_prime` starting from `guess`.
    fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64;
}

/// Bisection method implementation.
///
/// A robust but slow method that iteratively halves the interval.
/// Guaranteed to converge if the function is continuous and changes sign over the interval.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    pub params: IterativeParams,
}

impl Bisection {
    /// Creates a new Bisection solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            params: IterativeParams::new(max_iterations, tolerance),
        }
    }
}

impl Default for Bisection {
    fn default() -> Self {
        Self {
            params: IterativeParams::default(),
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
        if f_low.abs() < self.params.tolerance {
            return Ok(low);
        }

        let f_high = f(high);
        if f_high.abs() < self.params.tolerance {
            return Ok(high);
        }

        // Check if root is bracketed
        if f_low.signum() == f_high.signum() {
            return Err(AnalysisError::RootNotBracketed {
                min: low,
                max: high,
                f_min: f_low,
                f_max: f_high,
            });
        }

        let mut mid = low;

        for _ in 0..self.params.max_iterations {
            mid = (low + high) / 2.0;

            // Check convergence on domain (interval size)
            if (high - low).abs() < self.params.tolerance {
                return Ok(mid);
            }

            let f_mid = f(mid);

            // Check convergence on range (function value)
            if f_mid.abs() < self.params.tolerance {
                return Ok(mid);
            }

            // Narrow the interval
            // If f(low) and f(mid) have different signs, root is in [low, mid]
            if f_low.signum() != f_mid.signum() {
                high = mid;
                // high moved, low stays same, so f_low is still valid for next iter
            } else {
                low = mid;
                f_low = f_mid; // low moved, must update f_low
            }
        }

        // Return best guess (mid) if convergence not reached
        Err(AnalysisError::ConvergenceError(mid))
    }
}

/// Newton-Raphson method implementation.
///
/// Uses the derivative of the function to converge quadratically to the root.
/// If the derivative is not provided (via `RootFinder` trait), a numerical approximation is used.
#[derive(Debug, Clone, Copy)]
pub struct NewtonRaphson {
    pub params: IterativeParams,
}

impl NewtonRaphson {
    /// Creates a new Newton-Raphson solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            params: IterativeParams::new(max_iterations, tolerance),
        }
    }

    /// Finds a root using an analytical derivative $f'(x)$.
    ///
    /// # Arguments
    /// * `f` - The objective function.
    /// * `f_prime` - The derivative of the objective function.
    /// * `guess` - Initial guess for the root.
    pub fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        let mut x = guess;

        for _ in 0..self.params.max_iterations {
            let y = f(x);
            if y.abs() < self.params.tolerance {
                return Ok(x);
            }

            let dy = f_prime(x);
            if dy.abs() < 1e-14 {
                return Err(AnalysisError::DerivativeTooSmall { x, derivative: dy });
            }

            let next_x = x - y / dy;

            // Check convergence on domain step
            if (next_x - x).abs() < self.params.tolerance {
                return Ok(next_x);
            }

            x = next_x;
        }

        Err(AnalysisError::ConvergenceError(x))
    }
}

impl Default for NewtonRaphson {
    fn default() -> Self {
        Self {
            params: IterativeParams::default(),
        }
    }
}

impl RootFinder for NewtonRaphson {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        // Use midpoint as initial guess
        let guess = (min + max) / 2.0;
        self.find_root_open(f, guess)
    }
}

impl OpenRootFinder for NewtonRaphson {
    fn find_root_open<F>(&self, f: F, guess: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        // Numerical derivative step size
        let h = 1e-7;

        let derivative = |x: f64| (f(x + h) - f(x - h)) / (2.0 * h);

        // We use the specialized method with our numerical derivative
        self.find_root_with_derivative(&f, derivative, guess)
    }
}

impl DifferentiableRootFinder for NewtonRaphson {
    fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        self.find_root_with_derivative(f, f_prime, guess)
    }
}

/// Secant method implementation.
///
/// An open method that approximates the derivative using a secant line between two points.
/// Requires two initial guesses or a single guess (with a small step).
#[derive(Debug, Clone, Copy)]
pub struct Secant {
    pub params: IterativeParams,
}

impl Secant {
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            params: IterativeParams::new(max_iterations, tolerance),
        }
    }
}

impl Default for Secant {
    fn default() -> Self {
        Self {
            params: IterativeParams::default(),
        }
    }
}

impl OpenRootFinder for Secant {
    fn find_root_open<F>(&self, f: F, guess: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        let mut x0 = guess;
        // Generate a second point slightly offset for the first step
        let mut x1 = if x0.abs() < 1e-9 { 1e-4 } else { x0 * 1.001 };

        let mut f_x0 = f(x0);
        let mut f_x1 = f(x1);

        if f_x0.abs() < self.params.tolerance {
            return Ok(x0);
        }

        for _ in 0..self.params.max_iterations {
            if f_x1.abs() < self.params.tolerance {
                return Ok(x1);
            }

            // Avoid division by zero
            if (f_x1 - f_x0).abs() < 1e-14 {
                return Err(AnalysisError::InvalidParameters(format!(
                    "Secant slope too small between x0={} and x1={}",
                    x0, x1
                )));
            }

            let x_new = x1 - f_x1 * (x1 - x0) / (f_x1 - f_x0);

            // Check convergence on domain
            if (x_new - x1).abs() < self.params.tolerance {
                return Ok(x_new);
            }

            x0 = x1;
            f_x0 = f_x1;
            x1 = x_new;
            f_x1 = f(x1);
        }

        Err(AnalysisError::ConvergenceError(x1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secant_sqrt() {
        let solver = Secant::default();
        // x^2 - 2 = 0
        let root = solver.find_root_open(|x| x * x - 2.0, 1.0).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_newton_raphson_sqrt() {
        let solver = NewtonRaphson::default();
        // x^2 - 2 = 0, derivative 2x
        let root = solver
            .find_root_with_derivative(|x| x * x - 2.0, |x| 2.0 * x, 1.5)
            .unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_newton_raphson_numerical() {
        let solver = NewtonRaphson::default();
        // x^2 - 2 = 0
        let root = solver.find_root(|x| x * x - 2.0, 1.0, 2.0).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-5);
    }

    #[test]
    fn test_bisection_square_root() {
        let solver = Bisection::default();
        // x^2 - 2 = 0  => x = sqrt(2) approx 1.41421356
        let root = solver.find_root(|x| x * x - 2.0, 1.0, 2.0).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_bisection_linear() {
        let solver = Bisection::default();
        // 2x - 4 = 0 => x = 2
        let root = solver.find_root(|x| 2.0 * x - 4.0, 0.0, 5.0).unwrap();
        assert!((root - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_not_bracketed() {
        let solver = Bisection::default();
        // x^2 + 1 = 0 has no real roots. And signs are always positive.
        let result = solver.find_root(|x| x * x + 1.0, -2.0, 2.0);
        assert!(matches!(result, Err(AnalysisError::RootNotBracketed { .. })));
    }
}
