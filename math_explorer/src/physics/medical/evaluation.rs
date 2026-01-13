//! Evaluation Metrics for Medical Physics and Radar.
//!
//! Provides statistical tools to validate respiratory monitoring signals and tracking accuracy against clinical standards.
//! Also includes Plan Evaluation metrics like DVH and TCP.

use std::f64;

/// Calculates the Cumulative Dose-Volume Histogram (DVH).
///
/// The DVH summarizes the 3D dose distribution into a 2D graph showing how much volume
/// receives at least a certain dose.
///
/// # Arguments
///
/// * `doses` - A slice of dose values for all voxels in the structure of interest.
///
/// # Returns
///
/// * `Vec<(f64, f64)>` - A sorted vector of (Dose, Normalized Volume) pairs.
///   - Dose: The dose bin.
///   - Normalized Volume: Fraction of total volume (0.0 to 1.0) receiving >= Dose.
pub fn calculate_dvh(doses: &[f64]) -> Vec<(f64, f64)> {
    if doses.is_empty() {
        return Vec::new();
    }

    let mut sorted_doses = doses.to_vec();
    // Sort descending to easily compute cumulative count
    sorted_doses.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let total_voxels = sorted_doses.len() as f64;
    let mut dvh = Vec::new();

    // We can create bins or just return the curve at every point.
    // For a precise curve, we return the step function at every dose value.
    // V(D) is the fraction of voxels with dose >= D.

    for (i, &dose) in sorted_doses.iter().enumerate() {
        // i + 1 is the number of voxels with dose >= sorted_doses[i]
        // because we sorted descending.
        let vol = (i as f64 + 1.0) / total_voxels;
        dvh.push((dose, vol));
    }

    // Reverse back to ascending dose for standard plotting conventions,
    // but the prompt just asks for "pairs representing the Cumulative DVH".
    // Usually DVH is plotted with Dose on X (ascending).
    // If we list (Dose, Volume), and Doses are descending, the vector is ordered by decreasing X.
    // Let's sort by Dose ascending for the return value.
    dvh.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    dvh
}

/// Calculates Tumor Control Probability (TCP) using the Poisson Model.
///
/// The Poisson model assumes that the number of surviving clonogens follows a Poisson distribution.
/// TCP is the probability that zero clonogens survive.
///
/// # Arguments
///
/// * `n0` ($N_0$) - Initial number of clonogenic cells.
/// * `alpha` ($\alpha$) - Linear radiosensitivity parameter (Gy⁻¹).
/// * `beta` ($\beta$) - Quadratic radiosensitivity parameter (Gy⁻²).
/// * `dose_per_fraction` ($d$) - Dose delivered per fraction (Gy).
/// * `fractions` ($n$) - Number of fractions.
///
/// # Returns
///
/// * `f64` - The probability of tumor control (0.0 to 1.0).
///
/// # Formula
///
/// $TCP = \exp(-N_0 \exp(-\alpha n d - \beta n d^2))$
pub fn tcp_model(
    n0: f64,
    alpha: f64,
    beta: f64,
    dose_per_fraction: f64,
    fractions: f64,
) -> f64 {
    // The exponent inside is: - alpha * n * d - beta * n * d^2
    let exponent =
        -alpha * fractions * dose_per_fraction - beta * fractions * dose_per_fraction.powi(2);
    let surviving_clonogens = n0 * exponent.exp();

    (-surviving_clonogens).exp()
}

/// Calculates the Pearson Correlation Coefficient ($r$).
///
/// $$ r = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{n \sum X_i^2 - (\sum X_i)^2}} $$
///
/// # Arguments
/// * `x` - First dataset ($X$).
/// * `y` - Second dataset ($Y$).
///
/// # Returns
/// * `Some(f64)` if calculation succeeds.
/// * `None` if datasets are empty or have different lengths.
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let n = x.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y2 = 0.0;

    for i in 0..x.len() {
        sum_x += x[i];
        sum_y += y[i];
        sum_xy += x[i] * y[i];
        sum_x2 += x[i].powi(2);
        sum_y2 += y[i].powi(2);
    }

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x.powi(2)) * (n * sum_y2 - sum_y.powi(2))).sqrt();

    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

/// Calculates the Root-Mean-Square Error (RMSE).
///
/// $$ \text{RMSE} = \sqrt{\frac{1}{n} \sum_{i=1}^{n} (X_i - Y_i)^2} $$
///
/// # Arguments
/// * `predicted` - Predicted values ($X$).
/// * `observed` - Ground truth values ($Y$).
pub fn rmse(predicted: &[f64], observed: &[f64]) -> Option<f64> {
    if predicted.len() != observed.len() || predicted.is_empty() {
        return None;
    }

    let n = predicted.len() as f64;
    let mut sum_sq_diff = 0.0;

    for i in 0..predicted.len() {
        let diff = predicted[i] - observed[i];
        sum_sq_diff += diff.powi(2);
    }

    Some((sum_sq_diff / n).sqrt())
}

/// Calculates the Spatial Accuracy Percentage Error.
///
/// $$ \text{Percentage error} = \frac{X_{\text{meas}} - X_{\text{ref}}}{X_{\text{meas}}} \times 100 $$
///
/// Note: The prompt formula has $X_{meas}$ in the denominator, which is slightly unusual (typically $X_{ref}$ is denominator),
/// but I will implement it exactly as requested.
pub fn spatial_accuracy_percentage_error(measured: f64, reference: f64) -> f64 {
    if measured == 0.0 {
        return f64::INFINITY; // Avoid division by zero
    }
    ((measured - reference) / measured) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_behavior() {
        let n0 = 1e6;
        let alpha = 0.3;
        let beta = 0.03;
        let fractions = 30.0;

        let d_low = 1.0;
        let d_high = 3.0;

        let tcp_low = tcp_model(n0, alpha, beta, d_low, fractions);
        let tcp_high = tcp_model(n0, alpha, beta, d_high, fractions);

        // Higher dose should result in higher TCP (less survival)
        assert!(tcp_high > tcp_low);
    }

    #[test]
    fn test_dvh_calculation() {
        let doses = vec![10.0, 20.0, 30.0, 40.0];
        let dvh = calculate_dvh(&doses);

        // Doses are distinct.
        // Sorted desc: 40, 30, 20, 10
        // 40: vol = 1/4 = 0.25
        // 30: vol = 2/4 = 0.50
        // 20: vol = 3/4 = 0.75
        // 10: vol = 4/4 = 1.00

        // Output is sorted by dose asc: (10, 1.0), (20, 0.75), (30, 0.5), (40, 0.25)
        assert_eq!(dvh.len(), 4);
        assert_eq!(dvh[0], (10.0, 1.0));
        assert_eq!(dvh[3], (40.0, 0.25));
    }
}
