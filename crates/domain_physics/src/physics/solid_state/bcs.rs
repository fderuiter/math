/// Superconductivity (BCS Theory)
///
/// Describes the pairing of electrons into Cooper pairs via phonon mediation.
///
/// # Refactoring Note
/// This module has been refactored to use crate::error::SolidStateError;
use super::types::ElectronVolts;
use crate::error::SolidStateError;

// --- Traits ---

pub trait GapEquation {
    #[verified_engine::verified]
    fn calculate_next_gap(
        &self,
        current_gap: ElectronVolts,
    ) -> Result<ElectronVolts, SolidStateError>;
}

// --- Solver ---

#[derive(Debug, Clone)]
pub struct BcsGapSolver {
    pub max_iterations: usize,
    pub tolerance: f64,
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
    #[verified_engine::verified]
    pub fn new() -> Self {
        Self::default()
    }

    #[verified_engine::verified]
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    #[verified_engine::verified]
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        self.tolerance = tol;
        self
    }

    #[verified_engine::verified]
    pub fn with_mixing_param(mut self, param: f64) -> Self {
        self.mixing_param = param;
        self
    }

    #[verified_engine::verified]
    pub fn solve<M: GapEquation>(
        &self,
        model: &M,
        initial_guess: ElectronVolts,
    ) -> Result<ElectronVolts, SolidStateError> {
        let mut delta = initial_guess;

        for _ in 0..self.max_iterations {
            let next_delta = model.calculate_next_gap(delta)?;

            let diff = (next_delta - delta).abs();
            if (diff / ElectronVolts::new(1.0)) < self.tolerance {
                return Ok(next_delta);
            }

            delta = delta * (1.0 - self.mixing_param) + next_delta * self.mixing_param;
        }

        Err(SolidStateError::ConvergenceFailure(
            self.max_iterations,
            delta,
        ))
    }
}

// --- Models ---

pub struct IsotropicBCSModel {
    pub energies: Vec<ElectronVolts>,
    pub potential: ElectronVolts,
    pub debye_cutoff: ElectronVolts,
}

impl IsotropicBCSModel {
    #[verified_engine::verified]
    pub fn new(
        energies: Vec<f64>,
        potential: f64,
        debye_cutoff: f64,
    ) -> Result<Self, SolidStateError> {
        Ok(Self {
            energies: energies.into_iter().map(ElectronVolts::new).collect(),
            potential: ElectronVolts::new(potential),
            debye_cutoff: ElectronVolts::new(debye_cutoff),
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
        let delta_sq = current_gap.powf(2.0);

        for &xi in &self.energies {
            if xi.abs() <= self.debye_cutoff {
                let e_k = (xi.powf(2.0) + delta_sq).sqrt();
                let e_k_raw = e_k / ElectronVolts::new(1.0);
                if e_k_raw > 1e-12 {
                    summation += 1.0 / (2.0 * e_k_raw);
                }
            }
        }

        let factor = summation * (self.potential / ElectronVolts::new(1.0));
        Ok(current_gap * factor)
    }
}

// --- Legacy API ---

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
        .with_tolerance(math_commons::registry::TOLERANCE_STANDARD)
        .with_mixing_param(0.5);

    let initial_guess = ElectronVolts::new(0.01 * debye_energy);

    match solver.solve(&model, initial_guess) {
        Ok(delta) => Ok(delta / ElectronVolts::new(1.0)),
        Err(SolidStateError::ConvergenceFailure(_, last_val)) => {
            Ok(last_val / ElectronVolts::new(1.0))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[verified_engine::verified]
pub fn coherence_factors(xi_k: f64, delta: f64) -> Result<(f64, f64), String> {
    let xi = ElectronVolts::new(xi_k);
    let d = ElectronVolts::new(delta);
    coherence_factors_strong(xi, d)
}

#[verified_engine::verified]
pub fn coherence_factors_strong(
    xi_k: ElectronVolts,
    delta: ElectronVolts,
) -> Result<(f64, f64), String> {
    let e_k = (xi_k.powf(2.0) + delta.powf(2.0)).sqrt();
    let raw_v_sq = 0.5 * (1.0 - (xi_k / e_k));

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
        let energies: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) * 0.1).collect();
        let potential = 0.5;
        let debye = 5.0;

        let model = IsotropicBCSModel::new(energies.clone(), potential, debye).unwrap();
        let solver = BcsGapSolver::default();
        let initial = ElectronVolts::new(0.05);

        let result = solver.solve(&model, initial);
        assert!(result.is_ok());
        let delta = result.unwrap();
        assert!((delta / ElectronVolts::new(1.0)) > 0.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_legacy_wrapper() {
        let energies: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) * 0.1).collect();
        let potential = 0.5;
        let debye = 5.0;

        #[allow(deprecated)]
        let res = solve_gap_equation(&energies, potential, debye, 1000);
        assert!(res.is_ok());
        assert!(res.unwrap() > 0.0);
    }
}
