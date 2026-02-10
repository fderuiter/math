//! Hidden Markov Models (HMM).
//!
//! This module implements Hidden Markov Models with support for:
//! - Forward algorithm (filtering)
//! - Backward algorithm
//! - Viterbi algorithm (most likely state sequence)
//! - Forward-Backward algorithm (smoothing)

use crate::pure_math::statistics::markov::error::{MarkovError, Result};
use nalgebra::{DMatrix, DVector};

/// A Hidden Markov Model.
///
/// # Mathematical Background
///
/// A Hidden Markov Model consists of:
/// - **States**: A set of hidden states S = {s₁, ..., sₙ}
/// - **Observations**: A set of observable symbols O = {o₁, ..., oₘ}
/// - **π**: Initial state probabilities, πᵢ = P(X₁ = sᵢ)
/// - **A**: State transition probabilities, Aᵢⱼ = P(Xₜ₊₁ = sⱼ | Xₜ = sᵢ)
/// - **B**: Emission probabilities, Bᵢⱼ = P(Yₜ = oⱼ | Xₜ = sᵢ)
///
/// # Applications
///
/// - Speech recognition
/// - Part-of-speech tagging
/// - Bioinformatics (gene finding)
/// - Basketball: detecting "hot hand" states from shooting data
/// - Finance: regime detection in market states
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::statistics::markov::hmm::HiddenMarkovModel;
/// use nalgebra::{DMatrix, DVector};
///
/// // Two hidden states: "Cold" (0) and "Hot" (1)
/// // Two observations: "Miss" (0) and "Make" (1)
///
/// let initial = DVector::from_vec(vec![0.5, 0.5]);
///
/// let transitions = DMatrix::from_row_slice(2, 2, &[
///     0.7, 0.3,  // Cold → Cold/Hot
///     0.4, 0.6,  // Hot → Cold/Hot
/// ]);
///
/// let emissions = DMatrix::from_row_slice(2, 2, &[
///     0.7, 0.3,  // Cold: P(Miss)=0.7, P(Make)=0.3
///     0.2, 0.8,  // Hot: P(Miss)=0.2, P(Make)=0.8
/// ]);
///
/// let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();
///
/// // Observed sequence: Make, Make, Miss
/// let observations = vec![1, 0];
///
/// // Find most likely state sequence
/// let states = hmm.viterbi(&observations).unwrap();
/// println!("Most likely states: {:?}", states);
/// ```
#[derive(Debug, Clone)]
pub struct HiddenMarkovModel {
    /// Initial state probabilities π.
    initial: DVector<f64>,
    /// State transition matrix A (num_states × num_states).
    transitions: DMatrix<f64>,
    /// Emission matrix B (num_states × num_observations).
    emissions: DMatrix<f64>,
    /// Number of hidden states.
    num_states: usize,
    /// Number of observable symbols.
    num_observations: usize,
}

impl HiddenMarkovModel {
    /// Creates a new Hidden Markov Model.
    ///
    /// # Arguments
    ///
    /// * `initial` - Initial state probabilities π (must sum to 1)
    /// * `transitions` - State transition matrix A (rows must sum to 1)
    /// * `emissions` - Emission matrix B (rows must sum to 1)
    ///
    /// # Returns
    ///
    /// A new `HiddenMarkovModel` or an error if validation fails.
    ///
    /// # Errors
    ///
    /// - `DimensionMismatch`: If dimensions are inconsistent
    /// - `InvalidProbability`: If probabilities are invalid
    /// - `NotStochastic`: If matrices are not stochastic
    pub fn new(
        initial: DVector<f64>,
        transitions: DMatrix<f64>,
        emissions: DMatrix<f64>,
    ) -> Result<Self> {
        let num_states = initial.len();

        // Validate dimensions
        if transitions.nrows() != num_states || transitions.ncols() != num_states {
            return Err(MarkovError::DimensionMismatch {
                expected: num_states,
                actual: transitions.nrows(),
            });
        }

        if emissions.nrows() != num_states {
            return Err(MarkovError::DimensionMismatch {
                expected: num_states,
                actual: emissions.nrows(),
            });
        }

        let num_observations = emissions.ncols();

        // Validate initial probabilities
        Self::validate_probability_vector(&initial)?;

        // Validate transition matrix
        Self::validate_stochastic_matrix(&transitions)?;

        // Validate emission matrix
        Self::validate_stochastic_matrix(&emissions)?;

        Ok(HiddenMarkovModel {
            initial,
            transitions,
            emissions,
            num_states,
            num_observations,
        })
    }

    /// Validates that a vector is a probability distribution (sums to 1).
    fn validate_probability_vector(vec: &DVector<f64>) -> Result<()> {
        const TOLERANCE: f64 = 1e-10;

        let sum: f64 = vec.iter().sum();
        if (sum - 1.0).abs() > TOLERANCE {
            return Err(MarkovError::NotStochastic {
                reason: format!("Probability vector sums to {} instead of 1.0", sum),
            });
        }

        for &p in vec.iter() {
            if !p.is_finite() || !(0.0..=1.0).contains(&p) {
                return Err(MarkovError::InvalidProbability { value: p });
            }
        }

        Ok(())
    }

    /// Validates that a matrix is row-stochastic.
    fn validate_stochastic_matrix(matrix: &DMatrix<f64>) -> Result<()> {
        const TOLERANCE: f64 = 1e-10;

        for i in 0..matrix.nrows() {
            let row_sum: f64 = matrix.row(i).iter().sum();
            if (row_sum - 1.0).abs() > TOLERANCE {
                return Err(MarkovError::NotStochastic {
                    reason: format!("Row {} sums to {} instead of 1.0", i, row_sum),
                });
            }

            for j in 0..matrix.ncols() {
                let p = matrix[(i, j)];
                if !p.is_finite() || !(0.0..=1.0).contains(&p) {
                    return Err(MarkovError::InvalidProbability { value: p });
                }
            }
        }

        Ok(())
    }

    /// Returns the number of hidden states.
    pub fn num_states(&self) -> usize {
        self.num_states
    }

    /// Returns the number of observable symbols.
    pub fn num_observations(&self) -> usize {
        self.num_observations
    }

    /// Returns the initial state probabilities.
    pub fn initial(&self) -> &DVector<f64> {
        &self.initial
    }

    /// Returns the transition matrix.
    pub fn transitions(&self) -> &DMatrix<f64> {
        &self.transitions
    }

    /// Returns the emission matrix.
    pub fn emissions(&self) -> &DMatrix<f64> {
        &self.emissions
    }

    /// Computes P(observations) using the forward algorithm.
    ///
    /// # Mathematical Background
    ///
    /// Define α(t, i) = P(Y₁, ..., Yₜ, Xₜ = i), the probability of observing
    /// the first t observations and being in state i at time t.
    ///
    /// Recursion:
    /// - α(1, i) = πᵢ · Bᵢ,y₁
    /// - α(t+1, j) = Σᵢ α(t, i) · Aᵢⱼ · Bⱼ,yₜ₊₁
    ///
    /// Then P(Y₁, ..., Yₜ) = Σᵢ α(T, i).
    ///
    /// # Arguments
    ///
    /// * `observations` - Sequence of observation indices
    ///
    /// # Returns
    ///
    /// The probability of the observation sequence.
    pub fn forward(&self, observations: &[usize]) -> Result<f64> {
        let (alpha, _) = self.forward_probabilities(observations)?;
        Ok(alpha.column(alpha.ncols() - 1).sum())
    }

    /// Computes forward probabilities α(t, i) for all t and i.
    ///
    /// # Returns
    ///
    /// A matrix where column t contains α(t, ·).
    fn forward_probabilities(&self, observations: &[usize]) -> Result<(DMatrix<f64>, Vec<f64>)> {
        if observations.is_empty() {
            return Err(MarkovError::InvalidObservation {
                reason: "Observation sequence is empty".to_string(),
            });
        }

        let t_max = observations.len();
        let mut alpha = DMatrix::zeros(self.num_states, t_max);
        let mut scaling_factors = vec![0.0; t_max];

        // Initialize: α(1, i) = πᵢ · Bᵢ,y₁
        let y0 = observations[0];
        if y0 >= self.num_observations {
            return Err(MarkovError::InvalidObservation {
                reason: format!("Observation {} out of bounds", y0),
            });
        }

        for i in 0..self.num_states {
            alpha[(i, 0)] = self.initial[i] * self.emissions[(i, y0)];
        }

        // Scale to prevent underflow
        let scale0 = alpha.column(0).sum();
        if scale0 > 0.0 {
            for i in 0..self.num_states {
                alpha[(i, 0)] /= scale0;
            }
        }
        scaling_factors[0] = scale0;

        // Recursion: α(t+1, j) = Σᵢ α(t, i) · Aᵢⱼ · Bⱼ,yₜ₊₁
        for t in 1..t_max {
            let y_t = observations[t];
            if y_t >= self.num_observations {
                return Err(MarkovError::InvalidObservation {
                    reason: format!("Observation {} out of bounds", y_t),
                });
            }

            for j in 0..self.num_states {
                let mut sum = 0.0;
                for i in 0..self.num_states {
                    sum += alpha[(i, t - 1)] * self.transitions[(i, j)];
                }
                alpha[(j, t)] = sum * self.emissions[(j, y_t)];
            }

            // Scale
            let scale_t = alpha.column(t).sum();
            if scale_t > 0.0 {
                for i in 0..self.num_states {
                    alpha[(i, t)] /= scale_t;
                }
            }
            scaling_factors[t] = scale_t;
        }

        Ok((alpha, scaling_factors))
    }

    /// Computes backward probabilities β(t, i).
    ///
    /// # Mathematical Background
    ///
    /// Define β(t, i) = P(Yₜ₊₁, ..., Yₜ | Xₜ = i), the probability of
    /// observing the remaining observations given state i at time t.
    ///
    /// Recursion:
    /// - β(T, i) = 1
    /// - β(t, i) = Σⱼ Aᵢⱼ · Bⱼ,yₜ₊₁ · β(t+1, j)
    ///
    /// # Arguments
    ///
    /// * `observations` - Sequence of observation indices
    /// * `scaling_factors` - Scaling factors from forward pass
    ///
    /// # Returns
    ///
    /// A matrix where column t contains β(t, ·).
    fn backward_probabilities(
        &self,
        observations: &[usize],
        scaling_factors: &[f64],
    ) -> Result<DMatrix<f64>> {
        if observations.is_empty() {
            return Err(MarkovError::InvalidObservation {
                reason: "Observation sequence is empty".to_string(),
            });
        }

        let t_max = observations.len();
        let mut beta = DMatrix::zeros(self.num_states, t_max);

        // Initialize: β(T, i) = 1
        for i in 0..self.num_states {
            beta[(i, t_max - 1)] = 1.0 / scaling_factors[t_max - 1];
        }

        // Recursion (backward)
        for t in (0..t_max - 1).rev() {
            let y_next = observations[t + 1];

            for i in 0..self.num_states {
                let mut sum = 0.0;
                for j in 0..self.num_states {
                    sum += self.transitions[(i, j)]
                        * self.emissions[(j, y_next)]
                        * beta[(j, t + 1)];
                }
                beta[(i, t)] = sum / scaling_factors[t];
            }
        }

        Ok(beta)
    }

    /// Viterbi algorithm: finds the most likely state sequence.
    ///
    /// # Mathematical Background
    ///
    /// Define δ(t, i) = max P(X₁, ..., Xₜ₋₁, Xₜ = i, Y₁, ..., Yₜ),
    /// the maximum probability of any state sequence ending in state i at time t.
    ///
    /// Recursion:
    /// - δ(1, i) = πᵢ · Bᵢ,y₁
    /// - δ(t+1, j) = max_i [δ(t, i) · Aᵢⱼ] · Bⱼ,yₜ₊₁
    ///
    /// Backtracking gives the most likely path.
    ///
    /// # Arguments
    ///
    /// * `observations` - Sequence of observation indices
    ///
    /// # Returns
    ///
    /// The most likely state sequence.
    pub fn viterbi(&self, observations: &[usize]) -> Result<Vec<usize>> {
        if observations.is_empty() {
            return Err(MarkovError::InvalidObservation {
                reason: "Observation sequence is empty".to_string(),
            });
        }

        let t_max = observations.len();
        let mut delta = DMatrix::zeros(self.num_states, t_max);
        let mut psi = DMatrix::zeros(self.num_states, t_max);

        // Initialize: δ(1, i) = πᵢ · Bᵢ,y₁
        let y0 = observations[0];
        if y0 >= self.num_observations {
            return Err(MarkovError::InvalidObservation {
                reason: format!("Observation {} out of bounds", y0),
            });
        }

        for i in 0..self.num_states {
            delta[(i, 0)] = self.initial[i] * self.emissions[(i, y0)];
        }

        // Recursion
        for t in 1..t_max {
            let y_t = observations[t];
            if y_t >= self.num_observations {
                return Err(MarkovError::InvalidObservation {
                    reason: format!("Observation {} out of bounds", y_t),
                });
            }

            for j in 0..self.num_states {
                let mut max_val = 0.0;
                let mut max_idx = 0;

                for i in 0..self.num_states {
                    let val = delta[(i, t - 1)] * self.transitions[(i, j)];
                    if val > max_val {
                        max_val = val;
                        max_idx = i;
                    }
                }

                delta[(j, t)] = max_val * self.emissions[(j, y_t)];
                psi[(j, t)] = max_idx as f64;
            }
        }

        // Backtracking
        let mut path = vec![0; t_max];

        // Find best final state
        let mut max_val = 0.0;
        let mut max_idx = 0;
        for i in 0..self.num_states {
            if delta[(i, t_max - 1)] > max_val {
                max_val = delta[(i, t_max - 1)];
                max_idx = i;
            }
        }
        path[t_max - 1] = max_idx;

        // Backtrack
        for t in (0..t_max - 1).rev() {
            path[t] = psi[(path[t + 1], t + 1)] as usize;
        }

        Ok(path)
    }

    /// Computes the posterior state probabilities γ(t, i) = P(Xₜ = i | Y₁, ..., Yₜ).
    ///
    /// # Mathematical Background
    ///
    /// Using forward and backward probabilities:
    /// γ(t, i) = α(t, i) · β(t, i) / P(Y₁, ..., Yₜ)
    ///
    /// # Arguments
    ///
    /// * `observations` - Sequence of observation indices
    ///
    /// # Returns
    ///
    /// A matrix where column t contains γ(t, ·).
    pub fn posterior_probabilities(&self, observations: &[usize]) -> Result<DMatrix<f64>> {
        let (alpha, scaling_factors) = self.forward_probabilities(observations)?;
        let beta = self.backward_probabilities(observations, &scaling_factors)?;

        let t_max = observations.len();
        let mut gamma = DMatrix::zeros(self.num_states, t_max);

        for t in 0..t_max {
            for i in 0..self.num_states {
                gamma[(i, t)] = alpha[(i, t)] * beta[(i, t)];
            }

            // Normalize
            let sum = gamma.column(t).sum();
            if sum > 0.0 {
                for i in 0..self.num_states {
                    gamma[(i, t)] /= sum;
                }
            }
        }

        Ok(gamma)
    }

    /// Computes the posterior state probabilities for the most recent observation (filtering).
    ///
    /// # Arguments
    ///
    /// * `observations` - Sequence of observation indices
    ///
    /// # Returns
    ///
    /// The posterior probabilities P(Xₜ = i | Y₁, ..., Yₜ) at the final time T.
    pub fn filter(&self, observations: &[usize]) -> Result<DVector<f64>> {
        let (alpha, _) = self.forward_probabilities(observations)?;
        let t_max = observations.len();

        let mut posterior = alpha.column(t_max - 1).into_owned();
        let sum = posterior.sum();
        if sum > 0.0 {
            posterior /= sum;
        }

        Ok(posterior)
    }

    /// Generates a random observation sequence from the HMM.
    ///
    /// # Arguments
    ///
    /// * `length` - Length of the sequence to generate
    /// * `rng` - Random number generator
    ///
    /// # Returns
    ///
    /// A pair (state_sequence, observation_sequence).
    pub fn generate<R: rand::Rng>(
        &self,
        length: usize,
        rng: &mut R,
    ) -> Result<(Vec<usize>, Vec<usize>)> {
        use rand_distr::{Distribution, WeightedIndex};

        if length == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let mut states = Vec::with_capacity(length);
        let mut observations = Vec::with_capacity(length);

        // Sample initial state
        let initial_dist = WeightedIndex::new(self.initial.as_slice())
            .map_err(|_| MarkovError::NumericalError {
                reason: "Failed to create initial distribution".to_string(),
            })?;
        let mut current_state = initial_dist.sample(rng);
        states.push(current_state);

        // Sample initial observation
        let emission_weights: Vec<f64> = self.emissions.row(current_state).iter().copied().collect();
        let emission_dist = WeightedIndex::new(&emission_weights)
            .map_err(|_| MarkovError::NumericalError {
                reason: "Failed to create emission distribution".to_string(),
            })?;
        observations.push(emission_dist.sample(rng));

        // Generate remaining sequence
        for _ in 1..length {
            // Sample next state
            let transition_weights: Vec<f64> = self.transitions.row(current_state).iter().copied().collect();
            let transition_dist =
                WeightedIndex::new(&transition_weights).map_err(
                    |_| MarkovError::NumericalError {
                        reason: "Failed to create transition distribution".to_string(),
                    },
                )?;
            current_state = transition_dist.sample(rng);
            states.push(current_state);

            // Sample observation
            let emission_weights: Vec<f64> = self.emissions.row(current_state).iter().copied().collect();
            let emission_dist =
                WeightedIndex::new(&emission_weights).map_err(|_| {
                    MarkovError::NumericalError {
                        reason: "Failed to create emission distribution".to_string(),
                    }
                })?;
            observations.push(emission_dist.sample(rng));
        }

        Ok((states, observations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_hmm_creation() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        assert_eq!(hmm.num_states(), 2);
        assert_eq!(hmm.num_observations(), 2);
    }

    #[test]
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
    fn test_generate_sequence() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();
        let mut rng = StdRng::seed_from_u64(42);

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
    fn test_deterministic_generation() {
        let initial = DVector::from_vec(vec![0.5, 0.5]);
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

        let mut rng1 = StdRng::seed_from_u64(123);
        let (states1, obs1) = hmm.generate(20, &mut rng1).unwrap();

        let mut rng2 = StdRng::seed_from_u64(123);
        let (states2, obs2) = hmm.generate(20, &mut rng2).unwrap();

        // Same seed should produce same sequence
        assert_eq!(states1, states2);
        assert_eq!(obs1, obs2);
    }

    #[test]
    fn test_validation_errors() {
        let initial = DVector::from_vec(vec![0.5, 0.4]); // Doesn't sum to 1
        let transitions = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let emissions = DMatrix::from_row_slice(2, 2, &[0.8, 0.2, 0.3, 0.7]);

        assert!(HiddenMarkovModel::new(initial, transitions, emissions).is_err());
    }
}
