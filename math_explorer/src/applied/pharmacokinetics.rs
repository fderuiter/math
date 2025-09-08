// pharmacokinetics.rs

//! This module contains functions and structs for pharmacokinetic modeling,
//! specifically for Adderall as described in the user's request.

#[derive(Debug, Clone, Copy)]
pub struct PKParameters {
    /// Bioavailability (fraction)
    pub f: f64,
    /// Dose (amount)
    pub d: f64,
    /// Absorption rate constant (1/time)
    pub ka: f64,
    /// Elimination rate constant (1/time)
    pub ke: f64,
    /// Apparent volume of distribution (volume)
    pub v: f64,
}

/// Computes the concentration at time t for a single dose using the Bateman function.
///
/// C(t) = (F * D * ka) / (V * (ka - ke)) * (exp(-ke * t) - exp(-ka * t))
///
/// # Arguments
/// * `params` - The pharmacokinetic parameters.
/// * `t` - The time after the dose.
///
/// # Returns
/// The concentration at time `t`.
pub fn concentration_bateman(params: &PKParameters, t: f64) -> f64 {
    if t < 0.0 {
        return 0.0;
    }

    let PKParameters { f, d, ka, ke, v } = *params;

    // Handle the case where ka is very close to ke, which leads to a different solution
    if (ka - ke).abs() < 1e-9 {
        // Special case for ka = ke (equation from literature)
        // C(t) = (F * D * ka * t / V) * exp(-ka * t)
        return (f * d * ka * t / v) * (-ka * t).exp();
    }

    let factor = f * d * ka / (v * (ka - ke));
    factor * ((-ke * t).exp() - (-ka * t).exp())
}

/// Computes the total concentration at time t from multiple doses using superposition.
///
/// C_total(t) = sum_i C(t - t_i) for t >= t_i
///
/// # Arguments
/// * `params` - The pharmacokinetic parameters for a single dose.
/// * `dose_times` - A slice of times at which doses were administered.
/// * `t` - The time at which to calculate the total concentration.
///
/// # Returns
/// The total concentration at time `t`.
pub fn concentration_superposition(params: &PKParameters, dose_times: &[f64], t: f64) -> f64 {
    dose_times
        .iter()
        .map(|&dose_time| {
            if t >= dose_time {
                concentration_bateman(params, t - dose_time)
            } else {
                0.0
            }
        })
        .sum()
}

// --- Derived Parameters and Helper Functions ---

/// Calculates the elimination half-life from the elimination rate constant.
/// t_1/2 = ln(2) / k_e
pub fn half_life(ke: f64) -> f64 {
    if ke <= 0.0 {
        return f64::INFINITY;
    }
    std::f64::consts::LN_2 / ke
}

/// Calculates the time to maximum concentration (T_max).
/// T_max = ln(ka/ke) / (ka - ke)
pub fn t_max(ka: f64, ke: f64) -> f64 {
    if ka <= 0.0 || ke <= 0.0 {
        return 0.0; // Invalid input
    }
    // Handle the case where ka is very close to ke
    if (ka - ke).abs() < 1e-9 {
        // Limiting case for ka -> ke is 1/ke
        1.0 / ke
    } else {
        (ka.ln() - ke.ln()) / (ka - ke)
    }
}

/// Solves for the absorption rate constant (ka) given T_max and ke.
///
/// This function uses Newton's method to find the root of the equation:
/// f(ka) = ln(ka/ke) / (ka - ke) - T_max = 0
///
/// # Arguments
/// * `t_max_target` - The target T_max to solve for.
/// * `ke` - The known elimination rate constant.
/// * `initial_guess` - An initial guess for ka. A value around 1.0 is often reasonable for IR formulations.
/// * `max_iter` - Maximum number of iterations.
/// * `tolerance` - The tolerance for convergence.
///
/// # Returns
/// An `Option<f64>` containing the solved ka, or `None` if it fails to converge.
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
        // Provide a safe fallback guess if initial_guess is invalid
        ka = ke * 2.0;
    }

    for _ in 0..max_iter {
        // Ensure ka remains in a valid domain
        if ka <= 0.0 { return None; }

        // Handle ka being very close to ke, which is a singularity for the derivative
        if (ka - ke).abs() < 1e-9 {
            let t_max_at_ke = 1.0 / ke;
            if (t_max_at_ke - t_max_target).abs() < tolerance {
                return Some(ke);
            }
            // Nudge ka away from ke to avoid singularity in Newton's method
            ka += 1e-6;
        }

        let current_t_max = t_max(ka, ke);
        let fx = current_t_max - t_max_target;

        if fx.abs() < tolerance {
            return Some(ka);
        }

        // Derivative of t_max(ka, ke) w.r.t. ka
        let f_prime_ka = (1.0 / ka * (ka - ke) - (ka.ln() - ke.ln())) / (ka - ke).powi(2);

        if f_prime_ka.abs() < 1e-9 {
            // Derivative is zero or close to it, Newton's method fails.
            // This can happen if t_max_target is not achievable with the given ke.
            return None;
        }

        let next_ka = ka - fx / f_prime_ka;

        if !next_ka.is_finite() {
            return None;
        }
        ka = next_ka;
    }

    None // Failed to converge
}

// --- Formulation and Enantiomer-specific Models ---

/// A model for a drug composed of two enantiomers (e.g., d- and l-amphetamine).
#[derive(Debug, Clone, Copy)]
pub struct EnantiomerModel {
    /// Parameters for the d-enantiomer. The dose `d` should be the total dose of the mixture.
    pub d_params: PKParameters,
    /// Parameters for the l-enantiomer. The dose `d` should be the total dose of the mixture.
    pub l_params: PKParameters,
    /// Fraction of d-enantiomer in the dose (e.g., 0.75 for Adderall).
    pub f_d: f64,
    /// Fraction of l-enantiomer in the dose (e.g., 0.25 for Adderall).
    pub f_l: f64,
}

impl EnantiomerModel {
    /// Calculates the total concentration of both enantiomers at time `t` for a single IR dose.
    pub fn concentration_ir_single_dose(&self, t: f64) -> f64 {
        // Calculate concentration for each enantiomer, scaling the dose by its fraction
        let c_d = concentration_bateman(&PKParameters { d: self.d_params.d * self.f_d, ..self.d_params }, t);
        let c_l = concentration_bateman(&PKParameters { d: self.l_params.d * self.f_l, ..self.l_params }, t);
        c_d + c_l
    }

    /// Calculates the total concentration for multiple IR doses using superposition.
    pub fn concentration_ir_multiple_doses(&self, dose_times: &[f64], t: f64) -> f64 {
        dose_times
            .iter()
            .map(|&dose_time| {
                if t >= dose_time {
                    self.concentration_ir_single_dose(t - dose_time)
                } else {
                    0.0
                }
            })
            .sum()
    }

    /// Calculates the total concentration for a single XR dose using the two-pulse model.
    /// C_XR(t) = f1*C_IR(t;D) + f2*C_IR(t-L;D)
    pub fn concentration_xr_single_dose(&self, lag_time: f64, f1: f64, f2: f64, t: f64) -> f64 {
        // C_IR(t;D) is the concentration from a single IR dose of the total mixture,
        // which is what `concentration_ir_single_dose` calculates.
        let c_total_at_t = self.concentration_ir_single_dose(t);

        let c_total_at_t_minus_l = if t >= lag_time {
            self.concentration_ir_single_dose(t - lag_time)
        } else {
            0.0
        };

        // The total XR concentration is the sum of the two pulses, scaled by their fractions.
        f1 * c_total_at_t + f2 * c_total_at_t_minus_l
    }

    /// Calculates the total concentration for multiple XR doses using superposition.
    pub fn concentration_xr_multiple_doses(&self, dose_times: &[f64], lag_time: f64, f1: f64, f2: f64, t: f64) -> f64 {
        dose_times
            .iter()
            .map(|&dose_time| {
                if t >= dose_time {
                    self.concentration_xr_single_dose(lag_time, f1, f2, t - dose_time)
                } else {
                    0.0
                }
            })
            .sum()
    }
}
