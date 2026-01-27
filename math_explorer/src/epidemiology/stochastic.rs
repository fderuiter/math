use crate::epidemiology::compartmental::{SIRModel, SIRState};
use rand::Rng;

/// A trait for systems that can be simulated stochastically.
///
/// Unlike ODE systems which are continuous, stochastic systems define discrete events
/// that occur with specific propensities (rates).
pub trait StochasticSystem<State> {
    /// Returns the propensity (rate) of each reaction in the current state.
    fn propensities(&self, state: &State) -> Vec<f64>;

    /// Updates the state according to the reaction that occurred.
    fn react(&self, state: &mut State, reaction_index: usize);
}

/// A solver for stochastic simulation using the Gillespie Algorithm (SSA).
///
/// It uses the Direct Method to simulate exact stochastic trajectories.
///
/// # Example
/// ```
/// use math_explorer::epidemiology::stochastic::{GillespieSolver, StochasticSystem};
/// use math_explorer::epidemiology::compartmental::{SIRModel, SIRState};
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut solver = GillespieSolver::new(rng);
/// let mut model = SIRModel::new(100.0, 5.0, 0.5, 0.1);
/// let mut time = 0.0;
///
/// // Decouple state from model to avoid borrow checker errors
/// let mut state = model.state;
///
/// // Run for 10 time units
/// while time < 10.0 {
///     let dt = solver.step(&model, &mut state);
///     if dt.is_infinite() { break; }
///     time += dt;
/// }
/// model.state = state;
/// ```
pub struct GillespieSolver<R> {
    rng: R,
}

impl<R: Rng> GillespieSolver<R> {
    /// Creates a new solver with the provided random number generator.
    ///
    /// This allows for deterministic simulations by passing a seeded RNG.
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    /// Performs one step of the Gillespie algorithm.
    ///
    /// Returns the time elapsed for this step.
    /// Returns `f64::INFINITY` if no reactions can occur (total propensity is 0).
    pub fn step<S, State>(&mut self, system: &S, state: &mut State) -> f64
    where
        S: StochasticSystem<State>,
    {
        let rates = system.propensities(state);
        let total_rate: f64 = rates.iter().sum();

        if total_rate <= 0.0 {
            return f64::INFINITY;
        }

        // 1. Determine time step tau
        // r1 in (0, 1]
        let r1: f64 = self.rng.r#gen();
        let r1 = if r1 <= 0.0 { f64::MIN_POSITIVE } else { r1 };
        let tau = -r1.ln() / total_rate;

        // 2. Determine which reaction mu occurred
        let r2: f64 = self.rng.r#gen();
        let threshold = r2 * total_rate;
        let mut cumulative = 0.0;
        let mut reaction_index = 0;

        for (i, &rate) in rates.iter().enumerate() {
            cumulative += rate;
            if cumulative >= threshold {
                reaction_index = i;
                break;
            }
        }
        // Fallback for floating point errors
        if cumulative < threshold {
            reaction_index = rates.len().saturating_sub(1);
        }

        // 3. Update state
        system.react(state, reaction_index);

        tau
    }
}

impl StochasticSystem<SIRState> for SIRModel {
    fn propensities(&self, state: &SIRState) -> Vec<f64> {
        // Reaction 0: Infection (S + I -> 2I)
        // Rate: beta * S * I / N
        let infection_rate = self.beta * state.s * state.i / self.n;

        // Reaction 1: Recovery (I -> R)
        // Rate: gamma * I
        let recovery_rate = self.gamma * state.i;

        vec![infection_rate, recovery_rate]
    }

    fn react(&self, state: &mut SIRState, reaction_index: usize) {
        match reaction_index {
            0 => {
                // Infection: S decreases by 1, I increases by 1
                state.s -= 1.0;
                state.i += 1.0;
            }
            1 => {
                // Recovery: I decreases by 1, R increases by 1
                state.i -= 1.0;
                state.r += 1.0;
            }
            _ => panic!("Invalid reaction index for SIR Model"),
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
        let model = SIRModel::new(n, i0, 2.0, 0.1);

        let mut state = model.state; // Working copy of state
        let initial_s = state.s;
        let initial_i = state.i;

        // Take one step
        let dt = solver.step(&model, &mut state);

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
