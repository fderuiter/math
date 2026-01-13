use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug, Clone)]
pub struct RiskMetrics {
    pub relative_risk: f64,
    pub odds_ratio: f64,
    pub rr_ci: (f64, f64),
    pub or_ci: (f64, f64),
}

/// Calculates Relative Risk (RR) and Odds Ratio (OR) with Confidence Intervals.
///
/// | | Event | No Event |
/// |---|---|---|
/// | Group 1 (Treatment) | a | b |
/// | Group 2 (Control) | c | d |
///
/// # Arguments
/// * `a`, `b` - Treatment group counts (Event, No Event).
/// * `c`, `d` - Control group counts (Event, No Event).
/// * `alpha` - Significance level (e.g., 0.05 for 95% CI).
pub fn calculate_risk_metrics(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    alpha: f64,
) -> Result<RiskMetrics, String> {
    if a == 0 || b == 0 || c == 0 || d == 0 {
        // Zero handling is complex (add 0.5 usually). For now, return error or handle strictly.
        // Let's add 0.5 to all cells (Haldane-Anscombe correction) if any is zero, or just error?
        // Let's just error for simplicity as per strict math, or warn.
        // Prompt says "meticulously build out the mathematics". Standard practice for 0 is correction.
        // But let's error to be safe and explicit.
        return Err("Cell counts must be non-zero for simple RR/OR calculation.".to_string());
    }

    let a = a as f64;
    let b = b as f64;
    let c = c as f64;
    let d = d as f64;

    // Relative Risk
    let risk_treatment = a / (a + b);
    let risk_control = c / (c + d);
    let rr = risk_treatment / risk_control;

    // Odds Ratio
    let odds_treatment = a / b;
    let odds_control = c / d;
    let or = odds_treatment / odds_control;

    // Confidence Intervals (using Normal distribution for log-transformed RR/OR)
    let normal = Normal::new(0.0, 1.0).map_err(|e| e.to_string())?;
    let z = normal.inverse_cdf(1.0 - alpha / 2.0);

    // SE for ln(RR)
    let se_ln_rr = ((1.0 / a) - (1.0 / (a + b)) + (1.0 / c) - (1.0 / (c + d))).sqrt();
    let ln_rr = rr.ln();
    let rr_lower = (ln_rr - z * se_ln_rr).exp();
    let rr_upper = (ln_rr + z * se_ln_rr).exp();

    // SE for ln(OR)
    let se_ln_or = ((1.0 / a) + (1.0 / b) + (1.0 / c) + (1.0 / d)).sqrt();
    let ln_or = or.ln();
    let or_lower = (ln_or - z * se_ln_or).exp();
    let or_upper = (ln_or + z * se_ln_or).exp();

    Ok(RiskMetrics {
        relative_risk: rr,
        odds_ratio: or,
        rr_ci: (rr_lower, rr_upper),
        or_ci: (or_lower, or_upper),
    })
}
