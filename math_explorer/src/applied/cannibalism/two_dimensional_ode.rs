use crate::pure_math::analysis::ode::OdeSystem;
use nalgebra::Vector2;

/// Parameters for the Cannibalism Model.
///
/// Encapsulates the coefficients for the interaction dynamics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CannibalismParams {
    /// Birth rate of normal individuals
    pub beta_n: f64,
    /// Birth rate contribution from cannibalistic individuals (if any)
    pub beta_c: f64,
    /// Rate at which normal individuals become cannibals
    pub k_n: f64,
    /// Constant loss term for normal individuals due to cannibalism
    pub phi_loss: f64,
    /// Death rate of normal individuals
    pub mu_n: f64,
    /// Death rate of cannibalistic individuals
    pub mu_c: f64,
}

/// The state of the Cannibalism system.
///
/// - `x`: Normal population ($N$)
/// - `y`: Cannibal population ($C$)
pub type CannibalismState = Vector2<f64>;

/// The Cannibalism ODE System.
///
/// Implements `OdeSystem` to allow numerical integration of population dynamics.
#[derive(Debug, Clone)]
pub struct CannibalismModel {
    pub params: CannibalismParams,
}

impl CannibalismModel {
    /// Creates a new Cannibalism Model with the given parameters.
    pub fn new(params: CannibalismParams) -> Self {
        Self { params }
    }
}

impl OdeSystem<CannibalismState> for CannibalismModel {
    fn derivative(&self, _t: f64, state: &CannibalismState) -> CannibalismState {
        let n = state.x;
        let c = state.y;
        let p = &self.params;

        // dN/dt = beta_n * n + beta_c * c - k_n * n - phi - mu_n * n
        let dndt = p.beta_n * n + p.beta_c * c - p.k_n * n - p.phi_loss - p.mu_n * n;

        // dC/dt = k_n * n - mu_c * c
        let dcdt = p.k_n * n - p.mu_c * c;

        CannibalismState::new(dndt, dcdt)
    }
}

// Two-Dimensional ODE Model for Cannibalism

// dN/dt = beta_N(N, C) * N + beta_C(N, C) * C - K(N) * N - phi(N, C) - mu_N(N, C) * N
// dC/dt = K(N) * N - mu_C(N, C) * C

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
#[deprecated(since = "0.1.1", note = "Use CannibalismModel and OdeSystem instead")]
pub fn dndt(n: f64, c: f64, beta_n: f64, beta_c: f64, k_n: f64, phi_n_c: f64, mu_n: f64) -> f64 {
    let params = CannibalismParams {
        beta_n,
        beta_c,
        k_n,
        phi_loss: phi_n_c,
        mu_n,
        mu_c: 0.0, // Irrelevant for dndt
    };
    let model = CannibalismModel::new(params);
    let state = CannibalismState::new(n, c);
    // Vector2 .x access
    model.derivative(0.0, &state).x
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
#[deprecated(since = "0.1.1", note = "Use CannibalismModel and OdeSystem instead")]
pub fn dcdt(n: f64, c: f64, k_n: f64, mu_c: f64) -> f64 {
    let params = CannibalismParams {
        beta_n: 0.0, // Irrelevant
        beta_c: 0.0, // Irrelevant
        k_n,
        phi_loss: 0.0, // Irrelevant
        mu_n: 0.0, // Irrelevant
        mu_c,
    };
    let model = CannibalismModel::new(params);
    let state = CannibalismState::new(n, c);
    // Vector2 .y access
    model.derivative(0.0, &state).y
}
