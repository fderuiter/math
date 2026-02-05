use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug, PartialEq)]
pub enum ComparisonResult {
    Win,
    Loss,
    Tie,
}

/// Compares two subjects outcome-by-outcome based on a hierarchy of events.
///
/// This function iterates through the outcomes provided for two subjects. The `outcomes`
/// slices must be ordered by clinical priority (e.g., [Death, Heart Failure, QoL]).
///
/// # Logic
///
/// 1. Compare the first outcome (Highest Priority).
/// 2. If `subject1 > subject2` (Outcome 1 is better for subject 1), return `Win`.
/// 3. If `subject1 < subject2`, return `Loss`.
/// 4. If they are equal (Tie), move to the next outcome.
/// 5. If all outcomes are tied, return `Tie`.
///
/// # Arguments
///
/// * `subject1` - A slice of outcomes for the first subject (e.g., Treatment Group).
/// * `subject2` - A slice of outcomes for the second subject (e.g., Control Group).
///
/// # Type Constraints
///
/// The outcomes `T` must implement `PartialOrd`.
/// Note: Ensure that "Higher" means "Better" for the comparison to be intuitive.
/// For example, "Days to Death" (Higher is Better) vs "Hospitalization Count" (Lower is Better).
/// You should invert negative outcomes before passing them here so they align directionally.
pub fn compare_outcomes<T: PartialOrd>(subject1: &[T], subject2: &[T]) -> ComparisonResult {
    for (outcome1, outcome2) in subject1.iter().zip(subject2.iter()) {
        if outcome1 > outcome2 {
            return ComparisonResult::Win;
        } else if outcome1 < outcome2 {
            return ComparisonResult::Loss;
        }
    }
    ComparisonResult::Tie
}

/// Performs an Unmatched Pair comparison (All-Pairs).
///
/// Compares every subject in `group1` against every subject in `group2`.
/// This is an $O(N \times M)$ operation.
///
/// # Arguments
///
/// * `group1` - A list of subjects (each subject is a list of outcomes) for the first group.
/// * `group2` - A list of subjects for the second group.
///
/// # Returns
///
/// A tuple `(wins, losses)` representing the total number of wins and losses for `group1`.
pub fn unmatched_pairs<T: PartialOrd>(group1: &[Vec<T>], group2: &[Vec<T>]) -> (i32, i32) {
    let mut wins = 0;
    let mut losses = 0;
    for subj1 in group1 {
        for subj2 in group2 {
            match compare_outcomes(subj1, subj2) {
                ComparisonResult::Win => wins += 1,
                ComparisonResult::Loss => losses += 1,
                ComparisonResult::Tie => (),
            }
        }
    }
    (wins, losses)
}

/// Performs a Matched Pair comparison.
///
/// Compares `group1[i]` against `group2[i]`. Useful for studies with matched cohorts
/// (e.g., twin studies or propensity score matching).
///
/// # Panics
///
/// Panics if the groups have different lengths.
pub fn matched_pairs<T: PartialOrd>(group1: &[Vec<T>], group2: &[Vec<T>]) -> (i32, i32) {
    assert_eq!(
        group1.len(),
        group2.len(),
        "Groups must be of equal length for matched pairs."
    );
    let mut wins = 0;
    let mut losses = 0;
    for (subj1, subj2) in group1.iter().zip(group2.iter()) {
        match compare_outcomes(subj1, subj2) {
            ComparisonResult::Win => wins += 1,
            ComparisonResult::Loss => losses += 1,
            ComparisonResult::Tie => (),
        }
    }
    (wins, losses)
}

/// Calculates the raw Win Ratio.
///
/// $$ WR = \frac{N_{wins}}{N_{losses}} $$
///
/// Returns `f64::INFINITY` if losses are zero.
pub fn calculate_win_ratio(wins: i32, losses: i32) -> f64 {
    if losses == 0 {
        f64::INFINITY
    } else {
        wins as f64 / losses as f64
    }
}

/// Statistical results for a Win Ratio analysis.
#[derive(Debug, Clone, Copy)]
pub struct WinRatioStats {
    /// The calculated win ratio ($N_W / N_L$).
    pub win_ratio: f64,
    /// Lower bound of the 95% Confidence Interval.
    pub ci_low: f64,
    /// Upper bound of the 95% Confidence Interval.
    pub ci_high: f64,
    /// Two-sided P-value testing the null hypothesis that $WR = 1$.
    pub p_value: f64,
}

/// Calculates 95% Confidence Intervals and P-values for the Win Ratio.
///
/// Uses the normal approximation for the log-win-ratio.
///
/// # Returns
///
/// `None` if the total number of pairs is zero.
///
/// # Mathematical Details
///
/// The standard error is estimated as:
/// $$ SE = \sqrt{\frac{p(1-p)}{N}} $$
/// Where $p$ is the proportion of wins among untied pairs (simplified).
pub fn calculate_statistics(wins: i32, losses: i32) -> Option<WinRatioStats> {
    let total_pairs = wins + losses;
    if total_pairs == 0 {
        return None;
    }

    let p_win = wins as f64 / total_pairs as f64;
    let se = (p_win * (1.0 - p_win) / total_pairs as f64).sqrt();

    if se == 0.0 {
        // This can happen if all pairs are wins or all are losses.
        // In this case, the confidence interval is not well-defined in this formulation.
        // The p-value would be very small, but the z-score is infinite.
        // We can return a result indicating this edge case.
        let win_ratio = calculate_win_ratio(wins, losses);
        return Some(WinRatioStats {
            win_ratio,
            ci_low: if p_win == 1.0 { win_ratio } else { 0.0 },
            ci_high: if p_win == 1.0 {
                f64::INFINITY
            } else {
                win_ratio
            },
            p_value: 0.0, // Or a very small number, depending on interpretation.
        });
    }

    let ci_low_p = (p_win - 1.96 * se).max(0.0);
    // Clamp the upper bound to slightly less than 1.0 to avoid division by zero
    // or negative results when converting to ratio.
    // If the upper bound of p is 1, the ratio upper bound is infinity.
    let ci_high_p = (p_win + 1.96 * se).min(1.0);

    let ci_low = if ci_low_p == 1.0 {
        f64::INFINITY
    } else {
        ci_low_p / (1.0 - ci_low_p)
    };

    let ci_high = if ci_high_p == 1.0 {
        f64::INFINITY
    } else {
        ci_high_p / (1.0 - ci_high_p)
    };

    let z_score = (p_win - 0.5) / se;
    let normal = Normal::new(0.0, 1.0).unwrap();
    let p_value = 2.0 * (1.0 - normal.cdf(z_score.abs()));

    Some(WinRatioStats {
        win_ratio: calculate_win_ratio(wins, losses),
        ci_low,
        ci_high,
        p_value,
    })
}
