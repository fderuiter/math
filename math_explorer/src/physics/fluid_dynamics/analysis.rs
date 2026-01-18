//! Analytical tools and derived quantities for Fluid Dynamics.

use super::regimes::{FlowClassifier, FlowRegime, PipeFlowClassifier};
use super::types::FlowState;
use super::traits::FluidMaterial;

/// Calculates the Reynolds Number ($Re$).
///
/// $$Re = \frac{\rho u L}{\mu_{eff}}$$
///
/// For Non-Newtonian fluids, $\mu_{eff}$ is calculated at the characteristic shear rate $\dot{\gamma} = u/L$.
///
/// * `fluid`: Fluid material (Newtonian or Non-Newtonian).
/// * `velocity_magnitude`: Characteristic velocity scale $u$.
/// * `characteristic_length`: Characteristic length scale $L$.
pub fn reynolds_number<F: FluidMaterial + ?Sized>(
    fluid: &F,
    velocity_magnitude: f64,
    characteristic_length: f64,
) -> f64 {
    // Characteristic shear rate approx u / L
    let shear_rate = if characteristic_length > 1e-9 {
        velocity_magnitude / characteristic_length
    } else {
        0.0
    };

    let mu = fluid.dynamic_viscosity(shear_rate);

    if mu <= 0.0 {
        return f64::INFINITY; // Inviscid -> Infinite Re
    }

    (fluid.density() * velocity_magnitude * characteristic_length) / mu
}

/// Determines the flow regime from the Reynolds number using the standard Pipe Flow strategy.
///
/// **Deprecated:** Use `PipeFlowClassifier` or another implementation of `FlowClassifier` directly.
///
/// * $Re < 2000$: Laminar
/// * $2000 \le Re \le 4000$: Transitional
/// * $Re > 4000$: Turbulent
#[deprecated(
    since = "0.2.0",
    note = "Use `regimes::PipeFlowClassifier` for explicit strategy"
)]
pub fn flow_regime(re: f64) -> FlowRegime {
    PipeFlowClassifier.classify(re)
}

/// Calculates the Bernoulli constant along a streamline for steady, incompressible, inviscid flow.
///
/// $$C = p + \frac{1}{2}\rho v^2 + \rho g h$$
///
/// * `state`: Fluid state ($p, \mathbf{u}$).
/// * `fluid`: Fluid material (specifically density $\rho$).
/// * `height`: Elevation $h$ relative to a datum.
/// * `gravity`: Gravitational acceleration $g$ (magnitude, usually 9.81).
pub fn bernoulli_constant<F: FluidMaterial + ?Sized>(
    state: &FlowState,
    fluid: &F,
    height: f64,
    gravity: f64,
) -> f64 {
    let v_sq = state.velocity.norm_squared();
    let rho = fluid.density();
    state.pressure + 0.5 * rho * v_sq + rho * gravity * height
}

/// Calculates Shear Stress ($\tau$) in a Boundary Layer.
///
/// For Newtonian fluids: $\tau = \mu \frac{\partial u}{\partial y}$.
/// For Non-Newtonian fluids: $\tau = \eta(\dot{\gamma}) \dot{\gamma}$.
///
/// * `fluid`: Fluid material.
/// * `velocity_gradient_normal`: Gradient of velocity perpendicular to the wall ($\frac{\partial u}{\partial y}$).
pub fn shear_stress<F: FluidMaterial + ?Sized>(fluid: &F, velocity_gradient_normal: f64) -> f64 {
    fluid.shear_stress(velocity_gradient_normal)
}
