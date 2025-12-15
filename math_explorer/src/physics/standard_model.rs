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
pub mod gauge {
    /// Holds the coupling constants for the Standard Model gauge groups.
    ///
    /// * `g1`: Coupling for U(1)_Y (Hypercharge).
    /// * `g2`: Coupling for SU(2)_L (Weak Isospin).
    /// * `gs`: Coupling for SU(3)_C (Strong Color).
    #[derive(Debug, Clone, Copy)]
    pub struct GaugeCouplings {
        pub g1: f64,
        pub g2: f64,
        pub gs: f64,
    }

    impl GaugeCouplings {
        /// Creates a new instance of GaugeCouplings.
        pub fn new(g1: f64, g2: f64, gs: f64) -> Self {
            Self { g1, g2, gs }
        }

        /// Calculates the Weak Mixing Angle (Weinberg Angle) components.
        ///
        /// Returns a tuple `(cos_theta_w, sin_theta_w)`.
        ///
        /// The weak mixing angle $\theta_W$ relates the original gauge bosons (B, W3)
        /// to the physical mass eigenstates (Photon, Z).
        ///
        /// Formula: $\tan \theta_W = g_1 / g_2$
        pub fn weak_mixing_angle(&self) -> (f64, f64) {
            let theta_w = (self.g1 / self.g2).atan();
            (theta_w.cos(), theta_w.sin())
        }
    }
}

/// The Higgs Mechanism explains the origin of mass for gauge bosons via Spontaneous Symmetry Breaking.
pub mod higgs {
    use super::gauge::GaugeCouplings;

    /// Represents the Higgs Potential parameters.
    ///
    /// The potential is given by: $V(\phi) = \mu^2 \phi^\dagger \phi + \lambda (\phi^\dagger \phi)^2$
    ///
    /// For symmetry breaking to occur, `mu2` must be negative.
    #[derive(Debug, Clone, Copy)]
    pub struct HiggsPotential {
        /// The mass parameter squared ($\mu^2$). Must be negative for SSB.
        pub mu2: f64,
        /// The self-interaction coupling ($\lambda$). Must be positive for stability.
        pub lambda: f64,
    }

    impl HiggsPotential {
        /// Calculates the Vacuum Expectation Value (VEV) $v$.
        ///
        /// Formula: $v = \sqrt{-\frac{\mu^2}{\lambda}}$
        ///
        /// # Returns
        /// * `Ok(f64)`: The VEV if $\mu^2 < 0$.
        /// * `Err(String)`: If $\mu^2 \ge 0$, indicating no symmetry breaking.
        pub fn vev(&self) -> Result<f64, String> {
            if self.mu2 >= 0.0 {
                return Err("Symmetry is not broken: mu^2 must be negative.".to_string());
            }
            if self.lambda <= 0.0 {
                return Err("Potential is unstable: lambda must be positive.".to_string());
            }
            Ok((-self.mu2 / self.lambda).sqrt())
        }
    }

    /// Calculates the masses of the W and Z bosons.
    ///
    /// # Arguments
    /// * `vev`: The Vacuum Expectation Value $v$.
    /// * `couplings`: The gauge couplings.
    ///
    /// # Formulas
    /// * $M_W = \frac{1}{2} g_2 v$
    /// * $M_Z = \frac{v}{2} \sqrt{g_1^2 + g_2^2}$
    pub fn boson_masses(vev: f64, couplings: &GaugeCouplings) -> (f64, f64) {
        let m_w = 0.5 * couplings.g2 * vev;
        let m_z = 0.5 * vev * (couplings.g1.powi(2) + couplings.g2.powi(2)).sqrt();
        (m_w, m_z)
    }
}

/// Flavor Physics deals with quark mixing via the CKM matrix.
pub mod flavor {
    use nalgebra::Matrix3;
    use num_complex::Complex;

    /// Constructs the Cabibbo-Kobayashi-Maskawa (CKM) Matrix.
    ///
    /// The CKM matrix describes the mixing between mass eigenstates and weak interaction eigenstates of quarks.
    /// It is parametrized by three mixing angles ($\theta_{12}, \theta_{23}, \theta_{13}$) and one CP-violating phase ($\delta$).
    ///
    /// # Arguments
    /// * `theta12`: Mixing angle $\theta_{12}$ (radians).
    /// * `theta23`: Mixing angle $\theta_{23}$ (radians).
    /// * `theta13`: Mixing angle $\theta_{13}$ (radians).
    /// * `delta`: CP-violating phase $\delta$ (radians).
    ///
    /// # Returns
    /// A 3x3 Complex Matrix representing $V_{CKM}$.
    pub fn construct_ckm(theta12: f64, theta23: f64, theta13: f64, delta: f64) -> Matrix3<Complex<f64>> {
        let c12 = theta12.cos();
        let s12 = theta12.sin();
        let c23 = theta23.cos();
        let s23 = theta23.sin();
        let c13 = theta13.cos();
        let s13 = theta13.sin();

        let phase_pos = Complex::from_polar(1.0, delta);
        let phase_neg = Complex::from_polar(1.0, -delta);

        // Row 1
        let v_ud = Complex::new(c12 * c13, 0.0);
        let v_us = Complex::new(s12 * c13, 0.0);
        let v_ub = Complex::new(s13, 0.0) * phase_neg;

        // Row 2
        // -s12 c23 - c12 s23 s13 e^{i delta}
        let v_cd = Complex::new(-s12 * c23, 0.0) - Complex::new(c12 * s23 * s13, 0.0) * phase_pos;
        // c12 c23 - s12 s23 s13 e^{i delta}
        let v_cs = Complex::new(c12 * c23, 0.0) - Complex::new(s12 * s23 * s13, 0.0) * phase_pos;
        let v_cb = Complex::new(s23 * c13, 0.0);

        // Row 3
        // s12 s23 - c12 c23 s13 e^{i delta}
        let v_td = Complex::new(s12 * s23, 0.0) - Complex::new(c12 * c23 * s13, 0.0) * phase_pos;
        // -c12 s23 - s12 c23 s13 e^{i delta}
        let v_ts = Complex::new(-c12 * s23, 0.0) - Complex::new(s12 * c23 * s13, 0.0) * phase_pos;
        let v_tb = Complex::new(c23 * c13, 0.0);

        Matrix3::new(
            v_ud, v_us, v_ub,
            v_cd, v_cs, v_cb,
            v_td, v_ts, v_tb
        )
    }
}

/// Quantum Chromodynamics (QCD) focuses on the strong interaction and asymptotic freedom.
pub mod qcd {
    /// Calculates the 1-loop running of the strong coupling constant $\alpha_s(Q^2)$.
    ///
    /// QCD exhibits asymptotic freedom, meaning the coupling decreases at higher energy scales.
    ///
    /// # Arguments
    /// * `mu`: The starting energy scale $\mu$ (GeV).
    /// * `alpha_mu`: The value of $\alpha_s$ at scale $\mu$.
    /// * `q`: The target energy scale $Q$ (GeV).
    /// * `nf`: The number of active quark flavors at these scales.
    ///
    /// # Formula
    /// $\alpha_s(Q^2) = \frac{\alpha_s(\mu^2)}{1 + \frac{\alpha_s(\mu^2)}{12\pi} (33 - 2N_f) \ln(Q^2/\mu^2)}$
    ///
    /// # Returns
    /// * `Ok(f64)`: The value of $\alpha_s(Q^2)$.
    /// * `Err(String)`: If an invalid parameter (e.g., negative energy) is provided.
    pub fn running_coupling(mu: f64, alpha_mu: f64, q: f64, nf: f64) -> Result<f64, String> {
        if mu <= 0.0 || q <= 0.0 {
            return Err("Energy scales must be positive.".to_string());
        }
        if alpha_mu <= 0.0 {
            return Err("Coupling constant must be positive.".to_string());
        }

        let beta0 = 33.0 - 2.0 * nf;
        let log_term = (q.powi(2) / mu.powi(2)).ln();
        let denominator = 1.0 + (alpha_mu / (12.0 * std::f64::consts::PI)) * beta0 * log_term;

        if denominator <= 0.0 {
             return Err("Landau pole encountered: coupling diverges at this scale.".to_string());
        }

        Ok(alpha_mu / denominator)
    }
}

/// Neutrino Physics deals with the oscillation of neutrino flavors.
pub mod neutrinos {
    /// Calculates the probability of two-flavor neutrino oscillation $P(\nu_\alpha \to \nu_\beta)$.
    ///
    /// # Arguments
    /// * `theta`: The mixing angle $\theta$ (radians).
    /// * `delta_m2`: The mass-squared difference $\Delta m^2$ (eV^2).
    /// * `l_km`: The baseline distance $L$ (km).
    /// * `e_gev`: The neutrino energy $E$ (GeV).
    ///
    /// # Formula
    /// $P = \sin^2(2\theta) \sin^2\left( 1.27 \frac{\Delta m^2 L}{E} \right)$
    pub fn oscillation_prob(theta: f64, delta_m2: f64, l_km: f64, e_gev: f64) -> f64 {
        let term1 = (2.0 * theta).sin().powi(2);
        // The factor 1.27 comes from conversion of units: 1.27 * (L/km) * (dm^2/eV^2) / (E/GeV)
        let phase = 1.27 * delta_m2 * l_km / e_gev;
        let term2 = phase.sin().powi(2);
        term1 * term2
    }
}

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
