//! Accelerator Physics.
//!
//! Models for Linear Accelerators (Linacs) and beam characteristics.

/// Represents a model for the beam energy variation with current (Beam Loading).
///
/// In a standing wave accelerator, the energy gained by electrons decreases as the beam current increases
/// due to the beam loading effect (extraction of stored energy).
#[derive(Debug, Clone, Copy)]
pub struct BeamLoadingModel {
    /// The zero-current energy (MeV).
    pub nominal_energy: f64,
    /// The specific energy loss per unit current (MeV/mA).
    pub loading_factor: f64,
}

impl BeamLoadingModel {
    /// Creates a new Beam Loading Model.
    pub fn new(nominal_energy: f64, loading_factor: f64) -> Self {
        Self {
            nominal_energy,
            loading_factor,
        }
    }

    /// Returns the standard model for the generic Linac used in legacy calculations.
    ///
    /// E = 5.925 - 0.00808 * I_b
    pub fn standard() -> Self {
        Self {
            nominal_energy: 5.925,
            loading_factor: 0.00808,
        }
    }

    /// Calculates the average energy for a given beam current.
    ///
    /// $$ E = E_0 - S \times I_b $$
    pub fn calculate_energy(&self, beam_current: f64) -> f64 {
        self.nominal_energy - self.loading_factor * beam_current
    }
}

/// Calculates the average energy for the Beam Loading Line.
///
/// $$ E = 5.925 - I_b \times 0.00808 $$
///
/// # Arguments
///
/// * `beam_current` ($I_b$) - Beam current in mA.
///
/// # Returns
///
/// * `f64` - Average energy in MeV.
///
/// # Deprecated
/// Use `BeamLoadingModel::standard().calculate_energy(beam_current)` instead.
pub fn beam_loading_energy(beam_current: f64) -> f64 {
    BeamLoadingModel::standard().calculate_energy(beam_current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_loading_standard() {
        let model = BeamLoadingModel::standard();
        // At 0 current, energy should be 5.925
        assert!((model.calculate_energy(0.0) - 5.925).abs() < 1e-6);

        // At 100 mA
        let expected = 5.925 - 100.0 * 0.00808;
        assert!((model.calculate_energy(100.0) - expected).abs() < 1e-6);
    }

    #[test]
    fn test_legacy_wrapper() {
        assert!((beam_loading_energy(0.0) - 5.925).abs() < 1e-6);
    }
}
