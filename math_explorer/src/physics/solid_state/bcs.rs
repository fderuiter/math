//! 5. Superconductivity (BCS Theory)
//!
//! Describes the pairing of electrons into Cooper pairs via phonon mediation.

/// Solves the BCS Gap Equation iteratively.
///
/// \Delta_k = - \sum_{k'} V_{kk'} \frac{\Delta_{k'}}{2 E_{k'}}
/// where E_k = \sqrt{\xi_k^2 + \Delta_k^2}
///
/// Assumes an attractive potential -V exists for energies within the Debye cutoff.
pub fn solve_gap_equation(
    energies_xi: &[f64],
    potential_v_magnitude: f64,
    debye_energy: f64,
    iterations: usize
) -> Result<f64, String> {
    // Initial guess for the gap parameter Delta
    let mut delta = 0.01 * debye_energy;

    for _ in 0..iterations {
        let mut summation = 0.0;
        // Sum over all states k'
        for &xi in energies_xi {
            // Interaction acts only within the Debye window
            if xi.abs() <= debye_energy {
                let e_k = (xi.powi(2) + delta.powi(2)).sqrt();
                if e_k > 1e-12 {
                    summation += 1.0 / (2.0 * e_k);
                }
            }
        }

        // New Delta from Gap Equation:
        // Delta = V * Delta * Sum(1/2E)
        // (Assuming V is attractive constant -V_0, equation becomes positive)
        let new_delta = potential_v_magnitude * delta * summation;

        // Simple mixing to stabilize convergence
        delta = 0.5 * delta + 0.5 * new_delta;
    }

    Ok(delta)
}

/// Calculates the Bogoliubov coherence factors (u_k, v_k).
///
/// v_k^2 = 1/2 (1 - \xi_k / E_k) : Probability of pair occupation
/// u_k^2 = 1 - v_k^2             : Probability of emptiness
pub fn coherence_factors(xi_k: f64, delta: f64) -> (f64, f64) {
    let e_k = (xi_k.powi(2) + delta.powi(2)).sqrt();
    let v_sq = 0.5 * (1.0 - xi_k / e_k);
    // Ensure within [0, 1] for numerical safety
    let v_sq = v_sq.clamp(0.0, 1.0);
    let u_sq = 1.0 - v_sq;

    (u_sq.sqrt(), v_sq.sqrt())
}
