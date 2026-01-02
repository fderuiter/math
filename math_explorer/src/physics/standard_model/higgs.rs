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
