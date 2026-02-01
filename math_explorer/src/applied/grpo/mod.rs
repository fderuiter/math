//! # Group Relative Policy Optimization (GRPO)
//!
//! GRPO is a Reinforcement Learning algorithm designed to optimize language models without requiring
//! a separate "Critic" network (as used in PPO). Instead of estimating a baseline value function,
//! GRPO samples a **group** of outputs for the same input and uses the group's average reward
//! as the baseline.
//!
//! ## Theory
//!
//! In standard PPO (Proximal Policy Optimization), we need a Value Function $V(s)$ to estimate
//! how good a state is. GRPO eliminates this by:
//!
//! 1.  Sampling a group of $G$ outputs $\{o_1, o_2, ..., o_G\}$ for a single input $q$.
//! 2.  Calculating the reward $r_i$ for each output.
//! 3.  Computing the **advantage** $A_i$ by normalizing the reward against the group's statistics:
//!     $$ A_i = \frac{r_i - \text{mean}(\{r_1...r_G\})}{\text{std}(\{r_1...r_G\}) + \epsilon} $$
//!
//! This "relative" advantage encourages the model to prefer outputs that are better than the
//! other outputs it just generated, effectively using itself as the baseline.
//!
//! ## Pipeline
//!
//! ```mermaid
//! graph TD
//!     Input[Question q] --> Policy[Policy Model]
//!     Policy --> |Sample G times| Group[Group of Outputs<br/>o1, o2, ..., oG]
//!     Group --> RewardModel[Reward Model/Rules]
//!     RewardModel --> Rewards[Rewards<br/>r1, r2, ..., rG]
//!
//!     subgraph Optimization
//!     Rewards --> MeanStd[Calculate Mean & Std]
//!     MeanStd --> Advantage[Compute Advantage Ai]
//!     Advantage --> Surrogate[Clipped Surrogate Objective]
//!     Surrogate --> Update[Update Policy Weights]
//!     end
//!
//!     style Group fill:#e1f5fe,stroke:#01579b
//!     style Rewards fill:#e8f5e9,stroke:#2e7d32
//! ```
//!
//! ## 🚀 Quick Start: Manual Optimization Step
//!
//! This example demonstrates how to calculate the advantages and the objective function
//! for a hypothetical training step.
//!
//! ```rust
//! use math_explorer::applied::grpo::formulas::{response_level_advantage, clipped_surrogate_objective};
//!
//! // 1. Simulate a group of 4 outputs with their calculated rewards
//! // e.g., Output 1 got 0.8 reward, Output 2 got 0.9, etc.
//! let rewards = vec![0.8, 0.9, 0.5, 0.6];
//!
//! // 2. Calculate Advantages (Normalized against the group)
//! // High rewards should have positive advantages, low rewards negative.
//! let advantages: Vec<f64> = rewards
//!     .iter()
//!     .map(|&r| response_level_advantage(&rewards, r))
//!     .collect();
//!
//! println!("Advantages: {:.2?}", advantages);
//! // Expect: [0.57, 1.14, -1.14, -0.57] (approx)
//!
//! // 3. Compute the Optimization Objective
//! // Assume policy probabilities changed slightly since sampling
//! let current_probs = vec![0.55, 0.65, 0.35, 0.45]; // pi_theta
//! let old_probs     = vec![0.50, 0.60, 0.40, 0.40]; // pi_theta_old
//!
//! let objective = clipped_surrogate_objective(
//!     &current_probs,
//!     &old_probs,
//!     &advantages,
//!     0.2, // epsilon (clipping range)
//!     0.1, // beta (KL penalty weight)
//!     0.01 // KL divergence (simplified)
//! );
//!
//! println!("Objective Value: {:.4}", objective);
//! ```

pub mod formulas;
pub mod metrics;
pub mod rewards;
