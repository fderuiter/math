//! Battery degradation models.

use super::types::{Capacity, Cycles, DepthOfDischarge};

/// A power-law based battery degradation model.
///
/// This model predicts cycle life based on the Depth of Discharge (DoD) using the equation:
///
/// $$ N_{70}(d) = \alpha \cdot d^\beta $$
///
/// Where:
/// * $N_{70}$ is the number of cycles to 70% capacity.
/// * $d$ is the Depth of Discharge (in percent).
/// * $\alpha$ and $\beta$ are empirical constants derived from curve fitting.
///
/// # Example
///
/// ```rust
/// use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge};
///
/// // Create a standard model
/// let model = PowerLawModel::standard();
///
/// // Create a custom model (e.g., hypothetical "SuperBattery" with better constants)
/// let custom_model = PowerLawModel::new(2.0e5, -1.1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PowerLawModel {
    alpha: f64,
    beta: f64,
}

impl PowerLawModel {
    /// Creates a new `PowerLawModel` with custom constants.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The scaling factor (intercept).
    /// * `beta` - The power exponent (decay rate).
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }

    /// Returns the standard Li-ion model fitted to experimental data.
    ///
    /// This model is calibrated to typical Lithium-Ion chemistry behavior:
    /// * $\alpha \approx 1.019 \times 10^5$
    /// * $\beta \approx -1.2639$
    pub fn standard() -> Self {
        Self {
            alpha: 1.019e5,
            beta: -1.2639,
        }
    }

    /// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
    /// for a given depth-of-discharge (DoD).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge};
    /// let model = PowerLawModel::standard();
    /// let dod = DepthOfDischarge::new(80.0);
    ///
    /// let life = model.n70(dod);
    /// println!("Cycles to 70% capacity: {}", life);
    /// ```
    pub fn n70(&self, d: DepthOfDischarge) -> Cycles {
        let val = self.alpha * d.as_f64().powf(self.beta);
        Cycles::new(val)
    }

    /// Calculates the remaining battery capacity after a number of cycles.
    ///
    /// This assumes an exponential decay from 100% capacity towards 0%, scaled such
    /// that it passes through 70% at $N = N_{70}$.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge, Cycles};
    /// let model = PowerLawModel::standard();
    /// let dod = DepthOfDischarge::new(50.0);
    /// let cycles_used = Cycles::new(1000.0);
    ///
    /// let current_cap = model.capacity(cycles_used, dod);
    /// println!("Current Health: {}", current_cap);
    /// ```
    pub fn capacity(&self, n: Cycles, d: DepthOfDischarge) -> Capacity {
        let n70_val = self.n70(d).as_f64();
        if n70_val == 0.0 {
            return Capacity::new(0.0);
        }
        let exponent = n.as_f64() / n70_val;
        let cap = 0.7_f64.powf(exponent);
        // Clamp to 1.0 just in case floating point error pushes it slightly over for n=0
        Capacity::new(cap.min(1.0))
    }

    /// Calculates the number of equivalent full cycles to reach a target capacity.
    ///
    /// This is the inverse of the `capacity` function.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge, Capacity};
    /// let model = PowerLawModel::standard();
    /// let dod = DepthOfDischarge::new(50.0);
    /// let target = Capacity::new(0.8); // When will it hit 80%?
    ///
    /// let cycles = model.cycles_to_capacity(target, dod);
    /// ```
    pub fn cycles_to_capacity(&self, target: Capacity, d: DepthOfDischarge) -> Cycles {
        let n70_val = self.n70(d).as_f64();
        const LN_0_7: f64 = -0.3566749439387324; // ln(0.7)
        let ln_target = target.as_f64().ln();

        let val = (ln_target / LN_0_7) * n70_val;
        Cycles::new(val.max(0.0))
    }
}
