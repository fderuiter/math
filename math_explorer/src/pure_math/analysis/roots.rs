//! Root finding algorithms.
//!
//! This module implements the **Strategy Pattern** for root finding, distinguishing between
//! **Bracketing Methods** (which require an interval containing the root) and
//! **Open Methods** (which require an initial guess but not a bracket).

use std::fmt;

/// Errors that can occur during numerical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    /// The algorithm failed to converge within the maximum number of iterations.
    /// Contains the best guess so far.
    ConvergenceError(f64),
    /// The root was not bracketed by the given interval (signs must differ).
    RootNotBracketed {
        min: f64,
        max: f64,
        f_min: f64,
        f_max: f64,
    },
    /// The derivative was too small to continue (e.g., in Newton-Raphson).
    DerivativeTooSmall {
        x: f64,
        derivative: f64,
    },
    /// The provided interval was invalid (e.g., min > max).
    InvalidInterval {
        min: f64,
        max: f64,
    },
    /// Invalid parameters were provided (Legacy variant).
    #[deprecated(note = "Use specific error variants instead.")]
    InvalidParameters(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvergenceError(guess) => {
                write!(f, "Algorithm failed to converge. Best guess: {}", guess)
            }
            Self::RootNotBracketed { min, max, f_min, f_max } => write!(
                f,
                "Root not bracketed in [{}, {}]: f({})={}, f({})={}. Signs must differ.",
                min, max, min, f_min, max, f_max
            ),
            Self::DerivativeTooSmall { x, derivative } => write!(
                f,
                "Derivative too small at x={}: {}. Cannot continue.",
                x, derivative
            ),
            Self::InvalidInterval { min, max } => write!(
                f,
                "Invalid interval: [{}, {}]. Min must be less than or equal to Max.",
                min, max
            ),
            #[allow(deprecated)]
            Self::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// Strategy for finding roots when the root is known to be within an interval `[min, max]`.
///
/// Implementations (like [`Bisection`]) guarantee that if a root exists and is bracketed,
/// it will be found (within tolerance).
pub trait BracketingRootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Strategy for finding roots using an initial guess.
///
/// Open methods (like [`NewtonRaphson`]) do not require the root to be bracketed,
/// but convergence is not guaranteed and depends on the quality of the initial guess.
pub trait OpenRootFinder {
    /// Finds a root of the function `f` starting from `initial_guess`.
    fn find_root<F>(&self, f: F, initial_guess: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64;
}

/// Strategy for finding roots using the function's derivative.
pub trait DifferentiableRootFinder {
    /// Finds a root using the function `f` and its derivative `f_prime`.
    fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        initial_guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64;
}

/// Legacy strategy trait.
///
/// # Deprecation
/// Use [`BracketingRootFinder`] or [`OpenRootFinder`] instead to enforce correct usage semantics.
#[deprecated(note = "Use BracketingRootFinder or OpenRootFinder instead.")]
pub trait RootFinder {
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

impl BracketingRootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        if min > max {
            return Err(AnalysisError::InvalidInterval { min, max });
        }

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
            return Err(AnalysisError::RootNotBracketed {
                min: low,
                max: high,
                f_min: f_low,
                f_max: f_high,
            });
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

#[allow(deprecated)]
impl RootFinder for Bisection {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        <Self as BracketingRootFinder>::find_root(self, f, min, max)
    }
}

/// Newton-Raphson method implementation.
///
/// Uses the derivative of the function to converge quadratically to the root.
/// If the derivative is not provided (via [`DifferentiableRootFinder`]), a numerical approximation is used.
#[derive(Debug, Clone, Copy)]
pub struct NewtonRaphson {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Absolute tolerance for the root value.
    pub tolerance: f64,
}

impl NewtonRaphson {
    /// Creates a new Newton-Raphson solver.
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }

    /// Finds a root using an analytical derivative $f'(x)$.
    ///
    /// This method is a convenience wrapper for [`DifferentiableRootFinder::find_root_with_derivative`].
    pub fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        initial_guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        <Self as DifferentiableRootFinder>::find_root_with_derivative(self, f, f_prime, initial_guess)
    }
}

impl Default for NewtonRaphson {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

impl DifferentiableRootFinder for NewtonRaphson {
    fn find_root_with_derivative<F, D>(
        &self,
        f: F,
        f_prime: D,
        initial_guess: f64,
    ) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
        D: Fn(f64) -> f64,
    {
        let mut x = initial_guess;

        for _ in 0..self.max_iterations {
            let y = f(x);
            if y.abs() < self.tolerance {
                return Ok(x);
            }

            let dy = f_prime(x);
            if dy.abs() < 1e-14 {
                return Err(AnalysisError::DerivativeTooSmall { x, derivative: dy });
            }

            let next_x = x - y / dy;

            // Check convergence on domain step
            if (next_x - x).abs() < self.tolerance {
                return Ok(next_x);
            }

            x = next_x;
        }

        Err(AnalysisError::ConvergenceError(x))
    }
}

impl OpenRootFinder for NewtonRaphson {
    fn find_root<F>(&self, f: F, initial_guess: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        // Numerical derivative step size
        let h = 1e-7;
        let derivative = |x: f64| (f(x + h) - f(x - h)) / (2.0 * h);

        self.find_root_with_derivative(&f, derivative, initial_guess)
    }
}

#[allow(deprecated)]
impl RootFinder for NewtonRaphson {
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Result<f64, AnalysisError>
    where
        F: Fn(f64) -> f64,
    {
        // Use midpoint as initial guess
        let guess = (min + max) / 2.0;
        <Self as OpenRootFinder>::find_root(self, f, guess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let root = <NewtonRaphson as OpenRootFinder>::find_root(&solver, |x| x * x - 2.0, 1.5).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-5);
    }

    #[test]
    fn test_bisection_square_root() {
        let solver = Bisection::default();
        // x^2 - 2 = 0  => x = sqrt(2) approx 1.41421356
        let root = <Bisection as BracketingRootFinder>::find_root(&solver, |x| x * x - 2.0, 1.0, 2.0).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_bisection_linear() {
        let solver = Bisection::default();
        // 2x - 4 = 0 => x = 2
        let root = <Bisection as BracketingRootFinder>::find_root(&solver, |x| 2.0 * x - 4.0, 0.0, 5.0).unwrap();
        assert!((root - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_not_bracketed() {
        let solver = Bisection::default();
        // x^2 + 1 = 0 has no real roots. And signs are always positive.
        let result = <Bisection as BracketingRootFinder>::find_root(&solver, |x| x * x + 1.0, -2.0, 2.0);
        match result {
            Err(AnalysisError::RootNotBracketed { .. }) => (),
            _ => panic!("Expected RootNotBracketed error"),
        }
    }
}
