//! # Kolmogorov Complexity Approximations
//!
//! This module provides functions for approximating Kolmogorov complexity.

use crate::pure_math::algorithmic_information::geometry::Point2D;
use rug::{Integer, Rational};

/// Approximates the prefix Kolmogorov complexity of a natural number.
/// K(n) <= log2(n) + 2 * log2(log2(n)) + O(1)
pub fn prefix_kolmogorov_approx(n: &Integer) -> f64 {
    if *n == 0 {
        return 1.0;
    }
    let n_abs = n.clone().abs();
    let log_n = n_abs.to_f64().log2();
    if log_n <= 0.0 {
        return 1.0;
    }
    let log_log_n = log_n.log2();
    log_n + 2.0 * log_log_n + 1.0 // O(1) term
}

/// Computes the complexity of the truncated binary expansion of a point.
/// This approximates K(floor(x)_r).
pub fn k_floor_r(point: &Point2D, r: u32) -> f64 {
    let two_r = Rational::from((1u64 << r, 1));
    let x_m = (point[0].clone() * &two_r).trunc().numer().clone();
    let y_m = (point[1].clone() * &two_r).trunc().numer().clone();

    let k_x = prefix_kolmogorov_approx(&x_m);
    let k_y = prefix_kolmogorov_approx(&y_m);

    k_x + k_y
}
