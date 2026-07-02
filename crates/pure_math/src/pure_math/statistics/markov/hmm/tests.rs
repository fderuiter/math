#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use super::super::model::HiddenMarkovModel;
    use approx::assert_relative_eq;
    use nalgebra::{DMatrix, DVector};
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    #[verified_engine::verified]
    fn test_hmm_creation() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        assert_eq!(hmm.num_states(), 2);
        assert_eq!(hmm.num_observations(), 2);
    }

    #[test]
    #[verified_engine::verified]
    fn test_hmm_creation_f32() {
        let initial = DVector::from_vec(vec![0.5f32, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7f32, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8f32, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        assert_eq!(hmm.num_states(), 2);
        assert_eq!(hmm.num_observations(), 2);
    }

    #[test]
    #[verified_engine::verified]
    fn test_forward_algorithm() {
        // Simple HMM: two states, two observations
        let initial = DVector::from_vec(vec![0.6, 0.4]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.9, 0.1, 0.2, 0.8]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        let observations = vec![0, 1, 0];
        let prob = hmm.forward(&observations).unwrap();

        // Probability should be in (0, 1)
        assert!(prob > 0.0);
        assert!(prob <= 1.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_viterbi() {
        // Classic example: two states (Fair/Loaded dice), two observations (0-5)
        // Simplified to 2 observations for testing
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.9, 0.1, 0.1, 0.9]);
        let emissions = DMatrix::from_row_slice(
            2,
            2,
            &[
                0.5, 0.5, // Fair: uniform
                0.1, 0.9, // Loaded: biased toward 1
            ],
        );

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        // Observations: 1, 1, 1 (likely loaded)
        let observations = vec![1, 1, 1];
        let path = hmm.viterbi(&observations).unwrap();

        assert_eq!(path.len(), 3);
        // Most likely state sequence should end in loaded (state 1)
        // Due to persistence (0.9 stay probability), likely all state 1
        assert_eq!(path[2], 1);
    }

    #[test]
    #[verified_engine::verified]
    fn test_hot_hand_detection() {
        // Basketball shooting: Cold (0) vs Hot (1)
        // Observations: Miss (0), Make (1)
        let initial = DVector::from_vec(vec![0.5, 0.5]);

        let transitions = DMatrix::from_row_slice(
            2,
            2,
            &[
                0.8, 0.2, // Cold → Cold/Hot
                0.3, 0.7, // Hot → Cold/Hot
            ],
        );

        let emissions = DMatrix::from_row_slice(
            2,
            2,
            &[
                0.7, 0.3, // Cold: 30% makes
                0.2, 0.8, // Hot: 80% makes
            ],
        );

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        // Shooting sequence: Make, Make, Make, Miss
        let observations = vec![1, 1, 1, 0];

        // Viterbi: most likely states
        let states = hmm.viterbi(&observations).unwrap();

        // After three makes, should likely be in hot state
        assert_eq!(states[2], 1); // Hot after 3 makes

        // Filtering: current belief
        let posterior = hmm.filter(&observations).unwrap();

        // After 3 makes then a miss, should still believe somewhat in hot
        assert!(posterior[1] > 0.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_posterior_probabilities() {
        let initial = DVector::from_vec(vec![0.6, 0.4]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        let observations = vec![0, 1];
        let gamma = hmm.posterior_probabilities(&observations).unwrap();

        // Check that posteriors sum to 1 at each time
        for t in 0..2 {
            let sum: f64 = gamma.column(t).iter().sum();
            assert_relative_eq!(sum, 1.0, epsilon = 1e-6);
        }
    }

    #[test]
    #[verified_engine::verified]
    fn test_generate_sequence() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();
        let mut rng = oxidize_core::rng::OxidizeRng::default();

        let (states, observations) = hmm.generate(10, &mut rng).unwrap();

        assert_eq!(states.len(), 10);
        assert_eq!(observations.len(), 10);

        // All states should be valid
        for &s in &states {
            assert!(s < 2);
        }

        // All observations should be valid
        for &o in &observations {
            assert!(o < 2);
        }
    }

    #[test]
    #[verified_engine::verified]
    fn test_deterministic_generation() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        let mut rng1 = oxidize_core::rng::OxidizeRng::new(123);
        let (states1, obs1) = hmm.generate(20, &mut rng1).unwrap();

        let mut rng2 = oxidize_core::rng::OxidizeRng::new(123);
        let (states2, obs2) = hmm.generate(20, &mut rng2).unwrap();

        // Same seed should produce same sequence
        assert_eq!(states1, states2);
        assert_eq!(obs1, obs2);
    }

    #[test]
    #[verified_engine::verified]
    fn test_validation_errors() {
        let initial = DVector::from_vec(vec![0.5, 0.4]); // Doesn't sum to 1
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        assert!(HiddenMarkovModel::new(initial, transitions, emissions).is_err());
    }
}
