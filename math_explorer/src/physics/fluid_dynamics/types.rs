//! Types for Fluid Dynamics.

use nalgebra::{Matrix3, Vector3};
use std::fmt;

/// Errors related to fluid dynamics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// Density must be positive.
    InvalidDensity(f64),
    /// Viscosity must be non-negative.
    InvalidViscosity(f64),
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::InvalidDensity(rho) => write!(f, "Invalid density: {} (must be > 0)", rho),
            FluidError::InvalidViscosity(mu) => {
                write!(f, "Invalid dynamic viscosity: {} (must be >= 0)", mu)
            }
        }
    }
}

impl std::error::Error for FluidError {}

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
    pub fn new(density: f64, dynamic_viscosity: f64) -> Result<Self, FluidError> {
        if density <= 0.0 {
            return Err(FluidError::InvalidDensity(density));
        }
        if dynamic_viscosity < 0.0 {
            return Err(FluidError::InvalidViscosity(dynamic_viscosity));
        }
        Ok(Self {
            density,
            dynamic_viscosity,
        })
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

/// Encapsulates spatial derivatives required for momentum equations.
///
/// This Parameter Object groups related gradients to simplify function signatures.
#[derive(Debug, Clone, Copy)]
pub struct SpatialGradients {
    /// Jacobian of velocity ($\nabla \mathbf{u}$).
    pub velocity_gradient: Matrix3<f64>,
    /// Gradient of pressure ($\nabla p$).
    pub pressure_gradient: Vector3<f64>,
    /// Laplacian of velocity ($\nabla^2 \mathbf{u}$).
    pub laplacian_velocity: Vector3<f64>,
}
