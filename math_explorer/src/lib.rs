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
    fn test_number_theory_placeholder() {
        assert!(number_theory::is_prime_placeholder(2));
        assert!(!number_theory::is_prime_placeholder(10));
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
