//! Statistical functions for signal validation and accuracy assessment.

use std::error::Error;
use std::fmt;

/// Errors related to statistical calculations.
#[derive(Debug)]
pub enum StatisticsError {
    /// Input datasets have different lengths.
    MismatchedLengths { len_x: usize, len_y: usize },
    /// Input dataset is empty.
    EmptyInput,
    /// Variance is zero, causing division by zero in correlation.
    ZeroVariance,
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MismatchedLengths { len_x, len_y } => {
                write!(f, "Input datasets have different lengths: {} and {}", len_x, len_y)
            }
            Self::EmptyInput => write!(f, "Input dataset is empty"),
            Self::ZeroVariance => write!(f, "Zero variance detected (division by zero)"),
        }
    }
}

impl Error for StatisticsError {}

/// Calculates the Pearson Correlation Coefficient ($r$).
///
/// $$ r = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{n \sum X_i^2 - (\sum X_i)^2} \sqrt{n \sum Y_i^2 - (\sum Y_i)^2}} $$
///
/// # Arguments
///
/// * `x` - First dataset.
/// * `y` - Second dataset.
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> Result<f64, StatisticsError> {
    if x.len() != y.len() {
        return Err(StatisticsError::MismatchedLengths {
            len_x: x.len(),
            len_y: y.len(),
        });
    }
    if x.is_empty() {
        return Err(StatisticsError::EmptyInput);
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
        sum_x2 += x[i] * x[i];
        sum_y2 += y[i] * y[i];
    }

    let numerator = n * sum_xy - sum_x * sum_y;
    let den_x = n * sum_x2 - sum_x * sum_x;
    let den_y = n * sum_y2 - sum_y * sum_y;

    if den_x <= 0.0 || den_y <= 0.0 {
        return Err(StatisticsError::ZeroVariance);
    }

    Ok(numerator / (den_x.sqrt() * den_y.sqrt()))
}

/// Calculates the Root-Mean-Square Error (RMSE).
///
/// $$ \text{RMSE} = \sqrt{\frac{1}{n} \sum_{i=1}^{n} (X_i - Y_i)^2} $$
///
/// # Arguments
///
/// * `predicted` - Predicted values ($X$).
/// * `actual` - Ground truth values ($Y$).
pub fn rmse(predicted: &[f64], actual: &[f64]) -> Result<f64, StatisticsError> {
    if predicted.len() != actual.len() {
        return Err(StatisticsError::MismatchedLengths {
            len_x: predicted.len(),
            len_y: actual.len(),
        });
    }
    if predicted.is_empty() {
        return Err(StatisticsError::EmptyInput);
    }

    let n = predicted.len() as f64;
    let sum_sq_diff: f64 = predicted
        .iter()
        .zip(actual.iter())
        .map(|(p, a)| (p - a).powi(2))
        .sum();

    Ok((sum_sq_diff / n).sqrt())
}

/// Calculates the Spatial Accuracy Percentage Error.
///
/// $$ \text{Percentage error} = \frac{X_{\text{meas}} - X_{\text{ref}}}{X_{\text{meas}}} \times 100 $$
///
/// Note: The formula provided in requirements divides by $X_{\text{meas}}$ (measured),
/// though typically percentage error divides by $X_{\text{ref}}$. We follow the requirement here.
///
/// # Arguments
/// * `measured` - The measured value ($X_{\text{meas}}$).
/// * `reference` - The reference value ($X_{\text{ref}}$).
pub fn spatial_accuracy_percentage_error(measured: f64, reference: f64) -> f64 {
    if measured == 0.0 {
        return 0.0; // Avoid division by zero, though mathematically undefined.
    }
    ((measured - reference) / measured) * 100.0
}
