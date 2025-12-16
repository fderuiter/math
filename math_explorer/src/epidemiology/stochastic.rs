use rand::Rng;

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
}
