//! Fluid Rheology Models.
//!
//! Includes implementations for Non-Newtonian fluids.

use super::traits::FluidMaterial;

/// Power Law (Ostwald-de Waele) Fluid Model.
///
/// Used for shear-thinning (pseudoplastic, n < 1) or shear-thickening (dilatant, n > 1) fluids.
///
/// $$ \tau = K (\dot{\gamma})^n $$
/// $$ \eta_{apparent} = K (\dot{\gamma})^{n-1} $$
#[derive(Debug, Clone, Copy)]
pub struct PowerLawFluid {
    /// Density ($\rho$) in kg/m^3.
    pub density: f64,
    /// Consistency Index ($K$) in Pa·s^n.
    pub consistency_index: f64,
    /// Flow Behavior Index ($n$). Dimensionless.
    pub flow_behavior_index: f64,
    /// Minimum viscosity cutoff to avoid singularities at zero shear rate (for n < 1).
    pub min_viscosity: f64,
}

impl PowerLawFluid {
    /// Creates a new Power Law Fluid.
    ///
    /// * `density`: Fluid density.
    /// * `k`: Consistency index ($K$).
    /// * `n`: Flow behavior index ($n$).
    /// * `min_viscosity`: Lower bound for viscosity (e.g., 1e-9).
    pub fn new(density: f64, k: f64, n: f64, min_viscosity: f64) -> Self {
        Self {
            density,
            consistency_index: k,
            flow_behavior_index: n,
            min_viscosity,
        }
    }

    /// Standard properties for Human Blood (approximate).
    /// Shear-thinning behavior.
    pub fn human_blood() -> Self {
        Self {
            density: 1060.0,
            consistency_index: 0.04, // Pa·s^n
            flow_behavior_index: 0.85,
            min_viscosity: 0.003, // Asymptotic high-shear viscosity
        }
    }
}

impl FluidMaterial for PowerLawFluid {
    fn density(&self) -> f64 {
        self.density
    }

    fn dynamic_viscosity(&self, shear_rate: f64) -> f64 {
        // Avoid division by zero or negative powers of zero
        let rate = shear_rate.abs().max(1e-9);

        // eta = K * rate^(n-1)
        let viscosity = self.consistency_index * rate.powf(self.flow_behavior_index - 1.0);

        viscosity.max(self.min_viscosity)
    }
}
