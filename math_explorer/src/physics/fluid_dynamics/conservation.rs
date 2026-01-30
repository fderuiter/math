//! Conservation Laws: Mass, Momentum, and Energy.
//!
//! Implements the core Partial Differential Equations (PDEs) of Fluid Dynamics.

use super::error::FluidError;
use super::types::{FlowState, FluidProperties, SpatialGradients};
use nalgebra::{Matrix3, Vector3};

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
    gradient_tensor: &Matrix3<f64>,
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

/// Strategy for calculating momentum conservation (flow acceleration).
///
/// This trait allows for swapping between different flow regimes (e.g., Navier-Stokes, Euler)
/// without changing the integration logic.
pub trait MomentumEquation {
    /// Computes the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
    ///
    /// # Arguments
    /// * `properties` - Fluid density and viscosity.
    /// * `state` - Current velocity and pressure.
    /// * `grads` - Spatial gradients of velocity and pressure.
    /// * `body_force` - External body force (acceleration, e.g., gravity).
    fn compute_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        grads: &SpatialGradients,
        body_force: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError>;
}

/// Navier-Stokes momentum equation for viscous flow.
///
/// Requires the Laplacian of velocity.
#[derive(Debug, Clone, Copy, Default)]
pub struct NavierStokes;

impl MomentumEquation for NavierStokes {
    fn compute_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        grads: &SpatialGradients,
        body_force: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError> {
        if properties.density.abs() < f64::EPSILON {
            return Err(FluidError::ZeroDensity);
        }

        let laplacian = grads
            .laplacian_velocity
            .ok_or(FluidError::MissingLaplacian)?;

        let nu = properties.kinematic_viscosity();
        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(grads.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -grads.pressure_gradient / rho;

        // Viscous term: nu * del^2 u
        let viscous_term = laplacian * nu;

        Ok(convection + pressure_term + viscous_term + body_force)
    }
}

/// Euler momentum equation for inviscid flow.
///
/// Ignores viscosity and velocity Laplacian.
#[derive(Debug, Clone, Copy, Default)]
pub struct Euler;

impl MomentumEquation for Euler {
    fn compute_acceleration(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        grads: &SpatialGradients,
        body_force: Vector3<f64>,
    ) -> Result<Vector3<f64>, FluidError> {
        if properties.density.abs() < f64::EPSILON {
            return Err(FluidError::ZeroDensity);
        }

        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(grads.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -grads.pressure_gradient / rho;

        Ok(convection + pressure_term + body_force)
    }
}

/// Computes the time evolution of velocity based on the Navier-Stokes Equations.
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \nu \nabla^2 \mathbf{u} + \mathbf{g}$$
///
/// Returns the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
///
/// * `properties`: Fluid properties ($\rho, \mu$).
/// * `state`: Current flow state ($\mathbf{u}, p$).
/// * `velocity_gradient`: Jacobian of velocity ($\nabla \mathbf{u}$).
/// * `pressure_gradient`: Gradient of pressure ($\nabla p$).
/// * `laplacian_velocity`: Laplacian of velocity ($\nabla^2 \mathbf{u}$).
/// * `body_force`: External forces (e.g., gravity $\mathbf{g}$). Note: Input is acceleration vector (force per unit mass), or force vector if divided by rho manually.
///   Standard form usually takes body force density $\mathbf{f}$. If $\mathbf{f} = \rho \mathbf{g}$, then term is $\mathbf{g}$.
///   Here we assume `body_force` is $\mathbf{g}$ (acceleration).
pub fn navier_stokes_time_derivative(
    properties: &FluidProperties,
    state: &FlowState,
    velocity_gradient: &Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    laplacian_velocity: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Result<Vector3<f64>, FluidError> {
    let grads = SpatialGradients::new(
        *velocity_gradient,
        pressure_gradient,
        Some(laplacian_velocity),
    );
    NavierStokes.compute_acceleration(properties, state, &grads, body_force_accel)
}

/// Computes the time evolution of velocity based on the Euler Equations (Inviscid).
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \mathbf{g}$$
///
/// Assumes $\mu = 0$.
pub fn euler_time_derivative(
    rho: f64,
    state: &FlowState,
    velocity_gradient: &Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Result<Vector3<f64>, FluidError> {
    // Create dummy properties for Euler (viscosity not used)
    let properties = FluidProperties::new(rho, 0.0);
    let grads = SpatialGradients::new(*velocity_gradient, pressure_gradient, None);
    Euler.compute_acceleration(&properties, state, &grads, body_force_accel)
}
