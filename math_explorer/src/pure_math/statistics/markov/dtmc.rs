//! Discrete-Time Markov Chains (DTMC).
//!
//! This module implements discrete-time Markov chains with support for:
//! - Transient and absorbing states
//! - Canonical form decomposition
//! - Fundamental matrix computation
//! - Expected Possession Value (EPV) calculation
//! - Stationary distribution computation
//! - Absorption probabilities

use crate::pure_math::statistics::markov::error::{MarkovError, Result};
use nalgebra::{DMatrix, DVector, RealField};
use num_traits::ToPrimitive;

/// Classification of a state in a Markov chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateType {
    /// Transient state (can leave and never return).
    Transient,
    /// Absorbing state (once entered, cannot leave).
    Absorbing,
}

/// A discrete-time Markov chain.
///
/// # Mathematical Background
///
/// A discrete-time Markov chain is a stochastic process {Xₙ} with the Markov property:
/// P(Xₙ₊₁ = j | X₀, X₁, ..., Xₙ) = P(Xₙ₊₁ = j | Xₙ)
///
/// The chain is characterized by a transition matrix P where P[i,j] = P(Xₙ₊₁ = j | Xₙ = i).
///
/// For chains with absorbing states, the canonical form is:
/// ```text
/// P = [ Q  R ]
///     [ 0  I ]
/// ```
/// where:
/// - Q: transient → transient transitions
/// - R: transient → absorbing transitions
/// - 0: absorbing → transient (impossible)
/// - I: absorbing → absorbing (identity)
///
/// The fundamental matrix is N = (I - Q)⁻¹, where N[i,j] is the expected number
/// of times the chain visits transient state j starting from transient state i.
///
/// # Example
///
/// ```rust
/// use math_explorer::pure_math::statistics::markov::dtmc::{MarkovChain, StateType};
/// use nalgebra::DMatrix;
///
/// // Simple 3-state chain: two transient states, one absorbing
/// let transition_matrix = DMatrix::from_row_slice(3, 3, &[
///     0.7, 0.2, 0.1,  // State 0 (transient)
///     0.3, 0.5, 0.2,  // State 1 (transient)
///     0.0, 0.0, 1.0,  // State 2 (absorbing)
/// ]);
///
/// let state_types = vec![
///     StateType::Transient,
///     StateType::Transient,
///     StateType::Absorbing,
/// ];
///
/// let chain = MarkovChain::<f64>::new(transition_matrix, state_types).unwrap();
///
/// // Compute expected number of visits
/// let fundamental = chain.fundamental_matrix().unwrap();
/// println!("Expected visits: {:?}", fundamental);
///
/// // Compute absorption probabilities
/// let absorption_probs = chain.absorption_probabilities().unwrap();
/// println!("Absorption probabilities: {:?}", absorption_probs);
/// ```
#[derive(Debug, Clone)]
pub struct MarkovChain<T: RealField + Copy + ToPrimitive> {
    /// The transition matrix P.
    transition_matrix: DMatrix<T>,
    /// Type of each state (transient or absorbing).
    state_types: Vec<StateType>,
    /// Indices of transient states.
    transient_indices: Vec<usize>,
    /// Indices of absorbing states.
    absorbing_indices: Vec<usize>,
}

impl<T: RealField + Copy + ToPrimitive> MarkovChain<T> {
    /// Creates a new Markov chain.
    ///
    /// # Arguments
    ///
    /// * `transition_matrix` - The transition matrix P where P[i,j] = P(next=j | current=i)
    /// * `state_types` - Classification of each state
    ///
    /// # Returns
    ///
    /// A new `MarkovChain` or an error if validation fails.
    ///
    /// # Errors
    ///
    /// - `DimensionMismatch`: If matrix size doesn't match state_types length
    /// - `NotStochastic`: If matrix rows don't sum to 1
    pub fn new(transition_matrix: DMatrix<T>, state_types: Vec<StateType>) -> Result<Self> {
        let n = transition_matrix.nrows();

        // Validate dimensions
        if transition_matrix.ncols() != n {
            return Err(MarkovError::DimensionMismatch {
                expected: n,
                actual: transition_matrix.ncols(),
            });
        }

        if state_types.len() != n {
            return Err(MarkovError::DimensionMismatch {
                expected: n,
                actual: state_types.len(),
            });
        }

        // Validate stochasticity
        Self::validate_stochastic(&transition_matrix)?;

        // Validate absorbing states
        for (i, state_type) in state_types.iter().enumerate() {
            if *state_type == StateType::Absorbing {
                // Check that P[i,i] = 1 and all other entries in row i are 0
                for j in 0..n {
                    let expected = if i == j { T::one() } else { T::zero() };
                    let actual = transition_matrix[(i, j)];
                    let tolerance = T::from_f64(1e-10).unwrap();
                    if (actual - expected).abs() > tolerance {
                        return Err(MarkovError::InvalidState {
                            reason: format!(
                                "State {} marked as absorbing but P[{},{}] = {} != {}",
                                i, i, j, actual.to_f64().unwrap_or(f64::NAN), expected.to_f64().unwrap_or(f64::NAN)
                            ),
                        });
                    }
                }
            }
        }

        // Separate transient and absorbing indices
        let transient_indices: Vec<usize> = state_types
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if *t == StateType::Transient {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let absorbing_indices: Vec<usize> = state_types
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if *t == StateType::Absorbing {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        Ok(MarkovChain {
            transition_matrix,
            state_types,
            transient_indices,
            absorbing_indices,
        })
    }

    /// Validates that a matrix is row-stochastic (each row sums to 1).
    fn validate_stochastic(matrix: &DMatrix<T>) -> Result<()> {
        let tolerance = T::from_f64(1e-10).unwrap();

        for i in 0..matrix.nrows() {
            let row_sum: T = matrix.row(i).iter().fold(T::zero(), |acc, &x| acc + x);
            if (row_sum - T::one()).abs() > tolerance {
                return Err(MarkovError::NotStochastic {
                    reason: format!("Row {} sums to {} instead of 1.0", i, row_sum.to_f64().unwrap_or(f64::NAN)),
                });
            }

            // Check all probabilities are valid
            for j in 0..matrix.ncols() {
                let p = matrix[(i, j)];
                if !p.is_finite() || p < T::zero() || p > T::one() {
                    return Err(MarkovError::InvalidProbability { value: p.to_f64().unwrap_or(f64::NAN) });
                }
            }
        }

        Ok(())
    }

    /// Returns the transition matrix.
    pub fn transition_matrix(&self) -> &DMatrix<T> {
        &self.transition_matrix
    }

    /// Returns the state types.
    pub fn state_types(&self) -> &[StateType] {
        &self.state_types
    }

    /// Returns the number of states.
    pub fn num_states(&self) -> usize {
        self.state_types.len()
    }

    /// Returns the number of transient states.
    pub fn num_transient(&self) -> usize {
        self.transient_indices.len()
    }

    /// Returns the number of absorbing states.
    pub fn num_absorbing(&self) -> usize {
        self.absorbing_indices.len()
    }

    /// Extracts the Q submatrix (transient → transient transitions).
    ///
    /// # Returns
    ///
    /// The Q submatrix of size (n_transient × n_transient).
    pub fn q_matrix(&self) -> DMatrix<T> {
        let n_t = self.transient_indices.len();
        let mut q = DMatrix::zeros(n_t, n_t);

        for (i, &idx_i) in self.transient_indices.iter().enumerate() {
            for (j, &idx_j) in self.transient_indices.iter().enumerate() {
                q[(i, j)] = self.transition_matrix[(idx_i, idx_j)];
            }
        }

        q
    }

    /// Extracts the R submatrix (transient → absorbing transitions).
    ///
    /// # Returns
    ///
    /// The R submatrix of size (n_transient × n_absorbing).
    pub fn r_matrix(&self) -> DMatrix<T> {
        let n_t = self.transient_indices.len();
        let n_a = self.absorbing_indices.len();
        let mut r = DMatrix::zeros(n_t, n_a);

        for (i, &idx_i) in self.transient_indices.iter().enumerate() {
            for (j, &idx_j) in self.absorbing_indices.iter().enumerate() {
                r[(i, j)] = self.transition_matrix[(idx_i, idx_j)];
            }
        }

        r
    }

    /// Computes the fundamental matrix N = (I - Q)⁻¹.
    ///
    /// # Mathematical Background
    ///
    /// The fundamental matrix N has the property that N[i,j] is the expected
    /// number of times the chain visits transient state j before absorption,
    /// given that it starts in transient state i.
    ///
    /// # Returns
    ///
    /// The fundamental matrix N of size (n_transient × n_transient).
    ///
    /// # Errors
    ///
    /// Returns `SingularMatrix` if (I - Q) is not invertible.
    pub fn fundamental_matrix(&self) -> Result<DMatrix<T>> {
        let q = self.q_matrix();
        let n_t = q.nrows();
        let i_minus_q = DMatrix::identity(n_t, n_t) - q;

        i_minus_q
            .try_inverse()
            .ok_or_else(|| MarkovError::SingularMatrix {
                context: "Cannot invert (I - Q) for fundamental matrix".to_string(),
            })
    }

    /// Computes absorption probabilities B = N·R.
    ///
    /// # Mathematical Background
    ///
    /// B[i,j] is the probability that the chain, starting from transient state i,
    /// will eventually be absorbed into absorbing state j.
    ///
    /// # Returns
    ///
    /// The absorption probability matrix B of size (n_transient × n_absorbing).
    ///
    /// # Errors
    ///
    /// Returns an error if the fundamental matrix cannot be computed.
    pub fn absorption_probabilities(&self) -> Result<DMatrix<T>> {
        if self.absorbing_indices.is_empty() {
            return Err(MarkovError::InvalidState {
                reason: "No absorbing states in chain".to_string(),
            });
        }

        let n = self.fundamental_matrix()?;
        let r = self.r_matrix();
        Ok(n * r)
    }

    /// Computes expected absorption times (expected number of steps before absorption).
    ///
    /// # Mathematical Background
    ///
    /// The expected absorption time from transient state i is t[i] = Σⱼ N[i,j].
    ///
    /// # Returns
    ///
    /// A vector of expected absorption times, one for each transient state.
    ///
    /// # Errors
    ///
    /// Returns an error if the fundamental matrix cannot be computed.
    pub fn expected_absorption_times(&self) -> Result<DVector<T>> {
        let n = self.fundamental_matrix()?;
        let n_t = n.nrows();
        let mut times = DVector::zeros(n_t);

        for i in 0..n_t {
            times[i] = n.row(i).iter().fold(T::zero(), |acc, &x| acc + x);
        }

        Ok(times)
    }

    /// Computes the n-step transition matrix P^n.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of steps
    ///
    /// # Returns
    ///
    /// The n-step transition matrix P^n.
    pub fn n_step_transition(&self, n: usize) -> DMatrix<T> {
        if n == 0 {
            return DMatrix::identity(self.num_states(), self.num_states());
        }
        if n == 1 {
            return self.transition_matrix.clone();
        }

        // Use repeated squaring for efficiency
        let mut result = DMatrix::identity(self.num_states(), self.num_states());
        let mut base = self.transition_matrix.clone();
        let mut power = n;

        while power > 0 {
            if power % 2 == 1 {
                result *= &base;
            }
            base = &base * &base;
            power /= 2;
        }

        result
    }

    /// Computes the stationary distribution (if it exists).
    ///
    /// # Mathematical Background
    ///
    /// A stationary distribution π satisfies π·P = π and Σᵢ πᵢ = 1.
    /// For irreducible, aperiodic chains, the stationary distribution exists
    /// and is unique. It can be found as the limit of P^n as n → ∞.
    ///
    /// # Returns
    ///
    /// The stationary distribution as a probability vector, or None if it
    /// doesn't exist or convergence fails.
    pub fn stationary_distribution(&self) -> Option<DVector<T>> {
        // For chains with absorbing states, only absorbing states have non-zero
        // stationary probability
        if !self.absorbing_indices.is_empty() {
            let mut pi = DVector::zeros(self.num_states());
            let n_absorbing = self.absorbing_indices.len();

            // Each absorbing state gets equal probability
            for &idx in &self.absorbing_indices {
                pi[idx] = T::one() / T::from_usize(n_absorbing).unwrap();
            }

            return Some(pi);
        }

        // For ergodic chains, use power method
        const MAX_ITERS: usize = 10000;
        let tolerance = T::from_f64(1e-12).unwrap();

        let mut pi = DVector::from_element(self.num_states(), T::one() / T::from_usize(self.num_states()).unwrap());

        for _ in 0..MAX_ITERS {
            let pi_next = self.transition_matrix.transpose() * &pi;

            // Check convergence
            let diff = (&pi_next - &pi).norm();
            if diff < tolerance {
                return Some(pi_next);
            }

            pi = pi_next;
        }

        None
    }

    /// Computes the Expected Possession Value (EPV) for each transient state.
    ///
    /// # Mathematical Background
    ///
    /// EPV is computed as:
    /// EPV = N · r
    ///
    /// where:
    /// - N is the fundamental matrix
    /// - r is a reward vector for absorbing states
    ///
    /// # Arguments
    ///
    /// * `rewards` - Reward for each absorbing state
    ///
    /// # Returns
    ///
    /// A vector of EPV values, one for each transient state.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The fundamental matrix cannot be computed
    /// - The reward vector size doesn't match the number of absorbing states
    pub fn expected_possession_value(&self, rewards: &DVector<T>) -> Result<DVector<T>> {
        if rewards.len() != self.num_absorbing() {
            return Err(MarkovError::DimensionMismatch {
                expected: self.num_absorbing(),
                actual: rewards.len(),
            });
        }

        let b = self.absorption_probabilities()?;
        Ok(b * rewards)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_simple_absorbing_chain() {
        // Two transient states, one absorbing
        let p = DMatrix::from_row_slice(
            3,
            3,
            &[
                0.5, 0.3, 0.2, // State 0 (transient)
                0.2, 0.6, 0.2, // State 1 (transient)
                0.0, 0.0, 1.0, // State 2 (absorbing)
            ],
        );

        let states = vec![
            StateType::Transient,
            StateType::Transient,
            StateType::Absorbing,
        ];

        let chain = MarkovChain::new(p, states).unwrap();

        assert_eq!(chain.num_transient(), 2);
        assert_eq!(chain.num_absorbing(), 1);

        // Test Q matrix
        let q = chain.q_matrix();
        assert_eq!(q.nrows(), 2);
        assert_eq!(q.ncols(), 2);
        assert_relative_eq!(q[(0, 0)], 0.5);
        assert_relative_eq!(q[(0, 1)], 0.3);
        assert_relative_eq!(q[(1, 0)], 0.2);
        assert_relative_eq!(q[(1, 1)], 0.6);

        // Test R matrix
        let r = chain.r_matrix();
        assert_eq!(r.nrows(), 2);
        assert_eq!(r.ncols(), 1);
        assert_relative_eq!(r[(0, 0)], 0.2);
        assert_relative_eq!(r[(1, 0)], 0.2);

        // Test fundamental matrix
        let n = chain.fundamental_matrix().unwrap();
        assert_eq!(n.nrows(), 2);
        assert_eq!(n.ncols(), 2);

        // Verify (I - Q) * N = I
        let q = chain.q_matrix();
        let i_minus_q = DMatrix::identity(2, 2) - q;
        let product = i_minus_q * n;
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_relative_eq!(product[(i, j)], expected, epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_absorption_probabilities() {
        // Classic gambler's ruin: states 0 and 4 are absorbing
        let p = DMatrix::from_row_slice(
            5,
            5,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, // State 0 (absorbing - ruin)
                0.5, 0.0, 0.5, 0.0, 0.0, // State 1
                0.0, 0.5, 0.0, 0.5, 0.0, // State 2
                0.0, 0.0, 0.5, 0.0, 0.5, // State 3
                0.0, 0.0, 0.0, 0.0, 1.0, // State 4 (absorbing - win)
            ],
        );

        let states = vec![
            StateType::Absorbing, // 0
            StateType::Transient, // 1
            StateType::Transient, // 2
            StateType::Transient, // 3
            StateType::Absorbing, // 4
        ];

        let chain = MarkovChain::new(p, states).unwrap();

        let absorption = chain.absorption_probabilities().unwrap();

        // absorption[i,0] = probability of reaching state 0 from transient state i
        // absorption[i,1] = probability of reaching state 4 from transient state i

        // For symmetric random walk, probability of reaching 0 from state i is (4-i)/4
        // and probability of reaching 4 is i/4

        // Transient state 1 (index 0 in absorption matrix)
        assert_relative_eq!(absorption[(0, 0)], 0.75, epsilon = 1e-10); // Reach 0
        assert_relative_eq!(absorption[(0, 1)], 0.25, epsilon = 1e-10); // Reach 4

        // Transient state 2 (index 1 in absorption matrix)
        assert_relative_eq!(absorption[(1, 0)], 0.5, epsilon = 1e-10);
        assert_relative_eq!(absorption[(1, 1)], 0.5, epsilon = 1e-10);

        // Transient state 3 (index 2 in absorption matrix)
        assert_relative_eq!(absorption[(2, 0)], 0.25, epsilon = 1e-10);
        assert_relative_eq!(absorption[(2, 1)], 0.75, epsilon = 1e-10);
    }

    #[test]
    fn test_expected_possession_value() {
        // Simple basketball example
        let p = DMatrix::from_row_slice(
            4,
            4,
            &[
                0.5, 0.3, 0.1, 0.1, // State 0: offense
                0.2, 0.4, 0.2, 0.2, // State 1: advantage
                0.0, 0.0, 1.0, 0.0, // State 2: score (absorbing, +2)
                0.0, 0.0, 0.0, 1.0, // State 3: turnover (absorbing, 0)
            ],
        );

        let states = vec![
            StateType::Transient,
            StateType::Transient,
            StateType::Absorbing, // Score
            StateType::Absorbing, // Turnover
        ];

        let chain = MarkovChain::new(p, states).unwrap();

        // Rewards: scoring gives 2 points, turnover gives 0
        let rewards = DVector::from_vec(vec![2.0, 0.0]);

        let epv = chain.expected_possession_value(&rewards).unwrap();

        // EPV should be positive for both transient states
        assert!(epv[0] > 0.0);
        assert!(epv[1] > 0.0);

        // State 1 (advantage) should have higher EPV than state 0
        assert!(epv[1] > epv[0]);
    }

    #[test]
    fn test_n_step_transition() {
        let p = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);

        let states = vec![StateType::Transient, StateType::Transient];
        let chain = MarkovChain::new(p.clone(), states).unwrap();

        // P^0 should be identity
        let p0 = chain.n_step_transition(0);
        assert_relative_eq!(p0[(0, 0)], 1.0);
        assert_relative_eq!(p0[(0, 1)], 0.0);
        assert_relative_eq!(p0[(1, 0)], 0.0);
        assert_relative_eq!(p0[(1, 1)], 1.0);

        // P^1 should be P
        let p1 = chain.n_step_transition(1);
        assert_relative_eq!(p1[(0, 0)], 0.7);
        assert_relative_eq!(p1[(0, 1)], 0.3);

        // P^2 = P * P
        let p2 = chain.n_step_transition(2);
        let p2_direct = &p * &p;
        for i in 0..2 {
            for j in 0..2 {
                assert_relative_eq!(p2[(i, j)], p2_direct[(i, j)], epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_stationary_distribution_ergodic() {
        // Simple ergodic chain
        let p = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);

        let states = vec![StateType::Transient, StateType::Transient];
        let chain = MarkovChain::new(p, states).unwrap();

        let pi = chain.stationary_distribution().unwrap();

        // Should sum to 1
        assert_relative_eq!(pi.sum(), 1.0, epsilon = 1e-10);

        // Should satisfy π·P = π
        let pi_p = chain.transition_matrix().transpose() * &pi;
        for i in 0..2 {
            assert_relative_eq!(pi[i], pi_p[i], epsilon = 1e-10);
        }

        // For this specific chain, the stationary distribution is [4/7, 3/7]
        assert_relative_eq!(pi[0], 4.0 / 7.0, epsilon = 1e-10);
        assert_relative_eq!(pi[1], 3.0 / 7.0, epsilon = 1e-10);
    }

    #[test]
    fn test_validation_errors() {
        // Test non-square matrix
        let p = DMatrix::from_row_slice(2, 3, &[0.5, 0.3, 0.2, 0.3, 0.4, 0.3]);
        let states = vec![StateType::Transient, StateType::Transient];
        assert!(MarkovChain::new(p, states).is_err());

        // Test row that doesn't sum to 1
        let p = DMatrix::from_row_slice(2, 2, &[0.5, 0.3, 0.4, 0.6]);
        let states = vec![StateType::Transient, StateType::Transient];
        assert!(MarkovChain::new(p, states).is_err());

        // Test invalid absorbing state
        let p = DMatrix::from_row_slice(2, 2, &[0.7, 0.3, 0.4, 0.6]);
        let states = vec![StateType::Absorbing, StateType::Transient];
        assert!(MarkovChain::new(p, states).is_err());
    }
}
