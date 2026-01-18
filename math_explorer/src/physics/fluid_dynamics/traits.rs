//! Core traits for Fluid Dynamics.

/// Defines the behavior of a fluid material.
///
/// This trait allows for the implementation of both Newtonian and Non-Newtonian fluids
/// (e.g., Power Law, Bingham Plastic) by abstracting the viscosity model.
pub trait FluidMaterial {
    /// Returns the density of the fluid ($\rho$) in kg/m^3.
    fn density(&self) -> f64;

    /// Returns the dynamic viscosity ($\mu$ or $\eta$) in Pa·s at a given shear rate.
    ///
    /// For Newtonian fluids, this is constant.
    /// For Non-Newtonian fluids, this is the *apparent viscosity*: $\eta(\dot{\gamma}) = \tau / \dot{\gamma}$.
    ///
    /// * `shear_rate`: The magnitude of the shear rate ($\dot{\gamma}$) in 1/s.
    fn dynamic_viscosity(&self, shear_rate: f64) -> f64;

    /// Calculates the kinematic viscosity ($\nu = \mu / \rho$) in m^2/s at a given shear rate.
    fn kinematic_viscosity(&self, shear_rate: f64) -> f64 {
        self.dynamic_viscosity(shear_rate) / self.density()
    }

    /// Calculates the shear stress ($\tau$) in Pa at a given shear rate.
    ///
    /// $$ \tau = \eta(\dot{\gamma}) \cdot \dot{\gamma} $$
    ///
    /// * `shear_rate`: The magnitude of the shear rate ($\dot{\gamma}$) in 1/s.
    fn shear_stress(&self, shear_rate: f64) -> f64 {
        self.dynamic_viscosity(shear_rate) * shear_rate
    }
}
