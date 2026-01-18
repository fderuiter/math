//! Root finding algorithms.
//!
//! Provides a common interface for root finding strategies.

/// A strategy for finding roots of real-valued functions.
pub trait RootFinder {
    /// Finds a root of the function `f` within the interval `[min, max]`.
    ///
    /// # Arguments
    /// * `f` - The function to find the root of.
    /// * `min` - The lower bound of the search interval.
    /// * `max` - The upper bound of the search interval.
    ///
    /// # Returns
    /// * `Option<f64>` - The estimated root, or `None` if the method fails to converge
    ///   or the interval is invalid for the chosen strategy.
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Option<f64>
    where
        F: Fn(f64) -> f64;
}

/// Bisection Method.
///
/// A robust root-finding method that repeatedly bisects an interval and then
/// selects a subinterval in which a root must lie for further processing.
#[derive(Debug, Clone, Copy)]
pub struct Bisection {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Tolerance for convergence.
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
    fn find_root<F>(&self, f: F, min: f64, max: f64) -> Option<f64>
    where
        F: Fn(f64) -> f64,
    {
        let mut a = min;
        let mut b = max;
        let mut fa = f(a);
        let fb = f(b);

        if fa.abs() < self.tolerance {
            return Some(a);
        }
        if fb.abs() < self.tolerance {
            return Some(b);
        }

        // Bisection requires opposite signs at the boundaries.
        // However, to support legacy behavior where monotonicity was assumed
        // without explicit sign check (converging to boundary if no root),
        // we might need a looser implementation or specialized one.
        // But for a generic "RootFinder", strictness is better.
        // Let's implement standard bisection.
        if fa * fb > 0.0 {
            // Check if one bound is closer to 0 significantly?
            // If not bracketed, standard bisection can't proceed.
            // But wait, the original code in mechanism_design.rs assumed
            // J(v) is increasing.
            // If J(min) > 0, then root is < min. But we are bounded by min.
            // So closest valid "root" in range is min.
            // If J(max) < 0, then root is > max. Closest valid is max.

            // To emulate "Optimal Reserve Price" logic where we want the point where J goes positive:
            // If J(min) > 0, it's already positive, so we should return min?
            // If J(max) < 0, it never gets positive, return max? (Or None?)

            // Strictly speaking, RootFinder should find f(x)=0.
            return None;
        }

        for _ in 0..self.max_iterations {
            let c = (a + b) / 2.0;
            let fc = f(c);

            if fc.abs() < self.tolerance || (b - a) / 2.0 < self.tolerance {
                return Some(c);
            }

            if fa * fc < 0.0 {
                b = c;
                // fb is not strictly needed for next iteration logic in this simple implementation
                // but good for completeness if we extended checks.
                // However, to satisfy clippy:
                let _ = fb;
            } else {
                a = c;
                fa = fc;
            }
        }

        Some((a + b) / 2.0)
    }
}
