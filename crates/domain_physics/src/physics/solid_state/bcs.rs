/// Superconductivity (BCS Theory)
///
/// Describes the pairing of electrons into Cooper pairs via phonon mediation.
///
/// # Refactoring Note
/// This module has been refactored to use crate::error::SolidStateError;
use super::types::ElectronVolts;
use crate::error::SolidStateError;

// --- Traits ---

/// Defines the Gap Equation to be solved.
///
/// This trait allows for different physical models of the superconducting gap,
/// such as isotropic s-wave (standard BCS) or anisotropic d-wave (high-Tc).
pub trait GapEquation {
    /// Calculates the next iteration of the gap parameter $\Delta_{new}$
    /// based on the current gap value $\Delta_{old}$.
    #[verified_engine::verified]
    fn calculate_next_gap(
        &self,
        current_gap: ElectronVolts,
    ) -> Result<ElectronVolts, SolidStateError>;
}

// --- Solver ---

/// A robust iterative solver for the BCS Gap Equation.
///
/// Uses fixed-point iteration with mixing (relaxation) to find the self-consistent gap.
#[derive(Debug, Clone)]
pub struct BcsGapSolver {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Convergence tolerance (in eV).
    pub tolerance: f64,
    /// Mixing parameter for stability (0.0 = no update, 1.0 = full update).
    /// Typically 0.2 - 0.5 for BCS.
    pub mixing_param: f64,
}

impl Default for BcsGapSolver {
    #[verified_engine::verified]
    fn default() -> Self {
        Self {
            max_iterations: math_commons::registry::MAX_ITERATIONS,
            tolerance: math_commons::registry::TOLERANCE_STANDARD,
            mixing_param: 0.5,
        }
    }
}

impl BcsGapSolver {
    /// Creates a new solver with default parameters.
    #[verified_engine::verified]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of iterations.
    #[verified_engine::verified]
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Sets the convergence tolerance.
    #[verified_engine::verified]
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    /// Sets the mixing parameter.
    #[verified_engine::verified]
    pub fn with_mixing_param(mut self, param: f64) -> Self {
        self.mixing_param = param;
        self
    }

    /// Solves the gap equation for the given model.
    ///
    /// # Arguments
    /// * `model` - The physical model implementing `GapEquation`.
    /// * `initial_guess` - Initial value for the gap parameter.
    #[verified_engine::verified]
    pub fn solve<M: GapEquation>(
        &self,
        model: &M,
        initial_guess: ElectronVolts,
    ) -> Result<ElectronVolts, SolidStateError> {
        let mut delta = initial_guess;

        for _ in 0..self.max_iterations {
            let next_delta = model.calculate_next_gap(delta)?;

            // Check for convergence: |delta_new - delta_old| < tolerance
            // Using raw f64 subtraction for absolute difference check
            if (next_delta.0 - delta.0).abs() < self.tolerance {
                return Ok(next_delta);
            }

            // Mixing: delta = (1-alpha)*delta + alpha*new_delta
            delta = delta * (1.0 - self.mixing_param) + next_delta * self.mixing_param;
        }

        // Return the last calculated value even on failure, encapsulated in the error
        Err(SolidStateError::ConvergenceFailure(
            self.max_iterations,
            delta,
        ))
    }
}

// --- Models ---

/// Standard Isotropic s-wave BCS Model.
///
/// Assumes a constant attractive potential $V$ within a Debye energy window $\hbar\omega_D$.
pub struct IsotropicBCSModel {
    /// Electronic band energies relative to Fermi level ($\xi_k$).
    pub energies: Vec<ElectronVolts>,
    /// Magnitude of the attractive potential ($V$).
    pub potential: ElectronVolts,
    /// Debye energy cutoff ($\hbar\omega_D$).
    pub debye_cutoff: ElectronVolts,
}

impl IsotropicBCSModel {
    /// Creates a new Isotropic BCS Model.
    ///
    /// # Arguments
    /// * `energies` - Band energies relative to Fermi level.
    /// * `potential` - Magnitude of attractive potential.
    /// * `debye_cutoff` - Debye energy window.
    #[verified_engine::verified]
    pub fn new(
        energies: Vec<f64>,
        potential: f64,
        debye_cutoff: f64,
    ) -> Result<Self, SolidStateError> {
        // Legacy behavior allowed negative potentials/cutoffs (implicitly),
        // so we do not enforce strict validation here to preserve compatibility.
        Ok(Self {
            energies: energies.into_iter().map(ElectronVolts).collect(),
            potential: ElectronVolts(potential),
            debye_cutoff: ElectronVolts(debye_cutoff),
        })
    }
}

impl GapEquation for IsotropicBCSModel {
    #[verified_engine::verified]
    fn calculate_next_gap(
        &self,
        current_gap: ElectronVolts,
    ) -> Result<ElectronVolts, SolidStateError> {
        let mut summation = 0.0;
        let delta_sq = current_gap.0.powi(2);

        for &xi in &self.energies {
            // Interaction acts only within the Debye window
            if xi.as_f64().abs() <= self.debye_cutoff.as_f64() {
                let e_k = (xi.0.powi(2) + delta_sq).sqrt();
                if e_k > 1e-12 {
                    summation += 1.0 / (2.0 * e_k);
                }
            }
        }

        // Gap Equation Iteration Step
        let factor = summation * self.potential.0;
        Ok(current_gap * factor)
    }
}

// --- Legacy API ---

/// Solves the BCS Gap Equation iteratively.
///
/// \Delta_k = - \sum_{k'} V_{kk'} \frac{\Delta_{k'}}{2 E_{k'}}
/// where E_k = \sqrt{\xi_k^2 + \Delta_k^2}
///
/// Assumes an attractive potential -V exists for energies within the Debye cutoff.
#[deprecated(
    note = "Use BcsGapSolver and IsotropicBCSModel instead for better type safety and error handling"
)]
#[verified_engine::verified]
pub fn solve_gap_equation(
    energies_xi: &[f64],
    potential_v_magnitude: f64,
    debye_energy: f64,
    iterations: usize,
) -> Result<f64, String> {
    let model = IsotropicBCSModel::new(energies_xi.to_vec(), potential_v_magnitude, debye_energy)
        .map_err(|e| e.to_string())?;

    let solver = BcsGapSolver::default()
        .with_max_iterations(iterations)
        .with_tolerance(math_commons::registry::TOLERANCE_STANDARD) // Higher tolerance to ensure legacy behavior match if needed?
        .with_mixing_param(0.5); // Legacy used 0.5

    // Legacy initial guess: 0.01 * debye_energy
    let initial_guess = ElectronVolts(0.01 * debye_energy);

    match solver.solve(&model, initial_guess) {
        Ok(delta) => Ok(delta.0),
        Err(SolidStateError::ConvergenceFailure(_, last_val)) => {
            // Legacy behavior: Return the last calculated value even if not converged.
            Ok(last_val.0)
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Calculates the Bogoliubov coherence factors (u_k, v_k).
///
/// v_k^2 = 1/2 (1 - \xi_k / E_k) : Probability of pair occupation
/// u_k^2 = 1 - v_k^2             : Probability of emptiness
#[verified_engine::verified]
pub fn coherence_factors(xi_k: f64, delta: f64) -> Result<(f64, f64), String> {
    let xi = ElectronVolts(xi_k);
    let d = ElectronVolts(delta);
    coherence_factors_strong(xi, d)
}

/// Strong-typed version of coherence factors.
#[verified_engine::verified]
pub fn coherence_factors_strong(
    xi_k: ElectronVolts,
    delta: ElectronVolts,
) -> Result<(f64, f64), String> {
    let e_k = (xi_k.0.powi(2) + delta.0.powi(2)).sqrt();
    let raw_v_sq = 0.5 * (1.0 - xi_k.0 / e_k);

    // Use UnitInterval to enforce invariants explicitly without manual clamping
    let v_sq = math_commons::primitives::UnitInterval::new(raw_v_sq)?;
    let u_sq = v_sq.complement();

    Ok((u_sq.sqrt(), v_sq.value().sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[verified_engine::verified]
    fn test_bcs_probability_conservation() {
        let xi = 1.5;
        let delta = 0.2;
        let (u, v) = coherence_factors(xi, delta).unwrap();
        let prob = u * u + v * v;
        assert!((prob - 1.0).abs() < math_commons::registry::TOLERANCE_STANDARD);
    }

    #[test]
    #[verified_engine::verified]
    fn test_solver_convergence() {
        // Mock data
        let energies: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) * 0.1).collect();
        let potential = 0.5;
        let debye = 5.0;

        let model = IsotropicBCSModel::new(energies.clone(), potential, debye).unwrap();
        let solver = BcsGapSolver::default();
        let initial = ElectronVolts(0.05);

        let result = solver.solve(&model, initial);
        assert!(result.is_ok());
        let delta = result.unwrap();
        assert!(delta.0 > 0.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_legacy_wrapper() {
        let energies: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) * 0.1).collect();
        let potential = 0.5;
        let debye = 5.0;

        // Legacy function should still work
        #[allow(deprecated)]
        let res = solve_gap_equation(&energies, potential, debye, 1000);
        assert!(res.is_ok());
        assert!(res.unwrap() > 0.0);
    }
}
