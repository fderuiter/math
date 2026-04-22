use statrs::statistics::{Data, Distribution};

const EPSILON_NORM: f64 = 1e-8;

// Re-export moved functions to maintain backward compatibility
pub use super::metrics::pairwise_distance_bleu;
pub use super::rewards::{binary_reward, composite_reward, repetition_penalty, uncertainty_reward};

/// Calculates the response-level advantage using z-score normalization.
///
/// # Arguments
///
/// * `rewards` - A slice of scalar rewards for a group of responses.
/// * `reward_i` - The specific reward for which to calculate the advantage.
///
/// # Returns
///
/// The response-level advantage for `reward_i`.
pub fn response_level_advantage(rewards: &[f64], reward_i: f64) -> f64 {
    let data = Data::new(rewards.to_vec());
    let mean = data.mean().unwrap_or(0.0);
    let std_dev = data.std_dev().unwrap_or(0.0);
    (reward_i - mean) / (std_dev + EPSILON_NORM)
}

/// The clipped surrogate objective for the policy update in GRPO.
///
/// # Arguments
///
/// * `pi_thetas` - Probabilities of the sampled outputs under the current policy.
/// * `pi_theta_olds` - Probabilities of the sampled outputs under the old policy.
/// * `advantages` - Calculated advantages for the sampled outputs.
/// * `epsilon` - Clipping parameter to limit the policy update step size.
/// * `beta` - Coefficient for the KL divergence term (unused in this formula as written, but part of the general objective).
/// * `kl_divergence` - The KL divergence term.
///
/// # Returns
///
/// The calculated objective value to be maximized (or minimized if negative).
pub fn clipped_surrogate_objective(
    pi_thetas: &[f64],
    pi_theta_olds: &[f64],
    advantages: &[f64],
    epsilon: f64,
    beta: f64,
    kl_divergence: f64,
) -> f64 {
    let g = pi_thetas.len() as f64;
    let sum: f64 = pi_thetas
        .iter()
        .zip(pi_theta_olds.iter())
        .zip(advantages.iter())
        .map(|((pi_theta, pi_theta_old), advantage)| {
            let r_t = pi_theta / pi_theta_old;
            let clipped = r_t.max(1.0 - epsilon).min(1.0 + epsilon);
            (r_t * advantage).min(clipped * advantage)
        })
        .sum();

    -(1.0 / g) * sum + beta * kl_divergence
}

/// The Solver's empirical accuracy for a question x.
///
/// # Arguments
///
/// * `responses` - A slice of response strings.
/// * `pseudo_label` - The label considered correct (the consensus).
///
/// # Returns
///
/// The proportion of responses that match the pseudo-label.
pub fn solver_empirical_accuracy(responses: &[String], pseudo_label: &str) -> f64 {
    let m = responses.len() as f64;
    if m == 0.0 {
        return 0.0;
    }
    let correct_count = responses.iter().filter(|&y| y == pseudo_label).count() as f64;
    correct_count / m
}

/// Lower bound on the KL divergence.
///
/// # Arguments
///
/// * `p` - The probability.
/// * `beta` - A scaling parameter.
///
/// # Returns
///
/// The lower bound on the KL divergence.
pub fn kl_divergence_lower_bound(p: f64, beta: f64) -> f64 {
    if beta == 0.0 {
        return f64::INFINITY;
    }
    p * (1.0 - p) / (2.0 * beta.powi(2))
}
