use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug, PartialEq)]
pub enum ComparisonResult {
    Win,
    Loss,
    Tie,
}

/// Compare two subjects based on their clinical outcomes.
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

/// Simulate the unmatched pair comparison.
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

/// Simulate the matched pair comparison.
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

/// Calculate the win ratio.
pub fn calculate_win_ratio(wins: i32, losses: i32) -> f64 {
    if losses == 0 {
        f64::INFINITY
    } else {
        wins as f64 / losses as f64
    }
}

pub struct WinRatioStats {
    pub win_ratio: f64,
    pub ci_low: f64,
    pub ci_high: f64,
    pub p_value: f64,
}

/// Calculate 95% confidence interval and p-value for the win ratio.
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
