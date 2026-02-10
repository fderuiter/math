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
        let mut solver = Euler::default();

        // Use the generic solver strategy instead of manual Euler
        let next_state = solver.solve(&system, 0.0, &current_state, dt);
        let mut new_p_h = next_state[0];

        // Clamp to [0, 1] to handle numerical drift
        new_p_h = new_p_h.clamp(0.0, 1.0);

        Ok(new_p_h)
    }
}
