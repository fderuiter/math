//! Analytical tools and derived quantities for Fluid Dynamics.

use super::types::{FluidProperties, FlowState};

/// Calculates the Reynolds Number ($Re$).
///
/// $$Re = \frac{\rho u L}{\mu} = \frac{u L}{\nu}$$
///
/// * `properties`: Fluid properties.
/// * `velocity_magnitude`: Characteristic velocity scale $u$.
/// * `characteristic_length`: Characteristic length scale $L$.
pub fn reynolds_number(
    properties: &FluidProperties,
    velocity_magnitude: f64,
    characteristic_length: f64
) -> f64 {
    (properties.density * velocity_magnitude * characteristic_length) / properties.dynamic_viscosity
}

/// Classification of flow regimes based on Reynolds number.
#[derive(Debug, PartialEq, Eq)]
pub enum FlowRegime {
    Laminar,
    Transitional,
    Turbulent,
}

/// Determines the flow regime from the Reynolds number.
///
/// Note: Thresholds are approximate and geometry-dependent (e.g., pipe flow).
/// * $Re < 2000$: Laminar
/// * $2000 \le Re \le 4000$: Transitional
/// * $Re > 4000$: Turbulent
pub fn flow_regime(re: f64) -> FlowRegime {
    if re < 2000.0 {
        FlowRegime::Laminar
    } else if re <= 4000.0 {
        FlowRegime::Transitional
    } else {
        FlowRegime::Turbulent
    }
}

/// Calculates the Bernoulli constant along a streamline for steady, incompressible, inviscid flow.
///
/// $$C = p + \frac{1}{2}\rho v^2 + \rho g h$$
///
/// * `state`: Fluid state ($p, \mathbf{u}$).
/// * `rho`: Fluid density.
/// * `height`: Elevation $h$ relative to a datum.
/// * `gravity`: Gravitational acceleration $g$ (magnitude, usually 9.81).
pub fn bernoulli_constant(state: &FlowState, rho: f64, height: f64, gravity: f64) -> f64 {
    let v_sq = state.velocity.norm_squared();
    state.pressure + 0.5 * rho * v_sq + rho * gravity * height
}

/// Calculates Shear Stress ($\tau$) in a Boundary Layer (Newtonian Fluid).
///
/// $$\tau = \mu \frac{\partial u}{\partial y}$$
///
/// * `mu`: Dynamic viscosity.
/// * `velocity_gradient_normal`: Gradient of velocity perpendicular to the wall ($\frac{\partial u}{\partial y}$).
pub fn shear_stress(mu: f64, velocity_gradient_normal: f64) -> f64 {
    mu * velocity_gradient_normal
}
