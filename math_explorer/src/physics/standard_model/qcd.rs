//! Quantum Chromodynamics (QCD) focuses on the strong interaction and asymptotic freedom.

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
