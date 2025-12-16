/// Biochemical Kinetics (Enzymes)
///
/// This module implements enzyme kinetics using the Michaelis-Menten framework.
/// The core equation describes the rate of enzymatic reactions by relating reaction rate $v$
/// to substrate concentration $[S]$.
///
/// $$ v = \frac{V_{max}[S]}{K_m + [S]} $$
///
/// where:
/// - $V_{max}$ is the maximum rate achieved by the system, at maximum (saturating) substrate concentrations.
/// - $K_m$ is the Michaelis constant, representing the substrate concentration at which the reaction rate is half of $V_{max}$.
///
/// Represents an enzymatic reaction with defined kinetic parameters.
pub struct EnzymeReaction {
    /// Maximum reaction rate ($V_{max}$).
    pub v_max: f64,
    /// Michaelis constant ($K_m$), substrate concentration at half $V_{max}$.
    pub k_m: f64,
}

impl EnzymeReaction {
    /// Creates a new EnzymeReaction with given parameters.
    pub fn new(v_max: f64, k_m: f64) -> Result<Self, String> {
        if v_max < 0.0 || k_m < 0.0 {
            return Err("Parameters V_max and K_m must be non-negative.".to_string());
        }
        Ok(Self { v_max, k_m })
    }

    /// Calculates the reaction velocity for a given substrate concentration $[S]$.
    ///
    /// Formula: $v = V_{max} \frac{[S]}{K_m + [S]}$
    ///
    /// # Arguments
    /// * `substrate_conc` - The concentration of the substrate ($[S]$).
    ///
    /// # Returns
    /// The reaction velocity. Returns an error if concentration is negative.
    pub fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String> {
        if substrate_conc < 0.0 {
            return Err("Substrate concentration cannot be negative.".to_string());
        }
        // Use 0.0 to handle potential division by zero if Km=0 and S=0, though physically Km > 0 usually.
        // If Km + S is 0, the rate is undefined/0.
        let denominator = self.k_m + substrate_conc;
        if denominator == 0.0 {
            return Ok(0.0);
        }

        Ok(self.v_max * substrate_conc / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinetics_saturation() {
        // Vmax = 100.0, Km = 5.0
        let enzyme = EnzymeReaction::new(100.0, 5.0).unwrap();
        // Huge substrate concentration
        let s = 1e6;
        let v = enzyme.reaction_velocity(s).unwrap();

        // Should be close to Vmax
        assert!((v - 100.0).abs() < 0.1, "Velocity {} should be close to Vmax 100.0", v);
    }
}
