use super::traits::PharmacokineticModel;

/// Parameters for a standard one-compartment pharmacokinetic model with first-order absorption.
#[derive(Debug, Clone, Copy)]
pub struct PKParameters {
    /// Bioavailability (fraction).
    pub f: f64,
    /// Dose (amount).
    pub d: f64,
    /// Absorption rate constant (1/time).
    pub ka: f64,
    /// Elimination rate constant (1/time).
    pub ke: f64,
    /// Apparent volume of distribution (volume).
    pub v: f64,
}

/// A model representing a single dose with first-order absorption and elimination (Bateman function).
#[derive(Debug, Clone, Copy)]
pub struct BatemanModel {
    pub params: PKParameters,
}

impl BatemanModel {
    pub fn new(params: PKParameters) -> Self {
        Self { params }
    }
}

impl PharmacokineticModel for BatemanModel {
    fn concentration(&self, t: f64) -> f64 {
        if t < 0.0 {
            return 0.0;
        }

        let PKParameters { f, d, ka, ke, v } = self.params;

        // Handle the case where ka is very close to ke
        if (ka - ke).abs() < 1e-9 {
            // Special case for ka = ke
            return (f * d * ka * t / v) * (-ka * t).exp();
        }

        let factor = f * d * ka / (v * (ka - ke));
        factor * ((-ke * t).exp() - (-ka * t).exp())
    }
}

/// Calculates the elimination half-life from the elimination rate constant.
pub fn half_life(ke: f64) -> f64 {
    if ke <= 0.0 {
        return f64::INFINITY;
    }
    std::f64::consts::LN_2 / ke
}

/// Calculates the time to maximum concentration (T_max).
pub fn t_max(ka: f64, ke: f64) -> f64 {
    if ka <= 0.0 || ke <= 0.0 {
        return 0.0; // Invalid input
    }
    if (ka - ke).abs() < 1e-9 {
        1.0 / ke
    } else {
        (ka.ln() - ke.ln()) / (ka - ke)
    }
}

/// Solves for the absorption rate constant (ka) given T_max and ke using Newton's method.
pub fn solve_ka(
    t_max_target: f64,
    ke: f64,
    initial_guess: f64,
    max_iter: u32,
    tolerance: f64,
) -> Option<f64> {
    if t_max_target <= 0.0 || ke <= 0.0 {
        return None;
    }

    let mut ka = initial_guess;
    if ka <= 0.0 {
        ka = ke * 2.0;
    }

    for _ in 0..max_iter {
        if ka <= 0.0 { return None; }

        if (ka - ke).abs() < 1e-9 {
            let t_max_at_ke = 1.0 / ke;
            if (t_max_at_ke - t_max_target).abs() < tolerance {
                return Some(ke);
            }
            ka += 1e-6;
        }

        let current_t_max = t_max(ka, ke);
        let fx = current_t_max - t_max_target;

        if fx.abs() < tolerance {
            return Some(ka);
        }

        let f_prime_ka = (1.0 / ka * (ka - ke) - (ka.ln() - ke.ln())) / (ka - ke).powi(2);

        if f_prime_ka.abs() < 1e-9 {
            return None;
        }

        let next_ka = ka - fx / f_prime_ka;

        if !next_ka.is_finite() {
            return None;
        }
        ka = next_ka;
    }

    None
}
