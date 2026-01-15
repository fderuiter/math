// Two-Dimensional ODE Model for Cannibalism

// dN/dt = beta_N(N, C) * N + beta_C(N, C) * C - K(N) * N - phi(N, C) - mu_N(N, C) * N
// dC/dt = K(N) * N - mu_C(N, C) * C

use crate::pure_math::analysis::ode::OdeSystem;
use nalgebra::Vector2;

/// Parameters for the Cannibalism ODE system.
///
/// Encapsulates the coefficients for the 2D population model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CannibalismModel {
    /// Birth rate of normal individuals ($\beta_N$)
    pub beta_n: f64,
    /// Birth rate contribution from cannibalistic individuals ($\beta_C$)
    pub beta_c: f64,
    /// Rate at which normal individuals transition to cannibals ($k_N$)
    pub k_n: f64,
    /// Constant loss term for normal individuals due to cannibalism ($\phi$)
    pub phi_n_c: f64,
    /// Mortality rate of normal individuals ($\mu_N$)
    pub mu_n: f64,
    /// Mortality rate of cannibalistic individuals ($\mu_C$)
    pub mu_c: f64,
}

impl CannibalismModel {
    /// Creates a new `CannibalismModel` with the specified parameters.
    pub fn new(beta_n: f64, beta_c: f64, k_n: f64, phi_n_c: f64, mu_n: f64, mu_c: f64) -> Self {
        Self {
            beta_n,
            beta_c,
            k_n,
            phi_n_c,
            mu_n,
            mu_c,
        }
    }
}

/// Implementation of the ODE System trait.
///
/// State vector components:
/// - index 0: Normal population ($N$)
/// - index 1: Cannibal population ($C$)
impl OdeSystem<Vector2<f64>> for CannibalismModel {
    fn derivative(&self, _t: f64, state: &Vector2<f64>) -> Vector2<f64> {
        let n = state[0];
        let c = state[1];

        let dndt = self.beta_n * n + self.beta_c * c - self.k_n * n - self.phi_n_c - self.mu_n * n;
        let dcdt = self.k_n * n - self.mu_c * c;

        Vector2::new(dndt, dcdt)
    }
}

/// Placeholder function for the rate of change of normal individuals.
///
/// # Arguments
///
/// * `n` - number of normal individuals
/// * `c` - number of cannibalistic individuals
/// * `beta_n` - birth rate of normal individuals
/// * `beta_c` - birth rate of cannibalistic individuals
/// * `k_n` - rate at which normal individuals become cannibals
/// * `phi_n_c` - loss of normal individuals due to cannibalism
/// * `mu_n` - death rate of normal individuals
///
/// # Returns
///
/// The rate of change of the number of normal individuals.
#[deprecated(
    since = "0.2.0",
    note = "Use `CannibalismModel` and `OdeSystem` instead."
)]
pub fn dndt(n: f64, c: f64, beta_n: f64, beta_c: f64, k_n: f64, phi_n_c: f64, mu_n: f64) -> f64 {
    // This is a placeholder implementation.
    beta_n * n + beta_c * c - k_n * n - phi_n_c - mu_n * n
}

/// Placeholder function for the rate of change of cannibalistic individuals.
///
/// # Arguments
///
/// * `n` - number of normal individuals
/// * `c` - number of cannibalistic individuals
/// * `k_n` - rate at which normal individuals become cannibals
/// * `mu_c` - death rate of cannibalistic individuals
///
/// # Returns
///
/// The rate of change of the number of cannibalistic individuals.
#[deprecated(
    since = "0.2.0",
    note = "Use `CannibalismModel` and `OdeSystem` instead."
)]
pub fn dcdt(n: f64, c: f64, k_n: f64, mu_c: f64) -> f64 {
    // This is a placeholder implementation.
    k_n * n - mu_c * c
}
