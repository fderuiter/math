# Markov Chains Module

Comprehensive implementation of Markov processes for the `math_explorer` crate.

## Module Structure

```
markov/
├── error.rs       - Error types for Markov operations
├── dtmc.rs        - Discrete-Time Markov Chains
├── ctmc.rs        - Continuous-Time Markov Chains  
├── tensor.rs      - Time-indexed transition tensors
├── hmm.rs         - Hidden Markov Models
└── mod.rs         - Module documentation and exports
```

## Features

### Discrete-Time Markov Chains (DTMC)
-  Transient and absorbing state classification
-  Canonical form decomposition (Q, R matrices)
-  Fundamental matrix N = (I - Q)⁻¹
-  Expected Possession Value (EPV) calculations
-  Stationary distributions for ergodic chains
-  Absorption probabilities and expected times
-  N-step transition matrices with efficient computation

### Time-Indexed Transitions
-  Non-stationary chains with time-varying transitions
-  Linear interpolation between time slices
-  Average transitions over time intervals
-  Shot clock urgency modeling for basketball

### Continuous-Time Markov Chains (CTMC)
-  Generator matrix validation
-  Matrix exponential P(t) = exp(Gt) via Padé approximation
-  Steady-state distribution computation
-  Gillespie algorithm for trajectory simulation
-  Expected absorption times

### Hidden Markov Models (HMM)
-  Forward algorithm (observation probability)
-  Backward algorithm
-  Viterbi algorithm (most likely state sequence)
-  Forward-Backward (posterior probabilities)
-  Sequence generation with deterministic RNG
-  Filtering (current state belief)

## Examples

### 1. Expected Possession Value (Basketball)

Calculate the expected points from a possession given transition dynamics:

```rust
use math_explorer::pure_math::statistics::markov::dtmc::{MarkovChain, StateType};
use nalgebra::{DMatrix, DVector};

// Define states: offense, advantage, score, turnover
let transitions = DMatrix::from_row_slice(4, 4, &[
    0.50, 0.30, 0.15, 0.05,  // Offense: stay, advance, score, turnover
    0.20, 0.40, 0.30, 0.10,  // Advantage: better scoring position
    0.00, 0.00, 1.00, 0.00,  // Score (absorbing state)
    0.00, 0.00, 0.00, 1.00,  // Turnover (absorbing state)
]);

let state_types = vec![
    StateType::Transient,  // Offense
    StateType::Transient,  // Advantage
    StateType::Absorbing,  // Score
    StateType::Absorbing,  // Turnover
];

let chain = MarkovChain::new(transitions, state_types).unwrap();

// Rewards: +2 points for score, 0 for turnover
let rewards = DVector::from_vec(vec![2.0, 0.0]);
let epv = chain.expected_possession_value(&rewards).unwrap();

println!("EPV from offense state: {:.3} points", epv[0]);
println!("EPV from advantage state: {:.3} points", epv[1]);

// Also compute absorption probabilities
let absorption = chain.absorption_probabilities().unwrap();
println!("P(score | start in offense) = {:.3}", absorption[(0, 0)]);
println!("P(score | start in advantage) = {:.3}", absorption[(1, 0)]);
```

### 2. Shot Clock Urgency (Time-Varying Transitions)

Model how basketball strategy changes as the shot clock runs down:

```rust
use math_explorer::pure_math::statistics::markov::tensor::{TransitionTensor, TimeIndex};
use nalgebra::DMatrix;

// Create tensor spanning shot clock (0-24 seconds)
let mut tensor = TransitionTensor::new(
    3,
    TimeIndex::new(0.0).unwrap(),
    TimeIndex::new(24.0).unwrap()
);

// States: 0=possessing, 1=shot, 2=turnover

// At t=24 (full shot clock): patient offense
let p_24 = DMatrix::from_row_slice(3, 3, &[
    0.85, 0.10, 0.05,  // Mostly maintain possession
    0.00, 1.00, 0.00,  // Shot (absorbing)
    0.00, 0.00, 1.00,  // Turnover (absorbing)
]);
tensor.add_time_slice(TimeIndex::new(24.0).unwrap(), p_24).unwrap();

// At t=3 (expiring shot clock): urgent offense
let p_3 = DMatrix::from_row_slice(3, 3, &[
    0.30, 0.60, 0.10,  // Must shoot quickly
    0.00, 1.00, 0.00,  // Shot (absorbing)
    0.00, 0.00, 1.00,  // Turnover (absorbing)
]);
tensor.add_time_slice(TimeIndex::new(3.0).unwrap(), p_3).unwrap();

// Query transition matrix at any time (automatically interpolated)
let p_12 = tensor.transition_matrix_at(TimeIndex::new(12.0).unwrap()).unwrap();
println!("Shot probability at 12 seconds: {:.3}", p_12[(0, 1)]);

// Average transition over a time window
let p_avg = tensor.average_transition(
    TimeIndex::new(10.0).unwrap(),
    TimeIndex::new(20.0).unwrap(),
    50  // 50 sample points
).unwrap();
```

### 3. Hot Hand Detection (HMM)

Detect shooting streaks using Hidden Markov Models:

```rust
use math_explorer::pure_math::statistics::markov::hmm::HiddenMarkovModel;
use nalgebra::{DMatrix, DVector};

// Hidden states: Cold (0), Hot (1)
// Observations: Miss (0), Make (1)

let initial = DVector::from_vec(vec![0.5, 0.5]);

// State transitions
let transitions = DMatrix::from_row_slice(2, 2, &[
    0.70, 0.30,  // Cold → Cold (0.7), Cold → Hot (0.3)
    0.40, 0.60,  // Hot → Cold (0.4), Hot → Hot (0.6)
]);

// Emission probabilities
let emissions = DMatrix::from_row_slice(2, 2, &[
    0.70, 0.30,  // Cold: 70% miss, 30% make
    0.20, 0.80,  // Hot: 20% miss, 80% make
]);

let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

// Observed shooting sequence: Make, Make, Make, Miss, Make
let shots = vec![1, 1, 1, 0, 1];

// Find most likely state sequence
let states = hmm.viterbi(&shots).unwrap();
println!("Most likely states: {:?}", states);
// Expected: [1, 1, 1, 1, 1] or similar (hot state after makes)

// Current belief about being hot (filtering)
let posterior = hmm.filter(&shots).unwrap();
println!("P(Hot | all shots) = {:.3}", posterior[1]);

// Full posterior probabilities over time (smoothing)
let gamma = hmm.posterior_probabilities(&shots).unwrap();
for t in 0..shots.len() {
    println!("t={}: P(Hot) = {:.3}", t, gamma[(1, t)]);
}
```

### 4. Gambler's Ruin (Classic DTMC)

```rust
use math_explorer::pure_math::statistics::markov::dtmc::{MarkovChain, StateType};
use nalgebra::DMatrix;

// Gambler starts with $2 (states 0-4 representing $0 to $4)
// Win/lose $1 with equal probability
// Absorbing at $0 (ruin) and $4 (target)

let transitions = DMatrix::from_row_slice(5, 5, &[
    1.0, 0.0, 0.0, 0.0, 0.0,  // State 0: ruin (absorbing)
    0.5, 0.0, 0.5, 0.0, 0.0,  // State 1: $1
    0.0, 0.5, 0.0, 0.5, 0.0,  // State 2: $2
    0.0, 0.0, 0.5, 0.0, 0.5,  // State 3: $3
    0.0, 0.0, 0.0, 0.0, 1.0,  // State 4: target (absorbing)
]);

let states = vec![
    StateType::Absorbing,   // $0
    StateType::Transient,   // $1
    StateType::Transient,   // $2
    StateType::Transient,   // $3
    StateType::Absorbing,   // $4
];

let chain = MarkovChain::new(transitions, states).unwrap();

// Probability of reaching target from each starting position
let absorption = chain.absorption_probabilities().unwrap();
for i in 0..3 {
    println!("Starting with ${}: P(reach $4) = {:.3}", 
             i + 1, absorption[(i, 1)]);
}
// Expected: 0.25, 0.50, 0.75 (linear in symmetric random walk)

// Expected time until absorption
let times = chain.expected_absorption_times().unwrap();
println!("Expected games from $2: {:.1}", times[1]);
```

### 5. Birth-Death Process (CTMC)

```rust
use math_explorer::pure_math::statistics::markov::ctmc::ContinuousMarkovChain;
use nalgebra::DMatrix;
use rand::SeedableRng;
use rand::rngs::StdRng;

// Two-state birth-death: 
// State 0 → State 1 at rate λ=2.0
// State 1 → State 0 at rate μ=3.0

let generator = DMatrix::from_row_slice(2, 2, &[
    -2.0,  2.0,   // State 0: birth rate 2
     3.0, -3.0,   // State 1: death rate 3
]);

let chain = ContinuousMarkovChain::new(generator).unwrap();

// Steady-state distribution
if let Some(pi) = chain.steady_state() {
    println!("π(0) = {:.3}, π(1) = {:.3}", pi[0], pi[1]);
    // Expected: π(0) = 0.6, π(1) = 0.4 (ratio μ:λ = 3:2)
}

// Transition probabilities at t=1.0
let p_t = chain.transition_probabilities(1.0).unwrap();
println!("P(0→1, t=1) = {:.3}", p_t[(0, 1)]);

// Simulate trajectory
let mut rng = StdRng::seed_from_u64(42);
let trajectory = chain.simulate_trajectory(0, 10.0, &mut rng).unwrap();
println!("Trajectory: {:?}", trajectory);
// Output: [(time, state), ...] showing jumps between states
```

### 6. Market Regime Detection (HMM)

```rust
use math_explorer::pure_math::statistics::markov::hmm::HiddenMarkovModel;
use nalgebra::{DMatrix, DVector};

// Hidden states: Bull (0), Bear (1), Sideways (2)
// Observations: Large Up (0), Small Up (1), Flat (2), Small Down (3), Large Down (4)

let initial = DVector::from_vec(vec![0.33, 0.33, 0.34]);

let transitions = DMatrix::from_row_slice(3, 3, &[
    0.70, 0.20, 0.10,  // Bull tends to persist
    0.20, 0.70, 0.10,  // Bear tends to persist
    0.25, 0.25, 0.50,  // Sideways less persistent
]);

let emissions = DMatrix::from_row_slice(3, 5, &[
    0.30, 0.30, 0.20, 0.10, 0.10,  // Bull: mostly up
    0.10, 0.10, 0.20, 0.30, 0.30,  // Bear: mostly down
    0.10, 0.20, 0.40, 0.20, 0.10,  // Sideways: centered
]);

let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();

// Observed market moves: Up, Up, Flat, Down, Down
let observations = vec![1, 0, 2, 3, 4];

// Infer market regime
let states = hmm.viterbi(&observations).unwrap();
println!("Inferred regimes: {:?}", states);
// Likely: [0, 0, 2, 1, 1] (bull → sideways → bear transition)

// Probability of being in each regime now
let current_belief = hmm.filter(&observations).unwrap();
println!("P(Bull) = {:.3}", current_belief[0]);
println!("P(Bear) = {:.3}", current_belief[1]);
println!("P(Sideways) = {:.3}", current_belief[2]);
```

## Design Principles

### 1. Strong Typing
All domain concepts use newtypes to prevent misuse:
- `TimeIndex` for time-indexed transitions
- `StateType` enum for state classification
- Proper error types instead of strings

### 2. Deterministic Randomness
All stochastic methods accept an explicit RNG:
```rust
let mut rng = StdRng::seed_from_u64(42);
let trajectory = chain.simulate_trajectory(0, 10.0, &mut rng).unwrap();
// Reproducible across runs with same seed
```

### 3. Comprehensive Validation
- Transition matrices validated as stochastic (rows sum to 1)
- Generator matrices validated (rows sum to 0, proper signs)
- Dimension mismatches caught at construction
- Clear error messages with context

### 4. Numerical Stability
- Forward-Backward algorithm uses scaling to prevent underflow
- Matrix exponential uses Padé approximation with scaling/squaring
- Appropriate tolerances for floating-point comparisons

## Testing

Run all Markov chain tests:
```bash
cargo test --lib pure_math::statistics::markov
```

All tests include:
- Unit tests for individual functions
- Integration tests for complete workflows
- Deterministic RNG tests (same seed → same output)
- Known-result validation against literature

## Performance Notes

- N-step transitions use repeated squaring: O(log n) multiplications
- Matrix exponential via Padé(6,6): accurate and stable
- Forward-Backward scales O(T·N²) for T observations, N states
- Viterbi algorithm runs in O(T·N²) time

## References

### Books
1. Norris, J.R. (1997). *Markov Chains*. Cambridge University Press.
2. Ross, S.M. (1996). *Stochastic Processes* (2nd ed.). Wiley.
3. Rabiner, L.R. (1989). "A tutorial on hidden Markov models". *Proceedings of the IEEE*, 77(2).

### Papers
4. Viterbi, A.J. (1967). "Error bounds for convolutional codes". *IEEE Trans. IT*, 13(2).
5. Gillespie, D.T. (1977). "Exact stochastic simulation of coupled chemical reactions". *J. Phys. Chem.*, 81(25).

## Integration with math_explorer

This module follows all `math_explorer` architectural principles:
-  No God Files (each file < 500 lines, single responsibility)
-  Strong typing with newtypes
-  Dependency injection (RNG, not thread_rng())
-  Manual error handling (no thiserror)
-  Comprehensive documentation
-  Academic rigor with citations
-  Extensive test coverage

## Future Extensions

Possible additions:
- [ ] Baum-Welch algorithm (HMM parameter learning)
- [ ] Variable-order Markov chains
- [ ] Semi-Markov processes (non-exponential holding times)
- [ ] Markov Decision Processes (MDPs)
- [ ] Partially Observable MDPs (POMDPs)
- [ ] Monte Carlo Markov Chain (MCMC) methods
