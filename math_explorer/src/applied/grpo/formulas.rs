use statrs::statistics::{Data, Distribution};
use std::collections::HashSet;

const EPSILON_NORM: f64 = 1e-8;

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
///
pub fn response_level_advantage(rewards: &[f64], reward_i: f64) -> f64 {
    let data = Data::new(rewards.to_vec());
    let mean = data.mean().unwrap_or(0.0);
    let std_dev = data.std_dev().unwrap_or(0.0);
    (reward_i - mean) / (std_dev + EPSILON_NORM)
}

/// The clipped surrogate objective for the policy update in GRPO.
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

/// The uncertainty reward function.
pub fn uncertainty_reward(p_hat: f64) -> f64 {
    1.0 - 2.0 * (p_hat - 0.5).abs()
}

fn ngrams(s: &str, n: usize) -> HashSet<String> {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .windows(n)
        .map(|w| w.join(" "))
        .collect()
}

fn bleu_precision(candidate: &str, reference: &str, n: usize) -> f64 {
    let candidate_ngrams = ngrams(candidate, n);
    let reference_ngrams = ngrams(reference, n);

    if candidate_ngrams.is_empty() {
        return if reference_ngrams.is_empty() { 1.0 } else { 0.0 };
    }

    let intersection = candidate_ngrams.intersection(&reference_ngrams).count();
    intersection as f64 / candidate_ngrams.len() as f64
}

fn simple_bleu(candidate: &str, reference: &str) -> f64 {
    let p1 = bleu_precision(candidate, reference, 1);
    let p2 = bleu_precision(candidate, reference, 2);
    let p3 = bleu_precision(candidate, reference, 3);
    let p4 = bleu_precision(candidate, reference, 4);

    if p1 == 0.0 { return 0.0; }

    let candidate_len = candidate.split_whitespace().count();
    if candidate_len == 0 {
        return 0.0;
    }
    let reference_len = reference.split_whitespace().count();

    let brevity_penalty = if candidate_len > reference_len {
        1.0
    } else {
        (1.0 - reference_len as f64 / candidate_len as f64).exp()
    };

    brevity_penalty * (p1 * p2 * p3 * p4).powf(0.25)
}


/// Pairwise distance between questions using a simplified BLEU score.
pub fn pairwise_distance_bleu(question_i: &str, question_j: &str) -> f64 {
    let score = simple_bleu(question_i, question_j);
    1.0 - score
}

/// The repetition penalty for a question.
pub fn repetition_penalty(cluster_size: usize, batch_size: usize, lambda: f64) -> f64 {
    lambda * (cluster_size as f64 / batch_size as f64)
}

/// The composite reward for a valid question.
pub fn composite_reward(uncertainty_reward: f64, repetition_penalty: f64) -> f64 {
    (uncertainty_reward - repetition_penalty).max(0.0)
}

/// The binary reward for a generation x_i.
pub fn binary_reward(satisfies_check: bool) -> i32 {
    if satisfies_check { 1 } else { 0 }
}

/// The Solver's empirical accuracy for a question x.
pub fn solver_empirical_accuracy(responses: &[String], pseudo_label: &str) -> f64 {
    let m = responses.len() as f64;
    if m == 0.0 {
        return 0.0;
    }
    let correct_count = responses.iter().filter(|&y| y == pseudo_label).count() as f64;
    correct_count / m
}

/// Lower bound on the KL divergence.
pub fn kl_divergence_lower_bound(p: f64, beta: f64) -> f64 {
    if beta == 0.0 {
        return f64::INFINITY;
    }
    p * (1.0 - p) / (2.0 * beta.powi(2))
}
