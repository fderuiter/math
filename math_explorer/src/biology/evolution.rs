//! Evolutionary Dynamics
//!
//! This module implements Evolutionary Game Theory models, specifically the Hawk-Dove game.
//! It uses the Replicator Equation to model the evolution of strategy frequencies in a population.
//!
//! The change in frequency of a strategy $i$ is given by:
//! $$ \frac{dp_i}{dt} = p_i (f_i(\mathbf{p}) - \phi(\mathbf{p})) $$
//! where $f_i$ is the fitness of strategy $i$ and $\phi$ is the average population fitness.

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

    /// Updates the frequency of the Hawk strategy using the Replicator Equation.
    ///
    /// # Arguments
    /// * `hawk_freq` - Current frequency of Hawks ($p_H$). (Dove freq is $1 - p_H$).
    /// * `dt` - Time step.
    ///
    /// # Returns
    /// The new frequency of Hawks.
    pub fn update_frequencies(&self, hawk_freq: f64, dt: f64) -> Result<f64, String> {
        if !(0.0..=1.0).contains(&hawk_freq) {
            return Err("Frequency must be between 0.0 and 1.0".to_string());
        }

        let p_h = hawk_freq;
        let p_d = 1.0 - p_h;

        // Payoffs
        // E(H, H) = (V-C)/2
        // E(H, D) = V
        // E(D, H) = 0
        // E(D, D) = V/2
        let e_hh = (self.v - self.c) / 2.0;
        let e_hd = self.v;
        let e_dh = 0.0;
        let e_dd = self.v / 2.0;

        // Fitness
        let fit_h = p_h * e_hh + p_d * e_hd;
        let fit_d = p_h * e_dh + p_d * e_dd;

        // Average Fitness
        let phi = p_h * fit_h + p_d * fit_d;

        // Differential: dp_H/dt = p_H (Fitness_H - phi)
        let dp_h = p_h * (fit_h - phi);

        // Update
        let mut new_p_h = p_h + dp_h * dt;

        // Clamp to [0, 1] to handle numerical drift
        new_p_h = new_p_h.clamp(0.0, 1.0);

        Ok(new_p_h)
    }
}
