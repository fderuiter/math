//! Evaluation Metrics for Medical Physics and Tracking.
//!
//! Provides statistical tools to validate respiratory monitoring signals against clinical standards.

/// Calculates the Pearson Correlation Coefficient ($r$).
///
/// Measures the linear correlation between two datasets (e.g., predicted vs. observed respiratory motion).
///
/// # Arguments
///
/// * `x` - First dataset (e.g., predicted values).
/// * `y` - Second dataset (e.g., ground truth).
///
/// # Returns
///
/// * `Option<f64>` - The correlation coefficient, or `None` if inputs are invalid/empty.
///
/// # Formula
///
/// $r = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{n \sum X_i^2 - (\sum X_i)^2} \sqrt{n \sum Y_i^2 - (\sum Y_i)^2}}$
#[verified_engine::verified]
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
    let denom_x = n * sum_x2 - sum_x.powi(2);
    let denom_y = n * sum_y2 - sum_y.powi(2);

    if denom_x <= 0.0 || denom_y <= 0.0 {
        return None; // Zero variance
    }

    Some(numerator / (denom_x.sqrt() * denom_y.sqrt()))
}

/// Calculates the Root-Mean-Square Error (RMSE).
///
/// Quantifies the difference between predicted values and observed values.
///
/// # Arguments
///
/// * `predicted` - The estimated values.
/// * `observed` - The ground truth values.
///
/// # Returns
///
/// * `Option<f64>` - The RMSE value.
///
/// # Formula
///
/// $\text{RMSE} = \sqrt{\frac{1}{n} \sum_{i=1}^{n} (X_i - Y_i)^2}$
#[verified_engine::verified]
pub fn root_mean_square_error(predicted: &[f64], observed: &[f64]) -> Option<f64> {
    if predicted.len() != observed.len() || predicted.is_empty() {
        return None;
    }

    let n = predicted.len() as f64;
    let sum_sq_diff: f64 = predicted
        .iter()
        .zip(observed.iter())
        .map(|(p, o)| (p - o).powi(2))
        .sum();

    Some((sum_sq_diff / n).sqrt())
}

/// Calculates the Spatial Accuracy Percentage Error.
///
/// Used in SGRT and laser calibration to quantify geometric deviation.
///
/// # Arguments
///
/// * `measured` ($X_{\text{meas}}$) - The measured value.
/// * `reference` ($X_{\text{ref}}$) - The reference value.
///
/// # Returns
///
/// * `f64` - The percentage error.
///
/// # Formula
///
/// $\text{Percentage error} = \frac{X_{\text{meas}} - X_{\text{ref}}}{X_{\text{meas}}} \times 100$
///
/// *Note*: The provided formula divides by $X_{\text{meas}}$. Standard error usually divides by $X_{\text{ref}}$,
/// but we follow the requested formula.
#[verified_engine::verified]
pub fn spatial_accuracy_error(measured: f64, reference: f64) -> f64 {
    if measured.abs() < 1e-9 {
        return 0.0; // Avoid division by zero
    }
    ((measured - reference) / measured) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[verified_engine::verified]
    fn test_pearson() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        // Perfectly correlated (r=1)
        let r = pearson_correlation(&x, &y).unwrap();
        assert!((r - 1.0).abs() < 1e-6);

        let y_inv = vec![3.0, 2.0, 1.0];
        // Perfectly negatively correlated (r=-1)
        let r_inv = pearson_correlation(&x, &y_inv).unwrap();
        assert!((r_inv + 1.0).abs() < 1e-6);
    }

    #[test]
    #[verified_engine::verified]
    fn test_rmse() {
        let p = vec![1.0, 2.0, 3.0];
        let o = vec![1.0, 2.0, 3.0];
        assert_eq!(root_mean_square_error(&p, &o).unwrap(), 0.0);

        let p2 = vec![2.0, 2.0];
        let o2 = vec![1.0, 1.0];
        // Diff = 1, Sq = 1, Sum = 2, Mean = 1, Sqrt = 1
        assert_eq!(root_mean_square_error(&p2, &o2).unwrap(), 1.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_spatial_error() {
        // meas=100, ref=90. Error = (10/100)*100 = 10%
        assert_eq!(spatial_accuracy_error(100.0, 90.0), 10.0);
    }
}
