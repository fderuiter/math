/// Types used in self-calibration.
pub mod types {
    /// Represents a response from a model.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        /// The text of the response.
        pub text: String,
        /// The probability assigned by the model.
        pub probability: f64,
        /// The extracted answer.
        pub answer: Answer,
    }
    /// Type alias for an answer string.
    pub type Answer = String;
}

/// Scoring functions.
pub mod scoring {
    use super::types::{Answer, Response};
    use std::collections::HashMap;

    /// Calculates the soft self-consistency scores for a set of responses.
    ///
    /// # Arguments
    ///
    /// * `responses` - A slice of responses.
    ///
    /// # Returns
    ///
    /// A vector of scores corresponding to each response.
    pub fn calculate_soft_self_consistency_scores(responses: &[Response]) -> Vec<f64> {
        let mut answer_probabilities: HashMap<&Answer, f64> = HashMap::new();
        for response in responses {
            *answer_probabilities.entry(&response.answer).or_insert(0.0) += response.probability;
        }
        responses
            .iter()
            .map(|response| answer_probabilities.get(&response.answer).cloned().unwrap_or(0.0))
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ai::self_calibration::types::Response;
        #[test]
        fn test_calculate_soft_self_consistency_scores() {
            let responses = vec![
                Response { text: "A".to_string(), probability: 0.8, answer: "A".to_string() },
                Response { text: "B".to_string(), probability: 0.1, answer: "B".to_string() },
                Response { text: "A".to_string(), probability: 0.05, answer: "A".to_string() },
                Response { text: "B".to_string(), probability: 0.05, answer: "B".to_string() },
            ];
            let scores = calculate_soft_self_consistency_scores(&responses);
            let expected_scores = vec![0.85, 0.15, 0.85, 0.15];
            assert_eq!(scores.len(), expected_scores.len());
            for (score, expected) in scores.iter().zip(expected_scores.iter()) {
                assert!((score - expected).abs() < 1e-9);
            }
        }
    }
}

/// Temperature scaling functions.
pub mod temperature {
    use super::types::{Answer, Response};
    use std::collections::HashMap;

    /// Calculates the entropy of the answer distribution.
    ///
    /// # Arguments
    ///
    /// * `responses` - A slice of responses.
    ///
    /// # Returns
    ///
    /// The entropy value.
    pub fn calculate_answer_entropy(responses: &[Response]) -> f64 {
        if responses.is_empty() { return 0.0; }
        let mut answer_counts: HashMap<&Answer, usize> = HashMap::new();
        for response in responses {
            *answer_counts.entry(&response.answer).or_insert(0) += 1;
        }
        let total_responses = responses.len() as f64;
        let mut entropy = 0.0;
        for count in answer_counts.values() {
            let p_a = (*count as f64) / total_responses;
            if p_a > 0.0 { entropy -= p_a * p_a.ln(); }
        }
        entropy
    }

    /// Maps entropy to a temperature value.
    ///
    /// # Arguments
    ///
    /// * `entropy` - The entropy value.
    ///
    /// # Returns
    ///
    /// The temperature value.
    pub fn map_entropy_to_temperature(entropy: f64) -> f64 {
        1.0 + 0.5 * entropy
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ai::self_calibration::types::Response;
        #[test]
        fn test_calculate_answer_entropy() {
            let responses = vec![
                Response { text: "A".to_string(), probability: 0.7, answer: "A".to_string() },
                Response { text: "A".to_string(), probability: 0.1, answer: "A".to_string() },
                Response { text: "B".to_string(), probability: 0.1, answer: "B".to_string() },
                Response { text: "C".to_string(), probability: 0.1, answer: "C".to_string() },
            ];
            let entropy = calculate_answer_entropy(&responses);
            assert!((entropy - 1.03972077).abs() < 1e-8);
        }
    }
}

/// Training functions.
pub mod training {
    use super::scoring::calculate_soft_self_consistency_scores;
    use super::temperature::{calculate_answer_entropy, map_entropy_to_temperature};
    use super::types::Response;

    fn calculate_target_distribution(responses: &[Response]) -> Vec<f64> {
        let scores = calculate_soft_self_consistency_scores(responses);
        let entropy = calculate_answer_entropy(responses);
        let temperature = map_entropy_to_temperature(entropy);
        if temperature.abs() < 1e-9 {
            let mut max_score = f64::NEG_INFINITY;
            let mut max_idx = 0;
            for (i, score) in scores.iter().enumerate() {
                if *score > max_score { max_score = *score; max_idx = i; }
            }
            let mut p = vec![0.0; scores.len()];
            if !p.is_empty() { p[max_idx] = 1.0; }
            return p;
        }
        let temp_scores: Vec<f64> = scores.iter().map(|s| s / temperature).collect();
        let max_temp_score = temp_scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exps: Vec<f64> = temp_scores.iter().map(|s| (s - max_temp_score).exp()).collect();
        let sum_exps: f64 = exps.iter().sum();
        if sum_exps.abs() < 1e-9 {
            let n = responses.len();
            if n > 0 { vec![1.0 / n as f64; n] } else { vec![] }
        } else {
            exps.iter().map(|e| e / sum_exps).collect()
        }
    }

    /// Calculates the KL divergence loss between the predicted distribution and the target distribution.
    ///
    /// # Arguments
    ///
    /// * `responses` - A slice of responses (used to calculate target distribution).
    /// * `predicted_dist` - The predicted probability distribution.
    ///
    /// # Returns
    ///
    /// The KL divergence loss.
    pub fn calculate_kl_divergence_loss(responses: &[Response], predicted_dist: &[f64]) -> f64 {
        if responses.len() != predicted_dist.len() {
            panic!("Distributions have different lengths.");
        }
        if responses.is_empty() { return 0.0; }
        let target_dist = calculate_target_distribution(responses);
        let mut kl_divergence = 0.0;
        for (q, p) in predicted_dist.iter().zip(target_dist.iter()) {
            if *q > 0.0 && *p > 0.0 {
                kl_divergence += q * (q / p).ln();
            }
        }
        kl_divergence
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::ai::self_calibration::types::Response;
        #[test]
        fn test_kl_divergence_calculation() {
            let responses = vec![
                Response { text: "A".to_string(), probability: 0.8, answer: "A".to_string() },
                Response { text: "B".to_string(), probability: 0.1, answer: "B".to_string() },
                Response { text: "A".to_string(), probability: 0.05, answer: "A".to_string() },
                Response { text: "B".to_string(), probability: 0.05, answer: "B".to_string() },
            ];
            let predicted_dist_q = vec![0.4, 0.1, 0.4, 0.1];
            let loss = calculate_kl_divergence_loss(&responses, &predicted_dist_q);
            assert!((loss - 0.07019869).abs() < 1e-6);
        }
    }
}
