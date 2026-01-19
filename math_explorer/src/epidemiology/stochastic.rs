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
///
/// # Arguments
/// * `rng`: Random number generator.
/// * `rate_infect`: Infection rate.
/// * `rate_recover`: Recovery rate.
pub fn gillespie_step_time<R: Rng + ?Sized>(rng: &mut R, rate_infect: f64, rate_recover: f64) -> f64 {
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
    use rand::SeedableRng;
    use rand::rngs::StdRng;

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

    #[test]
    fn test_gillespie_determinism() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let t1 = gillespie_step_time(&mut rng1, 0.5, 0.1);
        let t2 = gillespie_step_time(&mut rng2, 0.5, 0.1);

        assert_eq!(t1, t2, "Seeded RNG should produce deterministic results");
    }
}
