//! # Standard Model of Particle Physics
//!
//! This module implements the mathematical formalism of the Standard Model,
//! covering the Gauge Principle, Spontaneous Symmetry Breaking, Flavor Physics,
//! Quantum Chromodynamics (QCD), and Neutrino Oscillations.
//!
//! The Standard Model is a quantum field theory based on the gauge group
//! SU(3)_C x SU(2)_L x U(1)_Y, describing the strong, weak, and electromagnetic
//! interactions.

/// The Gauge Sector describes the interaction fields and their couplings.
pub mod gauge;

/// The Higgs Mechanism explains the origin of mass for gauge bosons via Spontaneous Symmetry Breaking.
pub mod higgs;

/// Flavor Physics deals with quark mixing via the CKM matrix.
pub mod flavor;

/// Quantum Chromodynamics (QCD) focuses on the strong interaction and asymptotic freedom.
pub mod qcd;

/// Neutrino Physics deals with the oscillation of neutrino flavors.
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

    #[test]
    fn test_ckm_unitarity() {
        let theta12 = 0.2;
        let theta23 = 0.04;
        let theta13 = 0.003;
        let delta = 1.2;

        let v_ckm = flavor::construct_ckm(theta12, theta23, theta13, delta);
        let v_dag = v_ckm.adjoint();
        let identity = v_ckm * v_dag;

        for i in 0..3 {
            for j in 0..3 {
                let val = identity[(i, j)];
                if i == j {
                     assert_relative_eq!(val.re, 1.0, epsilon = 1e-10);
                     assert_relative_eq!(val.im, 0.0, epsilon = 1e-10);
                } else {
                     assert_relative_eq!(val.re, 0.0, epsilon = 1e-10);
                     assert_relative_eq!(val.im, 0.0, epsilon = 1e-10);
                }
            }
        }
    }

    #[test]
    fn test_asymptotic_freedom() {
        // Test that alpha_s decreases as Q increases for Nf < 16
        let mu = 91.2; // Z mass scale
        let alpha_mu = 0.118;
        let nf = 5.0; // Active flavors at this scale

        let q_low = 10.0;
        let q_high = 1000.0;

        let alpha_low = qcd::running_coupling(mu, alpha_mu, q_low, nf).unwrap();
        let alpha_high = qcd::running_coupling(mu, alpha_mu, q_high, nf).unwrap();

        // At lower energy (q_low < mu), coupling should be higher (confinement direction)
        assert!(alpha_low > alpha_mu);

        // At higher energy (q_high > mu), coupling should be lower (asymptotic freedom)
        assert!(alpha_high < alpha_mu);
    }

    #[test]
    fn test_neutrino_oscillation() {
        // Check bounds [0, 1]
        let p = neutrinos::oscillation_prob(0.5, 0.0025, 295.0, 0.6);
        assert!(p >= 0.0 && p <= 1.0);

        // Check zero probability for zero mixing angle
        let p_zero_angle = neutrinos::oscillation_prob(0.0, 0.0025, 295.0, 0.6);
        assert_relative_eq!(p_zero_angle, 0.0);

        // Check zero probability for zero mass difference
        let p_zero_mass = neutrinos::oscillation_prob(0.5, 0.0, 295.0, 0.6);
        assert_relative_eq!(p_zero_mass, 0.0);
    }
}
