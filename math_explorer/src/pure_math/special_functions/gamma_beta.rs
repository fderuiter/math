use statrs::function::{gamma, beta};

/// Gamma Function $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t} dt$.
///
/// Wraps `statrs` implementation.
pub fn gamma_function(x: f64) -> f64 {
    gamma::gamma(x)
}

/// Beta Function $B(m, n) = \int_0^1 t^{m-1} (1-t)^{n-1} dt$.
///
/// Wraps `statrs` implementation.
pub fn beta_function(m: f64, n: f64) -> f64 {
    beta::beta(m, n)
}

/// Computes factorial using Gamma function.
/// $n! = \Gamma(n+1)$
pub fn factorial(n: u64) -> f64 {
    gamma_function(n as f64 + 1.0)
}
