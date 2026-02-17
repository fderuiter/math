//! # Evolutionary Dynamics (Hawk-Dove)
//!
//! This module models population dynamics where individuals adopt strategies that compete for resources.
//! The classic example implemented here is the **Hawk-Dove Game**.
//!
//! ## 🦅 The Hawk-Dove Game
//!
//! - **Hawk**: Aggressive strategy. Fights for the resource.
//! - **Dove**: Passive strategy. Shares the resource or retreats.
//!
//! The payoff matrix is:
//!
//! | vs | Hawk | Dove |
//! |---|---|---|
//! | **Hawk** | $(V-C)/2$ | $V$ |
//! | **Dove** | $0$ | $V/2$ |
//!
//! Where:
//! - $V$: Value of the resource.
//! - $C$: Cost of injury from fighting.
//!
//! ## 🚀 Quick Start
//!
//! Simulate a population where the cost of fighting is high ($C > V$).
//! In this case, neither strategy is an Evolutionarily Stable Strategy (ESS),
//! and the population should converge to a mixed equilibrium.
//!
//! ```rust
//! use math_explorer::biology::evolution::HawkDovePopulation;
//!
//! // 1. Define the environment
//! // Value = 2.0, Cost = 10.0 (High cost of fighting)
//! let population = HawkDovePopulation::new(2.0, 10.0);
//!
//! // 2. Initial State: Mostly Hawks (90%)
//! let mut hawk_freq = 0.9;
//! let dt = 0.1;
//!
//! // 3. Evolve over time
//! for _ in 0..100 {
//!     hawk_freq = population.update_frequencies(hawk_freq, dt).unwrap();
//! }
//!
//! // 4. Check convergence
//! // Theoretical Equilibrium: p = V/C = 2/10 = 0.2
//! println!("Final Hawk Frequency: {:.3}", hawk_freq);
//! assert!((hawk_freq - 0.2).abs() < 0.05);
//! ```

use crate::applied::game_theory::error::GameTheoryError;
use crate::applied::game_theory::evolutionary::ReplicatorDynamics;
use crate::pure_math::analysis::ode::{Euler, Solver};
use nalgebra::{DMatrix, DVector};

/// Represents a population playing the Hawk-Dove game.
pub struct HawkDovePopulation {
    /// Value of the resource
    pub v: f64,
    /// Cost of injury
    pub c: f64,
}

impl HawkDovePopulation {
    pub fn new(v: f64, c: f64) -> Self {
        Self { v, c }
    }

    /// Constructs the Replicator Dynamics system for this game.
    ///
    /// The payoff matrix is:
    ///
    /// $$
    /// \begin{pmatrix}
    /// (V-C)/2 & V \\
    /// 0 & V/2
    /// \end{pmatrix}
    /// $$
    ///
    /// Row 0 / Col 0: Hawk
    /// Row 1 / Col 1: Dove
    pub fn to_replicator_dynamics(&self) -> Result<ReplicatorDynamics, GameTheoryError> {
        let e_hh = (self.v - self.c) / 2.0;
        let e_hd = self.v;
        let e_dh = 0.0;
        let e_dd = self.v / 2.0;

        let payoff = DMatrix::from_row_slice(2, 2, &[e_hh, e_hd, e_dh, e_dd]);

        ReplicatorDynamics::new(payoff)
    }

    /// Updates the frequency of the Hawk strategy using the Replicator Equation.
    ///
    /// # Arguments
    /// * `hawk_freq` - Current frequency of Hawks ($p_H$). (Dove freq is $1 - p_H$).
    /// * `dt` - Time step.
    ///
    /// # Returns
    /// The new frequency of Hawks.
    pub fn update_frequencies(&self, hawk_freq: f64, dt: f64) -> Result<f64, GameTheoryError> {
        if !(0.0..=1.0).contains(&hawk_freq) {
            return Err(GameTheoryError::InvalidParameter {
                name: "hawk_freq".to_string(),
                value: hawk_freq,
            });
        }

        let p_h = hawk_freq;
        let p_d = 1.0 - p_h;

        let current_state = DVector::from_vec(vec![p_h, p_d]);
        let system = self.to_replicator_dynamics()?;
        let mut solver = Euler::new(&current_state);

        // Use the generic solver strategy instead of manual Euler
        let next_state = solver.solve(&system, 0.0, &current_state, dt);
        let mut new_p_h = next_state[0];

        // Clamp to [0, 1] to handle numerical drift
        new_p_h = new_p_h.clamp(0.0, 1.0);

        Ok(new_p_h)
    }
}
