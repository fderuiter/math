//! Conservation Laws: Mass, Momentum, and Energy.
//!
//! Implements the core Partial Differential Equations (PDEs) of Fluid Dynamics.

use super::types::{FlowState, FluidProperties, SpatialGradients};
use nalgebra::Vector3;

/// Defines the strategy for computing the momentum conservation term.
pub trait MomentumEquation {
    /// Computes the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
    fn compute_time_derivative(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Vector3<f64>;
}

/// Navier-Stokes Momentum Equation (Viscous Flow).
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \nu \nabla^2 \mathbf{u} + \mathbf{g}$$
#[derive(Debug, Clone, Copy, Default)]
pub struct NavierStokes;

impl MomentumEquation for NavierStokes {
    fn compute_time_derivative(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Vector3<f64> {
        let nu = properties.kinematic_viscosity();
        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(gradients.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -gradients.pressure_gradient / rho;

        // Viscous term: nu * del^2 u
        let viscous_term = match gradients.laplacian_velocity {
            Some(laplacian) => laplacian * nu,
            None => Vector3::zeros(),
        };

        // Sum
        convection + pressure_term + viscous_term + body_force_accel
    }
}

/// Euler Momentum Equation (Inviscid Flow).
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \mathbf{g}$$
#[derive(Debug, Clone, Copy, Default)]
pub struct Euler;

impl MomentumEquation for Euler {
    fn compute_time_derivative(
        &self,
        properties: &FluidProperties,
        state: &FlowState,
        gradients: &SpatialGradients,
        body_force_accel: Vector3<f64>,
    ) -> Vector3<f64> {
        let rho = properties.density;

        // Convective term: -(u . del) u
        let convection = -(gradients.velocity_gradient * state.velocity);

        // Pressure term: -(1/rho) grad p
        let pressure_term = -gradients.pressure_gradient / rho;

        convection + pressure_term + body_force_accel
    }
}

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
    velocity_gradient: &nalgebra::Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    laplacian_velocity: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Vector3<f64> {
    let gradients = SpatialGradients::new(
        *velocity_gradient,
        pressure_gradient,
        Some(laplacian_velocity),
    );
    NavierStokes.compute_time_derivative(properties, state, &gradients, body_force_accel)
}

/// Computes the time evolution of velocity based on the Euler Equations (Inviscid).
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \mathbf{g}$$
///
/// Assumes $\mu = 0$.
pub fn euler_time_derivative(
    rho: f64,
    state: &FlowState,
    velocity_gradient: &nalgebra::Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Vector3<f64> {
    // Construct dummy properties with the given rho
    let properties = FluidProperties::new(rho, 0.0); // Viscosity ignored by Euler
    let gradients = SpatialGradients::new(
        *velocity_gradient,
        pressure_gradient,
        None,
    );
    Euler.compute_time_derivative(&properties, state, &gradients, body_force_accel)
}
