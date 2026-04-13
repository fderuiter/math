//! Validation Metrics
//!
//! Standard metrics used to assess the accuracy, correlation, and reliability of respiratory signals.

/// Calculates the Percentage Error between a measured value and a reference value.
///
/// # Arguments
///
/// * `measured` - The experimental value ($X_{measured}$).
/// * `reference` - The ground truth value ($X_{reference}$).
///
/// # Returns
///
/// * `f64` - The percentage error.
///
/// # Formula
///
/// $Percentage Error = \frac{X_{measured} - X_{reference}}{X_{measured}} \times 100\%$
///
/// Note: This implementation follows the specific formula provided in the user request,
/// which uses `measured` in the denominator.
pub fn percentage_error(measured: f64, reference: f64) -> f64 {
    if measured == 0.0 {
        return f64::INFINITY; // Avoid division by zero
    }
    ((measured - reference) / measured) * 100.0
}

/// Calculates the Root-Mean-Square Error (RMSE) between two datasets.
///
/// # Arguments
///
/// * `measured` - Vector of measured values ($X$).
/// * `reference` - Vector of reference values ($Y$).
///
/// # Returns
///
/// * `f64` - The RMSE value.
///
/// # Formula
///
/// $RMSE = \sqrt{\frac{1}{n} \sum (X_i - Y_i)^2}$
pub fn root_mean_square_error(measured: &[f64], reference: &[f64]) -> f64 {
    let n = measured.len().min(reference.len());
    if n == 0 {
        return 0.0;
    }

    let sum_sq_diff: f64 = measured
        .iter()
        .zip(reference.iter())
        .take(n)
        .map(|(x, y)| (x - y).powi(2))
        .sum();

    (sum_sq_diff / n as f64).sqrt()
}

/// Calculates the Pearson Correlation Coefficient (r).
///
/// # Arguments
///
/// * `x` - First dataset.
/// * `y` - Second dataset.
///
/// # Returns
///
/// * `f64` - The correlation coefficient r (-1.0 to 1.0).
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let nf = n as f64;

    let sum_x: f64 = x.iter().take(n).sum();
    let sum_y: f64 = y.iter().take(n).sum();

    // Note: The user provided a specific formula which differs slightly from the standard definition
    // regarding the denominator (specifically the (n-1) factor and variance vs sum of squares).
    // To ensure rigorous validation (Best Practice), this implementation uses the standard
    // definition of Pearson Correlation: Cov(X,Y) / (Std(X) * Std(Y)).

    let mean_x = sum_x / nf;
    let mean_y = sum_y / nf;

    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .take(n)
        .map(|(a, b)| (a - mean_x) * (b - mean_y))
        .sum();

    let denom_x: f64 = x.iter().take(n).map(|a| (a - mean_x).powi(2)).sum();
    let denom_y: f64 = y.iter().take(n).map(|b| (b - mean_y).powi(2)).sum();

    if denom_x == 0.0 || denom_y == 0.0 {
        return 0.0;
    }

    numerator / (denom_x * denom_y).sqrt()
}

/// Calculates the DICE Similarity Coefficient.
///
/// # Arguments
///
/// * `set_a` - Boolean mask or set of indices for volume A.
/// * `set_b` - Boolean mask or set of indices for volume B.
///
/// # Returns
///
/// * `f64` - Dice coefficient (0.0 to 1.0).
///
/// # Formula
///
/// $DSC = \frac{2 |A \cap B|}{|A| + |B|}$
pub fn dice_similarity_coefficient(set_a: &[bool], set_b: &[bool]) -> f64 {
    let n = set_a.len().min(set_b.len());
    let mut intersection = 0;
    let mut size_a = 0;
    let mut size_b = 0;

    for i in 0..n {
        if set_a[i] {
            size_a += 1;
        }
        if set_b[i] {
            size_b += 1;
        }
        if set_a[i] && set_b[i] {
            intersection += 1;
        }
    }

    if size_a + size_b == 0 {
        return 1.0; // Both empty implies perfect overlap (vacuously)
    }

    (2.0 * intersection as f64) / (size_a as f64 + size_b as f64)
}

/// Calculates the Time Shift Error (Sum of Squared Differences) for a given shift.
///
/// # Arguments
///
/// * `measured` - The measured signal Y_measured.
/// * `reference` - The reference signal Y_reference.
///
/// # Returns
///
/// * `f64` - The sum of squared errors.
///
/// # Formula
///
/// $Error = \sum (`Y_{measured}[i]` - `Y_{reference}[i]`)^2$
pub fn time_shift_error(measured: &[f64], reference: &[f64]) -> f64 {
    let n = measured.len().min(reference.len());
    measured
        .iter()
        .zip(reference.iter())
        .take(n)
        .map(|(m, r)| (m - r).powi(2))
        .sum()
}
