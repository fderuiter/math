//! Conservation Laws: Mass, Momentum, and Energy.
//!
//! Implements the core Partial Differential Equations (PDEs) of Fluid Dynamics.

use super::types::{FlowState, FluidError, FluidProperties, SpatialGradients};
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

/// Strategy for computing flow acceleration (momentum conservation).
pub trait MomentumEquation {
    /// Calculates the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError>;
}

/// Navier-Stokes equations for Viscous Flow.
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \nu \nabla^2 \mathbf{u} + \mathbf{g}$$
#[derive(Debug, Clone, Copy, Default)]
pub struct NavierStokes;

impl MomentumEquation for NavierStokes {
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError> {
        if properties.density <= 0.0 {
            return Err(FluidError::InvalidDensity);
        }

        let nu = properties.kinematic_viscosity();
        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(gradients.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -gradients.pressure_gradient / rho;

        // Viscous term: nu * del^2 u
        let viscous_term = gradients.laplacian_velocity * nu;

        Ok(convection + pressure_term + viscous_term + body_force_accel)
    }
}

/// Euler equations for Inviscid Flow.
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \mathbf{g}$$
///
/// Ignores viscosity ($\mu = 0$).
#[derive(Debug, Clone, Copy, Default)]
pub struct Euler;

impl MomentumEquation for Euler {
    fn calculate_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError> {
        if properties.density <= 0.0 {
            return Err(FluidError::InvalidDensity);
        }

        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(gradients.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -gradients.pressure_gradient / rho;

        Ok(convection + pressure_term + body_force_accel)
    }
}
