//! Root finding algorithms.
//!
//! This module implements the **Strategy Pattern** for root finding, allowing
//! different algorithms (Bisection, Newton-Raphson, Brent's) to be used interchangeably.

use std::fmt;

/// Errors that can occur during numerical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The algorithm failed to converge within the maximum number of iterations.
    /// Contains the best guess so far.
    ConvergenceError(f64),
    /// Invalid parameters were provided (e.g., root not bracketed).
    InvalidParameters(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvergenceError(guess) => {
                write!(f, "Algorithm failed to converge. Best guess: {}", guess)
            }
            Self::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
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

/// Bisection method implementation.
///
/// A robust but slow method that iteratively halves the interval.
/// Guaranteed to converge if the function is continuous and changes sign over the interval.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// Maximum number of iterations before giving up.
    pub max_iterations: usize,
    /// Absolute tolerance for the root value (convergence criteria).
    pub tolerance: f64,
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

        let f_high = f(high);
        if f_high.abs() < self.tolerance {
            return Ok(high);
        }

        // Check if root is bracketed
        if f_low.signum() == f_high.signum() {
            return Err(AnalysisError::InvalidParameters(format!(
                "Root not bracketed: f({})={}, f({})={}. Signs must differ.",
                low, f_low, high, f_high
            )));
        }

        let mut mid = low;

        for _ in 0..self.max_iterations {
            mid = (low + high) / 2.0;

            // Check convergence on domain (interval size)
            if (high - low).abs() < self.tolerance {
                return Ok(mid);
            }

            let f_mid = f(mid);

            // Check convergence on range (function value)
            if f_mid.abs() < self.tolerance {
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
/// Uses the derivative of the function to iteratively find a root.
/// $x_{n+1} = x_n - f(x_n) / f'(x_n)$
#[derive(Debug, Clone, Copy)]
pub struct NewtonRaphson {
    /// Maximum number of iterations before giving up.
    pub max_iterations: usize,
    /// Absolute tolerance for the root value (convergence criteria).
    pub tolerance: f64,
    /// Step size for numerical differentiation.
    pub epsilon: f64,
}

impl Default for NewtonRaphson {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-7,
            epsilon: 1e-5,
        }
    }
}

impl NewtonRaphson {
    /// Creates a new Newton-Raphson solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
            epsilon: 1e-5,
        }
    }

    /// Finds a root using the analytical derivative df.
    ///
    /// # Arguments
    /// * `f` - The objective function.
    /// * `df` - The derivative of the objective function.
    /// * `initial_guess` - Initial guess for the root.
    /// * `bounds` - Optional strict bounds [min, max]. If the next step falls outside,
    ///              a fallback strategy (midpoint towards bound) is used.
    pub fn find_root_with_derivative<F, DF>(
        &self,
        f: F,
        df: DF,
        initial_guess: f64,
        bounds: Option<(f64, f64)>,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        DF: Fn(f64) -> f64,
    {
        let mut x = initial_guess;

        // Initial bounds check
        if let Some((min, max)) = bounds {
            if x < min {
                x = min + self.tolerance;
            }
            if x > max {
                x = max - self.tolerance;
            }
        }

        for _ in 0..self.max_iterations {
            let fx = f(x);

            // Convergence check on function value
            if fx.abs() < self.tolerance {
                return Ok(x);
            }

            let dfx = df(x);
            if dfx.abs() < 1e-10 {
                return Err(AnalysisError::ConvergenceError(x));
            }

            let next_x = x - fx / dfx;

            if (next_x - x).abs() < self.tolerance {
                return Ok(next_x);
            }

            // Bounds handling
            if let Some((min, max)) = bounds {
                if next_x <= min {
                    // Backtrack towards min
                    x = (x + min) / 2.0;
                } else if next_x >= max {
                    // Backtrack towards max
                    x = (x + max) / 2.0;
                } else {
                    x = next_x;
                }
            } else {
                x = next_x;
            }
        }

        Err(AnalysisError::ConvergenceError(x))
    }
}

impl RootFinder for NewtonRaphson {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        // Use midpoint as guess
        let guess = (min + max) / 2.0;
        // Numerical differentiation
        let df = |x| (f(x + self.epsilon) - f(x - self.epsilon)) / (2.0 * self.epsilon);

        self.find_root_with_derivative(&f, df, guess, Some((min, max)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(matches!(result, Err(AnalysisError::InvalidParameters(_))));
    }

    #[test]
    fn test_newton_raphson_square_root() {
        let solver = NewtonRaphson::default();
        // x^2 - 2 = 0
        let f = |x: f64| x * x - 2.0;
        let df = |x: f64| 2.0 * x;
        let root = solver.find_root_with_derivative(f, df, 1.5, None).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_newton_raphson_numerical_diff() {
        let solver = NewtonRaphson::default();
        // x^2 - 2 = 0. Range [1.0, 2.0]. Guess will be 1.5.
        let root = solver.find_root(|x| x * x - 2.0, 1.0, 2.0).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-5); // Slightly looser tolerance due to epsilon
    }

    #[test]
    fn test_newton_raphson_bounded() {
        let solver = NewtonRaphson::default();
        // f(x) = x - 1. Root is 1.
        // Bounds [0.5, 1.5].
        // Start at 0.6.
        // f(0.6) = -0.4. f'(0.6) = 1.
        // next_x = 0.6 - (-0.4)/1 = 1.0. Correct.

        // Use a function that overshoots.
        // f(x) = atan(x). Root 0.
        // f'(x) = 1/(1+x^2).
        // If x is large, f(x) ~ pi/2, f'(x) is small. Step is huge.
        // x = 2. f(2) = 1.1. f'(2) = 0.2.
        // next_x = 2 - 1.1/0.2 = 2 - 5.5 = -3.5.
        // Bound [-3, 3]. -3.5 is out.
        // Should trigger backtracking.
        let f = |x: f64| x.atan();
        let df = |x: f64| 1.0 / (1.0 + x * x);
        let bounds = Some((-3.0, 3.0));

        let result = solver.find_root_with_derivative(f, df, 2.0, bounds);
        assert!(result.is_ok());
        assert!(result.unwrap().abs() < 1e-6);
    }
}
