//! Type definitions for the Cannibalism Model.

/// Parameters governing the cannibalism population dynamics.
///
/// The model follows the equations:
/// $$ \frac{dN}{dt} = \beta_N N + \beta_C C - k_N N - \phi(N, C) - \mu_N N $$
/// $$ \frac{dC}{dt} = k_N N - \mu_C C $$
///
/// Where:
/// - $N$: Population of Normal individuals
/// - $C$: Population of Cannibalistic individuals
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CannibalismParams {
    /// Birth rate of Normal individuals from Normal parents ($\beta_N$).
    pub beta_n: f64,
    /// Birth rate of Normal individuals from Cannibal parents ($\beta_C$).
    pub beta_c: f64,
    /// Rate at which Normal individuals metamorphose into Cannibals ($k_N$).
    pub k_n: f64,
    /// Cannibalism efficiency coefficient ($\alpha$ in $\phi(N,C) = \alpha NC$).
    pub alpha: f64,
    /// Natural death rate of Normal individuals ($\mu_N$).
    pub mu_n: f64,
    /// Natural death rate of Cannibalistic individuals ($\mu_C$).
    pub mu_c: f64,
}

impl Default for CannibalismParams {
    fn default() -> Self {
        Self {
            beta_n: 0.1,
            beta_c: 0.05,
            k_n: 0.01,
            alpha: 0.001,
            mu_n: 0.05,
            mu_c: 0.1,
        }
    }
}
