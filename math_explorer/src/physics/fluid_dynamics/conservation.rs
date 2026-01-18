//! Conservation Laws: Mass, Momentum, and Energy.
//!
//! Implements the core Partial Differential Equations (PDEs) of Fluid Dynamics.

use super::traits::FluidMaterial;
use super::types::FlowState;
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

/// Computes the time evolution of velocity based on the Navier-Stokes Equations.
///
/// $$\frac{\partial \mathbf{u}}{\partial t} = -(\mathbf{u} \cdot \nabla)\mathbf{u} - \frac{1}{\rho}\nabla p + \nu \nabla^2 \mathbf{u} + \mathbf{g}$$
///
/// Note: For Non-Newtonian fluids, this assumes the Generalized Newtonian Fluid model where
/// the viscous term uses an effective kinematic viscosity calculated from the local shear rate magnitude.
/// The shear rate is approximated here from the Frobenius norm of the velocity gradient tensor.
///
/// Returns the local acceleration $\frac{\partial \mathbf{u}}{\partial t}$.
///
/// * `fluid`: Fluid material (Newtonian or Non-Newtonian).
/// * `state`: Current flow state ($\mathbf{u}, p$).
/// * `velocity_gradient`: Jacobian of velocity ($\nabla \mathbf{u}$).
/// * `pressure_gradient`: Gradient of pressure ($\nabla p$).
/// * `laplacian_velocity`: Laplacian of velocity ($\nabla^2 \mathbf{u}$).
/// * `body_force`: External forces (e.g., gravity $\mathbf{g}$).
pub fn navier_stokes_time_derivative<F: FluidMaterial + ?Sized>(
    fluid: &F,
    state: &FlowState,
    velocity_gradient: &nalgebra::Matrix3<f64>,
    pressure_gradient: Vector3<f64>,
    laplacian_velocity: Vector3<f64>,
    body_force_accel: Vector3<f64>,
) -> Vector3<f64> {
    // Estimate shear rate magnitude from velocity gradient tensor (Frobenius norm)
    // This is a simplification; full stress tensor would require Rate of Strain tensor.
    // D = 0.5 * (grad_u + grad_u^T)
    // gamma_dot = sqrt(2 * D:D)
    // For now, we use the Frobenius norm of the gradient as a proxy for the characteristic rate scale.
    let shear_rate = velocity_gradient.norm();

    let nu = fluid.kinematic_viscosity(shear_rate);
    let rho = fluid.density();

    // Convective term: -(u . del) u
    let convection = -(velocity_gradient * state.velocity);

    // Pressure term: -(1/rho) grad p
    let pressure_term = -pressure_gradient / rho;

    // Viscous term: nu * del^2 u
    // strictly speaking, for variable viscosity, the term is div(nu * grad u) + ...
    // but this function assumes standard form with effective viscosity
    let viscous_term = laplacian_velocity * nu;

    // Sum
    convection + pressure_term + viscous_term + body_force_accel
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
    // Convective term: -(u . del) u
    let convection = -(velocity_gradient * state.velocity);

    // Pressure term: -(1/rho) grad p
    let pressure_term = -pressure_gradient / rho;

    convection + pressure_term + body_force_accel
}
