//! # Kolmogorov Complexity Approximations
//!
//! This module provides functions for approximating Kolmogorov complexity using upper bounds.
//!
//! Since Kolmogorov complexity is incomputable, we rely on established bounds.
//! For a natural number $n$, a standard prefix-free encoding takes $\log n + 2 \log \log n + O(1)$ bits.

use crate::algorithmic_information::geometry::Point2D;
use rug::{Integer, Rational};

/// Approximates the prefix Kolmogorov complexity of a natural number.
///
/// This uses the upper bound for a prefix-free code of an integer $n$:
/// $$ K(n) \le \log_2 n + 2 \log_2(\log_2 n) + O(1) $$
///
/// This formula accounts for encoding the length of the binary string of $n$
/// to ensure the code is prefix-free (self-delimiting).
///
/// # Arguments
///
/// * `n` - The integer to approximate complexity for.
///
/// # Returns
///
/// The approximate complexity in bits.
///
/// # Example
///
/// ```
/// use oxidize_pure_math::algorithmic_information::kolmogorov::prefix_kolmogorov_approx;
/// use rug::Integer;
///
/// let n = Integer::from(1024);
/// let k = prefix_kolmogorov_approx(&n);
///
/// // log2(1024) = 10
/// // log2(10) ≈ 3.32
/// // Bound ≈ 10 + 2*3.32 + 1 ≈ 17.6
/// assert!(k > 10.0 && k < 20.0);
/// ```
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
///
/// This approximates $K(\lfloor x \rfloor_r)$, where $\lfloor x \rfloor_r$ is the coordinate-wise
/// truncation of the point to precision $2^{-r}$.
///
/// It sums the complexities of the numerators of the truncated coordinates.
///
/// # Arguments
///
/// * `point` - The 2D point.
/// * `r` - The precision bits.
///
/// # Example
///
/// ```
/// use oxidize_pure_math::algorithmic_information::geometry::Point2D;
/// use oxidize_pure_math::algorithmic_information::kolmogorov::k_floor_r;
/// use rug::Rational;
///
/// let p = Point2D::new(Rational::from((3, 1)), Rational::from((4, 1))); // (3, 4)
/// let r = 2; // Precision 1/4
///
/// // At r=2 (denominator 4), 3 becomes 12/4, 4 becomes 16/4.
/// // Numerators are 12 and 16.
/// let complexity = k_floor_r(&p, r);
/// assert!(complexity > 0.0);
/// ```
pub fn k_floor_r(point: &Point2D, r: u32) -> f64 {
    let two_r = Rational::from((1u64 << r, 1));
    let x_m = (point[0].clone() * &two_r).trunc().numer().clone();
    let y_m = (point[1].clone() * &two_r).trunc().numer().clone();

    let k_x = prefix_kolmogorov_approx(&x_m);
    let k_y = prefix_kolmogorov_approx(&y_m);

    k_x + k_y
}
