//! 6. Electron-Phonon Interaction
//!
//! Describes how electrons scatter off lattice vibrations.

use num_complex::Complex;

/// Fröhlich Vertex for electron-phonon interaction.
/// Describes the coupling of electrons to longitudinal optical (LO) phonons in polar crystals.
pub struct FrohlichVertex {
    /// Fröhlich coupling constant (dimensionless).
    pub alpha: f64,
    /// LO Phonon frequency (unused in simple vertex calc but theoretically important).
    pub omega_lo: f64,
}

impl FrohlichVertex {
    pub fn new(alpha: f64, omega_lo: f64) -> Self {
        Self { alpha, omega_lo }
    }

    /// Returns the scattering amplitude M(q).
    /// M_q \propto 1 / q
    pub fn amplitude(&self, q: f64) -> Complex<f64> {
        if q.abs() < 1e-12 {
            return Complex::new(0.0, 0.0);
        }
        // M_q is typically purely imaginary in Fröhlich Hamiltonian formulation
        // |M_q|^2 ~ alpha / q^2
        let val = self.alpha.sqrt() / q;
        Complex::new(0.0, val)
    }
}
