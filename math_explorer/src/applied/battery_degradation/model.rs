//! Battery degradation models.

use super::types::{Capacity, Cycles, DepthOfDischarge};

/// A power-law based battery degradation model.
///
/// $N_{70}(d) = \alpha \cdot d^\beta$
#[derive(Debug, Clone, Copy)]
pub struct PowerLawModel {
    alpha: f64,
    beta: f64,
}

impl PowerLawModel {
    /// Creates a new `PowerLawModel`.
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }

    /// Returns the standard Li-ion model fitted to experimental data.
    ///
    /// alpha = 1.019e5, beta = -1.2639
    pub fn standard() -> Self {
        Self {
            alpha: 1.019e5,
            beta: -1.2639,
        }
    }

    /// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
    /// for a given depth-of-discharge (DoD).
    pub fn n70(&self, d: DepthOfDischarge) -> Cycles {
        let val = self.alpha * d.as_f64().powf(self.beta);
        Cycles::new(val)
    }

    /// Calculates the remaining battery capacity after a number of cycles.
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
    pub fn cycles_to_capacity(&self, target: Capacity, d: DepthOfDischarge) -> Cycles {
        let n70_val = self.n70(d).as_f64();
        const LN_0_7: f64 = -0.3566749439387324; // ln(0.7)
        let ln_target = target.as_f64().ln();

        let val = (ln_target / LN_0_7) * n70_val;
        Cycles::new(val.max(0.0))
    }
}
