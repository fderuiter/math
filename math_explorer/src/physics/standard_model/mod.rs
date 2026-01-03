//! # Standard Model of Particle Physics
//!
//! This module implements the mathematical formalism of the Standard Model,
//! covering the Gauge Principle, Spontaneous Symmetry Breaking, Flavor Physics,
//! Quantum Chromodynamics (QCD), and Neutrino Oscillations.
//!
//! The Standard Model is a quantum field theory based on the gauge group
//! SU(3)_C x SU(2)_L x U(1)_Y, describing the strong, weak, and electromagnetic
//! interactions.

pub mod gauge;
pub mod higgs;
pub mod flavor;
pub mod qcd;
pub mod neutrinos;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_higgs_boson_masses() {
        // Standard Model approximate values
        let v = 246.0; // GeV
        // Derived from M_Z ~ 91.18, M_W ~ 80.37
        // M_W = 1/2 g2 v => g2 = 2 * M_W / v = 2 * 80.37 / 246 = 0.6534
        // M_Z = v/2 sqrt(g1^2 + g2^2) => 2*M_Z/v = sqrt(...)
        // (2*91.18/246)^2 = g1^2 + g2^2 => g1^2 = (0.7413)^2 - (0.6534)^2 = 0.5495 - 0.4269 = 0.1226 => g1 = 0.35

        let g2 = 0.653;
        let g1 = 0.350;
        let gs = 1.0; // Irrelevant for this test

        let couplings = gauge::GaugeCouplings::new(g1, g2, gs);
        let (_, m_z) = higgs::boson_masses(v, &couplings);

        // Expected Z mass approx 91.18 GeV
        // Let's recompute expectation based on inputs
        let expected_mz = 0.5 * v * (g1.powi(2) + g2.powi(2)).sqrt();
        assert_relative_eq!(m_z, expected_mz, epsilon = 1e-4);

        // Check against rough physical value
        assert!(m_z > 90.0 && m_z < 92.0, "Z mass should be around 91.18 GeV, got {}", m_z);
    }
}
