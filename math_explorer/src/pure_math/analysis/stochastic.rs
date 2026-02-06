//! Stochastic Analysis Solvers.
//!
//! This module provides algorithms for simulating stochastic processes, such as the
//! Gillespie Algorithm (SSA) for discrete-event systems.

use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
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
    fn propensities(&self, state: &State, out: &mut Vec<f64>);

    /// Updates the state according to the reaction that occurred.
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
