//! Types for Fluid Dynamics.

use nalgebra::{Matrix3, Vector3};

/// Errors that can occur in fluid dynamics calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidError {
    /// Density must be strictly positive.
    InvalidDensity,
    /// Numerical instability or invalid calculation.
    CalculationError,
}

impl std::fmt::Display for FluidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDensity => write!(f, "Fluid density must be positive"),
            Self::CalculationError => write!(f, "Numerical error in calculation"),
        }
    }
}

impl std::error::Error for FluidError {}

/// Spatial gradients of flow variables.
///
/// Encapsulates the spatial derivatives needed for momentum equations.
#[derive(Debug, Clone, Copy)]
pub struct SpatialGradients {
    /// Jacobian of velocity ($\nabla \mathbf{u}$).
    pub velocity_gradient: Matrix3<f64>,
    /// Gradient of pressure ($\nabla p$).
    pub pressure_gradient: Vector3<f64>,
    /// Laplacian of velocity ($\nabla^2 \mathbf{u}$).
    pub laplacian_velocity: Vector3<f64>,
}

impl Default for SpatialGradients {
    fn default() -> Self {
        Self {
            velocity_gradient: Matrix3::zeros(),
            pressure_gradient: Vector3::zeros(),
            laplacian_velocity: Vector3::zeros(),
        }
    }
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
