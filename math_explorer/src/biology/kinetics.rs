//! Biochemical Kinetics (Enzymes)
//!
//! This module implements enzyme kinetics using the Michaelis-Menten framework and other models.
//!
//! # Kinetic Models
//!
//! * **Michaelis-Menten**: $v = \frac{V_{max}[S]}{K_m + [S]}$
//! * **Hill Kinetics**: $v = \frac{V_{max}[S]^n}{K_m^n + [S]^n}$

/// Defines the interface for kinetic models.
pub trait KineticsModel {
    /// Calculates the reaction velocity for a given substrate concentration $[S]$.
    fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String>;
}

/// Represents an enzymatic reaction following Michaelis-Menten kinetics.
///
/// $$ v = \frac{V_{max}[S]}{K_m + [S]} $$
pub struct MichaelisMenten {
    /// Maximum reaction rate ($V_{max}$).
    pub v_max: f64,
    /// Michaelis constant ($K_m$).
    pub k_m: f64,
}

impl MichaelisMenten {
    /// Creates a new MichaelisMenten model with given parameters.
    pub fn new(v_max: f64, k_m: f64) -> Result<Self, String> {
        if v_max < 0.0 || k_m < 0.0 {
            return Err("Parameters V_max and K_m must be non-negative.".to_string());
        }
        Ok(Self { v_max, k_m })
    }

    /// Calculates the reaction velocity (Legacy wrapper).
    ///
    /// Delegates to `KineticsModel` implementation.
    pub fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String> {
        <Self as KineticsModel>::reaction_velocity(self, substrate_conc)
    }
}

impl KineticsModel for MichaelisMenten {
    fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String> {
        if substrate_conc < 0.0 {
            return Err("Substrate concentration cannot be negative.".to_string());
        }
        let denominator = self.k_m + substrate_conc;
        if denominator == 0.0 {
            return Ok(0.0);
        }
        Ok(self.v_max * substrate_conc / denominator)
    }
}

/// Represents an enzymatic reaction following Hill kinetics (cooperative binding).
///
/// $$ v = \frac{V_{max}[S]^n}{K_m^n + [S]^n} $$
pub struct HillKinetics {
    /// Maximum reaction rate ($V_{max}$).
    pub v_max: f64,
    /// Michaelis constant ($K_m$).
    pub k_m: f64,
    /// Hill coefficient ($n$). $n>1$ indicates positive cooperativity.
    pub n: f64,
}

impl HillKinetics {
    /// Creates a new HillKinetics model.
    pub fn new(v_max: f64, k_m: f64, n: f64) -> Result<Self, String> {
        if v_max < 0.0 || k_m < 0.0 || n < 0.0 {
            return Err("Parameters V_max, K_m, and n must be non-negative.".to_string());
        }
        Ok(Self { v_max, k_m, n })
    }
}

impl KineticsModel for HillKinetics {
    fn reaction_velocity(&self, substrate_conc: f64) -> Result<f64, String> {
        if substrate_conc < 0.0 {
            return Err("Substrate concentration cannot be negative.".to_string());
        }
        let s_n = substrate_conc.powf(self.n);
        let k_n = self.k_m.powf(self.n);
        let denominator = k_n + s_n;

        if denominator == 0.0 {
            return Ok(0.0);
        }

        Ok(self.v_max * s_n / denominator)
    }
}

// Backward compatibility
#[deprecated(note = "Use MichaelisMenten instead")]
pub type EnzymeReaction = MichaelisMenten;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_original_behavior() {
        #[allow(deprecated)]
        let reaction = EnzymeReaction::new(100.0, 50.0).unwrap();
        // At [S] = Km, v = Vmax / 2
        let v = reaction.reaction_velocity(50.0).unwrap();
        assert!((v - 50.0).abs() < 1e-6);

        // At [S] = 0, v = 0
        let v_0 = reaction.reaction_velocity(0.0).unwrap();
        assert!((v_0 - 0.0).abs() < 1e-6);

        // At very high [S], v -> Vmax
        let v_high = reaction.reaction_velocity(10000.0).unwrap();
        assert!((v_high - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_hill_kinetics() {
        // n=1 should behave like Michaelis-Menten
        let hill_mm = HillKinetics::new(100.0, 50.0, 1.0).unwrap();
        let v_mm = hill_mm.reaction_velocity(50.0).unwrap();
        assert!((v_mm - 50.0).abs() < 1e-6);

        // n=2 (Positive Cooperativity)
        // v = Vmax * S^2 / (Km^2 + S^2)
        // If S = Km, v = Vmax / 2 (still true for Hill)
        let hill_coop = HillKinetics::new(100.0, 50.0, 2.0).unwrap();
        let v_coop_half = hill_coop.reaction_velocity(50.0).unwrap();
        assert!((v_coop_half - 50.0).abs() < 1e-6);

        // If S = 2*Km = 100
        // MM: v = 100 * 100 / (50 + 100) = 10000 / 150 = 66.67
        // Hill(n=2): v = 100 * 100^2 / (50^2 + 100^2) = 100 * 10000 / (2500 + 10000) = 1000000 / 12500 = 80.0
        // Higher velocity due to cooperativity
        let mm = MichaelisMenten::new(100.0, 50.0).unwrap();
        let v_mm_high = mm.reaction_velocity(100.0).unwrap();
        let v_hill_high = hill_coop.reaction_velocity(100.0).unwrap();

        assert!(v_hill_high > v_mm_high);
        assert!((v_hill_high - 80.0).abs() < 1e-6);
    }
}
