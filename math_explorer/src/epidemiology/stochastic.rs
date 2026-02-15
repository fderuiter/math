use crate::epidemiology::compartmental::{SIRModel, SIRState};
pub use crate::pure_math::analysis::stochastic::{
    GillespieSolver, StochasticError, StochasticSystem,
};
use rand::Rng;

impl StochasticSystem<SIRState> for SIRModel {
    fn propensities(&self, state: &SIRState, out: &mut Vec<f64>) {
        // Reaction 0: Infection (S + I -> 2I)
        // Rate: beta * S * I / N
        let infection_rate = self.beta * state.s * state.i / self.n;

        // Reaction 1: Recovery (I -> R)
        // Rate: gamma * I
        let recovery_rate = self.gamma * state.i;

        out.push(infection_rate);
        out.push(recovery_rate);
    }

    fn react(&self, state: &mut SIRState, reaction_index: usize) -> Result<(), StochasticError> {
        match reaction_index {
            0 => {
                // Infection: S decreases by 1, I increases by 1
                state.s -= 1.0;
                state.i += 1.0;
                Ok(())
            }
            1 => {
                // Recovery: I decreases by 1, R increases by 1
                state.i -= 1.0;
                state.r += 1.0;
                Ok(())
            }
            _ => Err(StochasticError::InvalidReactionIndex(reaction_index)),
        }
    }
}

/// Calculates the probability of extinction given R0 and initial cases.
///
/// Based on Branching Process theory: $P_{ext} = (1/R_0)^{I_0}$ if $R_0 > 1$.
pub fn probability_of_extinction(r0: f64, initial_cases: f64) -> f64 {
    if r0 <= 1.0 {
        1.0
    } else {
        (1.0 / r0).powf(initial_cases)
    }
}

/// Calculates time to next event for SIR system (Gillespie).
///
/// $\tau = - \ln(U) / (\text{rate}_{infect} + \text{rate}_{recover})$
#[deprecated(
    since = "0.1.0",
    note = "Use GillespieSolver instead for full simulation"
)]
pub fn gillespie_step_time(rate_infect: f64, rate_recover: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let u: f64 = rng.r#gen(); // Uniform (0, 1)

    // Avoid log(0)
    let u = if u == 0.0 { 1e-10 } else { u };

    let total_rate = rate_infect + rate_recover;
    if total_rate == 0.0 {
        return f64::INFINITY;
    }

    -u.ln() / total_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_extinction_probability() {
        let r0 = 0.5;
        let i0 = 10.0;
        assert_eq!(probability_of_extinction(r0, i0), 1.0);

        let r0_high = 2.0;
        let i0_one = 1.0;
        // P = 1/2
        assert!((probability_of_extinction(r0_high, i0_one) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_gillespie_solver_deterministic() {
        // Seeded RNG for deterministic test
        let rng = StdRng::seed_from_u64(42);
        let mut solver = GillespieSolver::new(rng);

        let n = 100.0;
        let i0 = 10.0;
        // High beta, low gamma -> likely infection
        let model = SIRModel::new(n, i0, 2.0, 0.1).unwrap();

        let mut state = model.state; // Working copy of state
        let initial_s = state.s;
        let initial_i = state.i;

        // Take one step
        let dt = solver.step(&model, &mut state).unwrap();

        assert!(dt > 0.0, "Time step should be positive");
        assert!(dt.is_finite());

        // Check conservation of population
        assert_eq!(state.s + state.i + state.r, n);

        // State should have changed by exactly 1 individual (integer steps)
        let s_diff = (state.s - initial_s).abs();
        let i_diff = (state.i - initial_i).abs();

        assert!(s_diff <= 1.0);
        assert!(i_diff >= 1.0); // I changes in both infection and recovery
    }
}
