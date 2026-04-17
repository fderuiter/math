// Two-Dimensional ODE Model for Cannibalism

// dN/dt = beta_N(N, C) * N + beta_C(N, C) * C - K(N) * N - phi(N, C) - mu_N(N, C) * N
// dC/dt = K(N) * N - mu_C(N, C) * C

use super::error::CannibalismError;
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
    /// Returns a new builder for constructing a `CannibalismModel`.
    pub fn builder() -> CannibalismModelBuilder {
        CannibalismModelBuilder::default()
    }
}

/// Builder for constructing a `CannibalismModel` with validated parameters.
#[derive(Debug, Clone, Default)]
pub struct CannibalismModelBuilder {
    beta_n: Option<f64>,
    beta_c: Option<f64>,
    k_n: Option<f64>,
    phi_n_c: Option<f64>,
    mu_n: Option<f64>,
    mu_c: Option<f64>,
}

impl CannibalismModelBuilder {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the birth rate of normal individuals ($\beta_N$).
    pub fn beta_n(mut self, beta_n: f64) -> Self {
        self.beta_n = Some(beta_n);
        self
    }

    /// Sets the birth rate contribution from cannibalistic individuals ($\beta_C$).
    pub fn beta_c(mut self, beta_c: f64) -> Self {
        self.beta_c = Some(beta_c);
        self
    }

    /// Sets the rate at which normal individuals transition to cannibals ($k_N$).
    pub fn k_n(mut self, k_n: f64) -> Self {
        self.k_n = Some(k_n);
        self
    }

    /// Sets the constant loss term for normal individuals due to cannibalism ($\phi$).
    pub fn phi_n_c(mut self, phi_n_c: f64) -> Self {
        self.phi_n_c = Some(phi_n_c);
        self
    }

    /// Sets the mortality rate of normal individuals ($\mu_N$).
    pub fn mu_n(mut self, mu_n: f64) -> Self {
        self.mu_n = Some(mu_n);
        self
    }

    /// Sets the mortality rate of cannibalistic individuals ($\mu_C$).
    pub fn mu_c(mut self, mu_c: f64) -> Self {
        self.mu_c = Some(mu_c);
        self
    }

    /// Validates the parameters and builds the `CannibalismModel`.
    ///
    /// # Errors
    ///
    /// Returns a `CannibalismError` if any required parameter is missing,
    /// or if any parameter is negative (all physical rates must be non-negative).
    pub fn build(self) -> Result<CannibalismModel, CannibalismError> {
        let beta_n = self.beta_n.ok_or_else(|| CannibalismError::MissingParameter("beta_n".into()))?;
        let beta_c = self.beta_c.ok_or_else(|| CannibalismError::MissingParameter("beta_c".into()))?;
        let k_n = self.k_n.ok_or_else(|| CannibalismError::MissingParameter("k_n".into()))?;
        let phi_n_c = self.phi_n_c.ok_or_else(|| CannibalismError::MissingParameter("phi_n_c".into()))?;
        let mu_n = self.mu_n.ok_or_else(|| CannibalismError::MissingParameter("mu_n".into()))?;
        let mu_c = self.mu_c.ok_or_else(|| CannibalismError::MissingParameter("mu_c".into()))?;

        if beta_n < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "beta_n".into(), value: beta_n });
        }
        if beta_c < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "beta_c".into(), value: beta_c });
        }
        if k_n < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "k_n".into(), value: k_n });
        }
        if phi_n_c < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "phi_n_c".into(), value: phi_n_c });
        }
        if mu_n < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "mu_n".into(), value: mu_n });
        }
        if mu_c < 0.0 {
            return Err(CannibalismError::InvalidParameter { name: "mu_c".into(), value: mu_c });
        }

        Ok(CannibalismModel {
            beta_n,
            beta_c,
            k_n,
            phi_n_c,
            mu_n,
            mu_c,
        })
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
