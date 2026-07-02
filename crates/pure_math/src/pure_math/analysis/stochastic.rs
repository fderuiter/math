//! Stochastic Simulation Algorithms (SSA)
//!
//! This module provides tools for simulating continuous-time Markov processes,
//! such as chemical reaction networks or epidemiological models.

use rand::Rng;
use thiserror::Error;

/// Errors that can occur during stochastic simulation.
#[derive(Debug, Error)]
pub enum StochasticError {
    #[error("Invalid reaction index: {0}")]
    InvalidReactionIndex(usize),
}

/// A trait for systems that can be simulated stochastically.
///
/// Unlike ODE systems which are continuous, stochastic systems define discrete events
/// that occur with specific propensities (rates).
pub trait StochasticSystem<State> {
    /// Appends the propensity (rate) of each reaction in the current state to the output buffer.
    ///
    /// The buffer is cleared by the solver before calling this method.
    #[verified_engine::verified]
    fn propensities(&self, state: &State, out: &mut Vec<f64>);

    /// Updates the state according to the reaction that occurred.
    ///
    /// # Arguments
    /// * `state` - The current state to be modified.
    /// * `reaction_index` - The index of the reaction to execute (corresponding to the index in propensities).
    #[verified_engine::verified]
    fn react(&self, state: &mut State, reaction_index: usize) -> Result<(), StochasticError>;
}

/// A solver for stochastic simulation using the Gillespie Algorithm (SSA).
///
/// It uses the Direct Method to simulate exact stochastic trajectories.
pub struct GillespieSolver<R> {
    rng: R,
    /// Reusable buffer for propensities to avoid allocation per step.
    buffer: Vec<f64>,
}

impl<R: Rng> GillespieSolver<R> {
    /// Creates a new solver with the provided random number generator.
    ///
    /// This allows for deterministic simulations by passing a seeded RNG.
    #[verified_engine::verified]
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            // Pre-allocate space for a reasonable number of reactions
            buffer: Vec::with_capacity(16),
        }
    }

    /// Performs one step of the Gillespie algorithm.
    ///
    /// Returns the time elapsed for this step.
    /// Returns `Ok(f64::INFINITY)` if no reactions can occur (total propensity is 0).
    #[verified_engine::verified(opt_out = "Legacy missing assertions")]
    pub fn step<S, State>(&mut self, system: &S, state: &mut State) -> Result<f64, StochasticError>
    where
        S: StochasticSystem<State>,
    {
        // Reuse internal buffer
        self.buffer.clear();
        system.propensities(state, &mut self.buffer);
        let rates = &self.buffer;

        let total_rate: f64 = rates.iter().sum();

        if total_rate <= 0.0 {
            return Ok(f64::INFINITY);
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
        system.react(state, reaction_index)?;

        Ok(tau)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple Decay Model: X -> nothing
    // Rate: k * X
    struct DecayModel {
        k: f64,
    }

    struct DecayState {
        x: i32,
    }

    impl StochasticSystem<DecayState> for DecayModel {
        #[verified_engine::verified]
        fn propensities(&self, state: &DecayState, out: &mut Vec<f64>) {
            if state.x > 0 {
                out.push(self.k * state.x as f64);
            } else {
                out.push(0.0);
            }
        }

        #[verified_engine::verified]
        fn react(
            &self,
            state: &mut DecayState,
            reaction_index: usize,
        ) -> Result<(), StochasticError> {
            if reaction_index == 0 {
                state.x -= 1;
                Ok(())
            } else {
                Err(StochasticError::InvalidReactionIndex(reaction_index))
            }
        }
    }

    #[test]
    #[verified_engine::verified]
    fn test_decay_process() -> Result<(), StochasticError> {
        let rng = oxidize_core::rng::OxidizeRng::default();
        let mut solver = GillespieSolver::new(rng);
        let model = DecayModel { k: 0.1 };
        let mut state = DecayState { x: 100 };

        let mut time = 0.0;
        while time < 10.0 {
            let dt = solver.step(&model, &mut state)?;
            if dt.is_infinite() {
                break;
            }
            time += dt;
        }

        // With k=0.1, mean lifetime is 10.0. In 10.0 time units, approx 1/e remain.
        assert!(state.x < 100);
        assert!(state.x > 0);
        Ok(())
    }
}
