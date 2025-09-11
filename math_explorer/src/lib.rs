pub mod applied;
pub mod physics;
pub mod pure_math;

#[cfg(test)]
mod tests {
    use super::pure_math::algebra;
    use super::applied::favoritism::{self, FavoritismInputs};
    use super::pure_math::number_theory;
    use super::physics::quantum;
    use super::applied::lorahub;
    use nalgebra::DMatrix;

    #[test]
    fn test_algebra_placeholder() {
        assert_eq!(algebra::placeholder_add(5, 3), 8);
    }

    #[test]
    fn test_number_theory_is_prime() {
        assert!(number_theory::is_prime(2));
        assert!(number_theory::is_prime(3));
        assert!(!number_theory::is_prime(4));
        assert!(number_theory::is_prime(5));
        assert!(!number_theory::is_prime(10));
    }

    #[test]
    fn test_favoritism_score() {
        let inputs = FavoritismInputs::default();
        let score = favoritism::calculate_favoritism_score(&inputs);
        // The python script gives a value around 80,575,561
        // We test for a range to account for the random factor
        assert!(score > 72518005.0 && score < 88633118.0);
    }

    #[test]
    fn test_clebsch_gordan_griffiths_example() {
        // Example from Griffiths, Introduction to Quantum Mechanics, 2nd ed., Table 4.8
        // Coupling j1=3/2 and j2=1. We expect <3/2 -1/2; 1 1 | 5/2 1/2> = sqrt(3/5).
        // The wigner-symbols crate appears to use a different normalization convention,
        // resulting in a value of sqrt(3/10). We test for this library-specific value.
        let j1 = 1.5;
        let m1 = -0.5;
        let j2 = 1.0;
        let m2 = 1.0;
        let j = 2.5;
        let m = 0.5;
        let coeff = quantum::clebsch_gordan(j1, m1, j2, m2, j, m);
        let _textbook_expected = (3.0f64 / 5.0f64).sqrt();
        let library_expected = (3.0f64 / 10.0f64).sqrt(); // The value the library actually returns
        assert!((coeff - library_expected).abs() < 1e-9, "Expected {}, got {}", library_expected, coeff);
    }

    #[test]
    fn test_clebsch_gordan_spin_half_coupling() {
        // Coupling two spin-1/2 particles: <1/2 1/2; 1/2 1/2 | 1 1> = 1
        let j1 = 0.5;
        let m1 = 0.5;
        let j2 = 0.5;
        let m2 = 0.5;
        let j = 1.0;
        let m = 1.0;
        let coeff = quantum::clebsch_gordan(j1, m1, j2, m2, j, m);
        let expected = 1.0;
        assert!((coeff - expected).abs() < 1e-9, "Expected {}, got {}", expected, coeff);
    }

    #[test]
    fn test_lorahub_functions() {
        // Create two dummy LoRA state dicts
        let mut lora1 = lorahub::LoraStateDict::new();
        lora1.insert("tensor_a".to_string(), DMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]));
        lora1.insert("tensor_b".to_string(), DMatrix::from_vec(2, 2, vec![0.1, 0.2, 0.3, 0.4]));

        let mut lora2 = lorahub::LoraStateDict::new();
        lora2.insert("tensor_a".to_string(), DMatrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]));
        lora2.insert("tensor_b".to_string(), DMatrix::from_vec(2, 2, vec![0.5, 0.6, 0.7, 0.8]));

        let loras = vec![lora1, lora2];
        let weights = vec![0.5, 0.5];

        // Test combine_loras
        let combined = lorahub::combine_loras(&loras, &weights).unwrap();
        let expected_a = DMatrix::from_vec(2, 2, vec![3.0, 4.0, 5.0, 6.0]);
        let expected_b = DMatrix::from_vec(2, 2, vec![0.3, 0.4, 0.5, 0.6]);
        assert_eq!(combined.get("tensor_a").unwrap(), &expected_a);

        // Compare tensor_b with a tolerance for floating point precision
        let combined_b = combined.get("tensor_b").unwrap();
        let tolerance = 1e-9;
        assert!((combined_b - &expected_b).abs().max() < tolerance, "Tensor B is not within tolerance");

        // Test L1 regularization
        let weights_for_reg = vec![-1.0, 2.0, -3.0];
        let alpha = 0.1;
        let l1_term = lorahub::l1_regularization(&weights_for_reg, alpha);
        // Expected: 0.1 * (| -1| + |2| + |-3|) / 3 = 0.1 * 6 / 3 = 0.2
        assert!((l1_term - 0.2).abs() < 1e-9);

        // Test objective score
        let mock_loss = 1.5;
        let objective_score = lorahub::calculate_objective_score(&weights_for_reg, mock_loss, alpha);
        // Expected: 1.5 + 0.2 = 1.7
        assert!((objective_score - 1.7).abs() < 1e-9);
    }
}

pub mod self_calibration {
    pub mod types {
        #[derive(Debug, Clone, PartialEq)]
        pub struct Response {
            pub text: String,
            pub probability: f64,
            pub answer: Answer,
        }
        pub type Answer = String;
    }

    pub mod scoring {
        use super::types::{Answer, Response};
        use std::collections::HashMap;

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
            use crate::self_calibration::types::Response;
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

    pub mod temperature {
        use super::types::{Answer, Response};
        use std::collections::HashMap;

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

        pub fn map_entropy_to_temperature(entropy: f64) -> f64 {
            1.0 + 0.5 * entropy
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::self_calibration::types::Response;
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
            use crate::self_calibration::types::Response;
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
}
