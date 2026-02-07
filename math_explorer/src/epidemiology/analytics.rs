use crate::epidemiology::error::EpidemiologyError;
use crate::pure_math::analysis::roots::NewtonRaphson;

/// Solves the final size equation for S_inf using Newton-Raphson.
///
/// Equation: $\ln(S_0 / S_\infty) = R_0 (1 - S_\infty / N)$
/// Rearranged for root finding: $f(x) = \ln(S_0 / x) - R_0 (1 - x / N) = 0$
pub fn calculate_final_size(r0: f64, s0: f64, n: f64) -> Result<f64, EpidemiologyError> {
    if r0 <= 0.0 {
        return Err(EpidemiologyError::InvalidParameter {
            name: "R0".to_string(),
            value: r0,
        });
    }

    // Initial guess strategy:
    // If R0 > 1, the final size S_inf is approximately S0 * exp(-R0).
    // If R0 <= 1, the epidemic doesn't take off, S_inf ~ S0.
    let guess = if r0 > 1.0 { s0 * (-r0).exp() } else { s0 };

    // f(x) = ln(S0) - ln(x) - R0 + R0*x/N
    // f'(x) = -1/x + R0/N
    let f = |x: f64| s0.ln() - x.ln() - r0 * (1.0 - x / n);
    let df = |x: f64| -1.0 / x + r0 / n;

    let solver = NewtonRaphson::default();

    // Bounds: (0, N).
    // Although mathematical domain is (0, infinity), physically it's [0, N].
    // And ln(x) requires x > 0.
    let bounds = Some((1e-5, n));

    let root = solver.find_root_with_derivative(f, df, guess, bounds)?;

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_size_high_r0() {
        // R0 = 5.0
        let r0 = 5.0;
        let s0 = 999.0;
        let n = 1000.0;

        let s_inf = calculate_final_size(r0, s0, n).expect("Solver failed");

        // For R0=5, herd immunity threshold is 1 - 1/5 = 0.8 => 80% infected.
        // So S_inf should be small (less than 20% of N).
        assert!(s_inf < n * 0.2, "S_inf should be small for high R0");
        assert!(s_inf > 0.0);
    }
}
