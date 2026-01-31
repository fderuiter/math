//! Types for Fluid Dynamics.

use nalgebra::{Matrix3, Vector3};

/// Errors that can occur during fluid dynamics calculations.
#[derive(Debug, Clone)]
pub enum FluidError {
    InvalidDensity(String),
    NumericalInstability(String),
}

/// Physical properties of the fluid.
#[derive(Debug, Clone, Copy)]
pub struct FluidProperties {
    /// Density ($\rho$) in kg/m^3.
    pub density: f64,
    /// Dynamic viscosity ($\mu$) in Pa·s.
    pub dynamic_viscosity: f64,
}

impl FluidProperties {
    /// Creates a new `FluidProperties`.
    pub fn new(density: f64, dynamic_viscosity: f64) -> Self {
        Self {
            density,
            dynamic_viscosity,
        }
    }

    /// Calculates the kinematic viscosity ($\nu = \mu / \rho$) in m^2/s.
    pub fn kinematic_viscosity(&self) -> f64 {
        self.dynamic_viscosity / self.density
    }

    /// Standard properties for Water at 20°C.
    pub fn water() -> Self {
        Self {
            density: 998.2,
            dynamic_viscosity: 1.002e-3,
        }
    }

    /// Standard properties for Air at 15°C (Sea Level).
    pub fn air() -> Self {
        Self {
            density: 1.225,
            dynamic_viscosity: 1.81e-5,
        }
    }
}

/// Represents the state of a fluid element at a specific point in space and time.
#[derive(Debug, Clone, Copy)]
pub struct FlowState {
    /// Velocity vector ($\mathbf{u}$) in m/s.
    pub velocity: Vector3<f64>,
    /// Pressure ($p$) in Pa.
    pub pressure: f64,
}

impl FlowState {
    pub fn new(velocity: Vector3<f64>, pressure: f64) -> Self {
        Self { velocity, pressure }
    }
}

/// Container for spatial derivatives required by momentum equations.
///
/// Encapsulates the various gradients needed to compute flow acceleration,
/// reducing parameter bloat in solver functions.
#[derive(Debug, Clone, Copy)]
pub struct SpatialGradients {
    /// Jacobian of velocity ($\nabla \mathbf{u}$).
    pub velocity_gradient: Matrix3<f64>,
    /// Gradient of pressure ($\nabla p$).
    pub pressure_gradient: Vector3<f64>,
    /// Laplacian of velocity ($\nabla^2 \mathbf{u}$).
    pub laplacian_velocity: Vector3<f64>,
}

impl SpatialGradients {
    pub fn new(
        velocity_gradient: Matrix3<f64>,
        pressure_gradient: Vector3<f64>,
        laplacian_velocity: Vector3<f64>,
    ) -> Self {
        Self {
            velocity_gradient,
            pressure_gradient,
            laplacian_velocity,
        }
    }
}
