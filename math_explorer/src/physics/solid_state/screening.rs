//! Electron-Electron Screening
//!
//! Describes how electric fields are modified by the presence of mobile charge carriers.

/// Calculates the Thomas-Fermi dielectric function \epsilon(q).
///
/// \epsilon(q) = 1 + k_{TF}^2 / q^2
///
/// This approximation is valid for static fields and small wavevectors (q -> 0)
/// in a free electron gas.
pub fn thomas_fermi_dielectric(q: f64, k_tf: f64) -> f64 {
    if q.abs() < 1e-10 {
        // Divergence at q=0 implies infinite screening length for constant potential (perfect shielding).
        return 1e10;
    }
    1.0 + (k_tf.powi(2) / q.powi(2))
}

/// Calculates the screened potential in real space (Yukawa Potential).
///
/// V(r) \propto (e^{-k_{TF} r}) / r
///
/// This represents the potential of a point charge screened by the electron gas.
pub fn yukawa_potential(r: f64, k_tf: f64) -> f64 {
    if r <= 1e-12 {
        return f64::INFINITY;
    }
    (-k_tf * r).exp() / r
}
