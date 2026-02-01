//! Conservation Laws: Mass, Momentum, and Energy.
//!
//! Implements the core Partial Differential Equations (PDEs) of Fluid Dynamics.

use super::types::{FlowState, FluidProperties, SpatialGradients};
use nalgebra::Vector3;

/// Calculates the Material Derivative ($D/Dt$) of a scalar property.
///
/// $$\frac{D\phi}{Dt} = \frac{\partial \phi}{\partial t} + \mathbf{u} \cdot \nabla \phi$$
///
/// * `local_change`: Partial time derivative $\frac{\partial \phi}{\partial t}$.
/// * `velocity`: Flow velocity $\mathbf{u}$.
/// * `gradient`: Spatial gradient $\nabla \phi$.
pub fn material_derivative_scalar(
    local_change: f64,
    velocity: Vector3<f64>,
    gradient: Vector3<f64>,
) -> f64 {
    local_change + velocity.dot(&gradient)
}

/// Calculates the Material Derivative ($D/Dt$) of a vector property (e.g., velocity).
///
/// $$\frac{D\mathbf{A}}{Dt} = \frac{\partial \mathbf{A}}{\partial t} + (\mathbf{u} \cdot \nabla) \mathbf{A}$$
///
/// * `local_change`: Partial time derivative $\frac{\partial \mathbf{A}}{\partial t}$.
/// * `velocity`: Flow velocity $\mathbf{u}$.
/// * `gradient_tensor`: Jacobian matrix representing $\nabla \mathbf{A}$ (where $J_{ij} = \partial A_i / \partial x_j$).
pub fn material_derivative_vector(
    local_change: Vector3<f64>,
    velocity: Vector3<f64>,
    gradient_tensor: &nalgebra::Matrix3<f64>,
) -> Vector3<f64> {
    // (\mathbf{u} \cdot \nabla) \mathbf{A} corresponds to Jacobian * velocity vector
    // J = [ dA_x/dx  dA_x/dy  dA_x/dz ]
    //     [ dA_y/dx  dA_y/dy  dA_y/dz ]
    //     [ ...                   ]
    // Result is J * u
    local_change + gradient_tensor * velocity
}

/// Checks the Continuity Equation (Conservation of Mass) for Incompressible Flow.
///
/// $$\nabla \cdot \mathbf{u} = 0$$
///
/// Returns the divergence of the velocity field. Ideally, this should be zero.
pub fn continuity_divergence(velocity_divergence: f64) -> f64 {
    velocity_divergence
}

// --- Momentum Equation Strategies ---

/// Defines a strategy for calculating flow acceleration based on a momentum equation.
pub trait MomentumEquation {
    /// Calculates the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force: &Vector3<f64>,
    ) -> Vector3<f64>;
}

/// Navier-Stokes Momentum Equation.
///
/// Includes convection, pressure gradient, viscous diffusion, and body forces.
#[derive(Debug, Clone, Copy, Default)]
pub struct NavierStokes;

impl MomentumEquation for NavierStokes {
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force: &Vector3<f64>,
    ) -> Vector3<f64> {
        let nu = properties.kinematic_viscosity();

        // Viscous term: nu * del^2 u
        let viscous_term = gradients.laplacian_velocity * nu;

        calculate_common_terms(properties, state, gradients, body_force) + viscous_term
    }
}

/// Euler Momentum Equation (Inviscid Flow).
///
/// Includes convection, pressure gradient, and body forces. Assumes zero viscosity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Euler;

impl MomentumEquation for Euler {
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force: &Vector3<f64>,
    ) -> Vector3<f64> {
        calculate_common_terms(properties, state, gradients, body_force)
    }
}

/// Helper to calculate terms common to both Euler and Navier-Stokes:
/// Convection, Pressure Gradient, and Body Force.
fn calculate_common_terms(
    properties: &FluidProperties,
    state: &FlowState,
    gradients: &SpatialGradients,
    body_force: &Vector3<f64>,
) -> Vector3<f64> {
    // Convective term: -(u . del) u
    let convection = -(gradients.velocity_gradient * state.velocity);

    // Pressure term: -(1/rho) grad p
    let pressure_term = -gradients.pressure_gradient / properties.density;

    convection + pressure_term + body_force
}

// --- Legacy Functions (Deprecated) ---

/// Computes the time evolution of velocity based on the Navier-Stokes Equations.
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \nu \nabla^2 \mathbf{u} + \mathbf{g}$$
///
/// Returns the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
#[deprecated(
    since = "0.2.0",
    note = "Use `NavierStokes` strategy with `SpatialGradients` instead."
)]
pub fn navier_stokes_time_derivative(
    properties: &FluidProperties,
    state: &FlowState,
    velocity_gradient: &nalgebra::Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    laplacian_velocity: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Vector3<f64> {
    let gradients = SpatialGradients::new(
        *velocity_gradient,
        pressure_gradient,
        laplacian_velocity,
    );
    NavierStokes.calculate_acceleration(properties, state, &gradients, &body_force_accel)
}

/// Computes the time evolution of velocity based on the Euler Equations (Inviscid).
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \mathbf{g}$$
///
/// Assumes $\mu = 0$.
#[deprecated(
    since = "0.2.0",
    note = "Use `Euler` strategy with `SpatialGradients` instead."
)]
pub fn euler_time_derivative(
    rho: f64,
    state: &FlowState,
    velocity_gradient: &nalgebra::Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Vector3<f64> {
    // Construct minimal properties (viscosity ignored by Euler)
    let properties = FluidProperties::new(rho, 0.0);
    let gradients = SpatialGradients::new(
        *velocity_gradient,
        pressure_gradient,
        Vector3::zeros(), // Laplacian irrelevant for Euler
    );
    Euler.calculate_acceleration(&properties, state, &gradients, &body_force_accel)
}
