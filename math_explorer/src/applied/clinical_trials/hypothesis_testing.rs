use statrs::distribution::{ChiSquared, ContinuousCDF, StudentsT};
use std::f64;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub statistic: f64,
    pub p_value: f64,
    pub is_significant: bool,                    // Based on provided alpha
    pub confidence_interval: Option<(f64, f64)>, // For the difference
}

/// Performs an independent two-sample t-test (assuming equal variances).
///
/// # Arguments
/// * `group1` - Data for group 1.
/// * `group2` - Data for group 2.
/// * `alpha` - Significance level (e.g., 0.05).
pub fn t_test_independent(
    group1: &[f64],
    group2: &[f64],
    alpha: f64,
) -> Result<TestResult, String> {
    let n1 = group1.len();
    let n2 = group2.len();

    if n1 < 2 || n2 < 2 {
        return Err("Sample sizes must be at least 2".to_string());
    }

    let mean1 = mean(group1);
    let mean2 = mean(group2);

    let var1 = variance(group1, mean1);
    let var2 = variance(group2, mean2);

    // Pooled variance
    let dof = (n1 + n2 - 2) as f64;
    let pooled_var = ((n1 as f64 - 1.0) * var1 + (n2 as f64 - 1.0) * var2) / dof;
    let pooled_std = pooled_var.sqrt();

    // Standard error of the difference
    let se_diff = pooled_std * (1.0 / n1 as f64 + 1.0 / n2 as f64).sqrt();

    // t-statistic
    let t_stat = (mean1 - mean2) / se_diff;

    // P-value (two-tailed)
    let t_dist = StudentsT::new(0.0, 1.0, dof).map_err(|e| e.to_string())?;
    let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

    // Confidence Interval for the difference
    // diff +/- t_crit * se_diff
    let t_crit = t_dist.inverse_cdf(1.0 - alpha / 2.0);
    let margin = t_crit * se_diff;
    let diff = mean1 - mean2;
    let ci = (diff - margin, diff + margin);

    Ok(TestResult {
        statistic: t_stat,
        p_value,
        is_significant: p_value < alpha,
        confidence_interval: Some(ci),
    })
}

/// Performs a Chi-Square test for a 2x2 contingency table.
///
/// | | Event | No Event |
/// |---|---|---|
/// | Group 1 | a | b |
/// | Group 2 | c | d |
///
/// # Arguments
/// * `a`, `b` - Group 1 counts (Event, No Event).
/// * `c`, `d` - Group 2 counts (Event, No Event).
/// * `alpha` - Significance level.
pub fn chi_square_2x2(a: u32, b: u32, c: u32, d: u32, alpha: f64) -> Result<TestResult, String> {
    let total = (a + b + c + d) as f64;
    if total == 0.0 {
        return Err("Total count is zero".to_string());
    }

    let a_f = a as f64;
    let b_f = b as f64;
    let c_f = c as f64;
    let d_f = d as f64;

    let row1 = a_f + b_f;
    let row2 = c_f + d_f;
    let col1 = a_f + c_f;
    let col2 = b_f + d_f;

    // Expected values
    let e_a = (row1 * col1) / total;
    let e_b = (row1 * col2) / total;
    let e_c = (row2 * col1) / total;
    let e_d = (row2 * col2) / total;

    if e_a == 0.0 || e_b == 0.0 || e_c == 0.0 || e_d == 0.0 {
        return Err("Expected frequencies too low (zero) for Chi-Square".to_string());
    }

    let term_a = (a_f - e_a).powi(2) / e_a;
    let term_b = (b_f - e_b).powi(2) / e_b;
    let term_c = (c_f - e_c).powi(2) / e_c;
    let term_d = (d_f - e_d).powi(2) / e_d;

    let chi_sq = term_a + term_b + term_c + term_d;

    // DoF for 2x2 is 1
    let dist = ChiSquared::new(1.0).map_err(|e| e.to_string())?;
    let p_value = 1.0 - dist.cdf(chi_sq);

    Ok(TestResult {
        statistic: chi_sq,
        p_value,
        is_significant: p_value < alpha,
        confidence_interval: None, // CI for chi-sq statistic is not standard; usually CI for OR/RR is used.
    })
}

fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn variance(data: &[f64], mean: f64) -> f64 {
    let sum_sq_diff: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    sum_sq_diff / (data.len() - 1) as f64
}
