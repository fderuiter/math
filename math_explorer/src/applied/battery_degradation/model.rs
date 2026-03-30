//! Battery degradation models.

use super::types::{Capacity, Cycles, DepthOfDischarge};

/// A trait representing a battery degradation model.
pub trait DegradationModel {
    /// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
    /// for a given depth-of-discharge (DoD).
    fn n70(&self, d: DepthOfDischarge) -> Cycles;

    /// Calculates the remaining battery capacity after a number of cycles.
    fn capacity(&self, n: Cycles, d: DepthOfDischarge) -> Capacity;

    /// Calculates the number of equivalent full cycles to reach a target capacity.
    fn cycles_to_capacity(&self, target: Capacity, d: DepthOfDischarge) -> Cycles;
}

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
}

impl DegradationModel for PowerLawModel {
    /// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
    /// for a given depth-of-discharge (DoD).
    fn n70(&self, d: DepthOfDischarge) -> Cycles {
        let val = self.alpha * d.as_f64().powf(self.beta);
        Cycles::new_clamped(val)
    }

    /// Calculates the remaining battery capacity after a number of cycles.
    fn capacity(&self, n: Cycles, d: DepthOfDischarge) -> Capacity {
        let n70_val = self.n70(d).as_f64();
        if n70_val == 0.0 {
            return Capacity::new_clamped(0.0);
        }
        let exponent = n.as_f64() / n70_val;
        let cap = 0.7_f64.powf(exponent);
        // Clamp to 1.0 just in case floating point error pushes it slightly over for n=0
        Capacity::new_clamped(cap)
    }

    /// Calculates the number of equivalent full cycles to reach a target capacity.
    fn cycles_to_capacity(&self, target: Capacity, d: DepthOfDischarge) -> Cycles {
        let n70_val = self.n70(d).as_f64();
        const LN_0_7: f64 = -0.3566749439387324; // ln(0.7)
        let ln_target = target.as_f64().ln();

        let val = (ln_target / LN_0_7) * n70_val;
        Cycles::new_clamped(val)
    }
}

// Inherent methods for backward compatibility, delegating to trait implementation
impl PowerLawModel {
    /// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
    /// for a given depth-of-discharge (DoD).
    pub fn n70(&self, d: DepthOfDischarge) -> Cycles {
        DegradationModel::n70(self, d)
    }

    /// Calculates the remaining battery capacity after a number of cycles.
    pub fn capacity(&self, n: Cycles, d: DepthOfDischarge) -> Capacity {
        DegradationModel::capacity(self, n, d)
    }

    /// Calculates the number of equivalent full cycles to reach a target capacity.
    pub fn cycles_to_capacity(&self, target: Capacity, d: DepthOfDischarge) -> Cycles {
        DegradationModel::cycles_to_capacity(self, target, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_model_n70() {
        let model = PowerLawModel::standard();

        // Check anchor points from documentation approximately
        // DoD=100%, N70=300
        let n70_100 = model.n70(DepthOfDischarge::new(100.0).unwrap()).as_f64();
        assert!(
            (n70_100 - 300.0).abs() < 50.0,
            "Expected ~300, got {}",
            n70_100
        );

        // DoD=10%, N70=6000
        let n70_10 = model.n70(DepthOfDischarge::new(10.0).unwrap()).as_f64();
        assert!(
            (n70_10 - 6000.0).abs() < 500.0,
            "Expected ~6000, got {}",
            n70_10
        );
    }

    #[test]
    fn test_capacity_decay() {
        let model = PowerLawModel::standard();
        let dod = DepthOfDischarge::new(60.0).unwrap();
        let n70 = model.n70(dod).as_f64();

        // At 0 cycles, capacity should be 1.0
        let cap_0 = model.capacity(Cycles::new(0.0).unwrap(), dod);
        assert!((cap_0.as_f64() - 1.0).abs() < 1e-6);

        // At n70 cycles, capacity should be 0.7
        let cap_n70 = model.capacity(Cycles::new(n70).unwrap(), dod);
        assert!((cap_n70.as_f64() - 0.7).abs() < 1e-6);
    }

    #[test]
    fn test_cycles_to_capacity() {
        let model = PowerLawModel::standard();
        let dod = DepthOfDischarge::new(50.0).unwrap();

        // Target 0.7 capacity -> should return n70
        let cycles = model.cycles_to_capacity(Capacity::new(0.7).unwrap(), dod);
        let n70 = model.n70(dod);
        assert!((cycles.as_f64() - n70.as_f64()).abs() < 1e-6);

        // Target 1.0 capacity -> should be 0 cycles
        let cycles_0 = model.cycles_to_capacity(Capacity::new(1.0).unwrap(), dod);
        assert!(cycles_0.as_f64() < 1e-6);
    }

    #[test]
    fn test_trait_implementation() {
        // This function accepts any implementation of DegradationModel
        fn evaluate_model<M: DegradationModel>(model: &M) -> f64 {
            let d = DepthOfDischarge::new(50.0).unwrap();
            model.n70(d).as_f64()
        }

        let model = PowerLawModel::standard();
        let result = evaluate_model(&model);
        assert!(result > 0.0);
    }
}
