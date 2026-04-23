//! # Group Relative Policy Optimization (GRPO)
//!
//! A Reinforcement Learning algorithm designed for reasoning tasks, which optimizes a policy
//! by evaluating a *group* of outputs for a given input, rather than a single output.
//!
//! GRPO estimates the "advantage" of a response by comparing its reward to the average
//! reward of other responses in the same group. This reduces gradient variance without
//! needing a separate Value Network (Critic).
//!
//! ##  The Optimization Loop
//!
//! ```mermaid
//! flowchart LR
//!     Policy[Policy π_θ] -->|Sample G outputs| Group[Output Group]
//!     Group -->|Evaluate| Rewards[Rewards]
//!     Rewards -->|Normalize| Adv[Advantages]
//!     Adv -->|Update θ| Obj[Clipped Surrogate Objective]
//!     Obj --> Policy
//! ```
//!
//! ##  Quick Start: Calculating the Objective
//!
//! Calculate the GRPO loss (clipped surrogate objective) for a hypothetical training step.
//!
//! ```rust
//! use math_explorer::applied::grpo::formulas::{clipped_surrogate_objective, response_level_advantage};
//!
//! // 1. Simulate a group of 3 outputs with raw rewards
//! let rewards = vec![1.0, 0.5, 0.2];
//!
//! // 2. Calculate Advantages (normalized z-scores)
//! // Mean = 0.566, StdDev ≈ 0.404
//! let adv_0 = response_level_advantage(&rewards, rewards[0]); // High advantage
//! let adv_1 = response_level_advantage(&rewards, rewards[1]); // Near zero
//! let adv_2 = response_level_advantage(&rewards, rewards[2]); // Negative advantage
//!
//! // 3. Compute Objective
//! // Assume current policy probabilities (pi) and old policy (pi_old)
//! // If pi > pi_old for a good advantage (adv_0), objective increases.
//! let pi_thetas = vec![0.6, 0.3, 0.1];
//! let pi_olds   = vec![0.5, 0.3, 0.2];
//! let advantages = vec![adv_0, adv_1, adv_2];
//!
//! let loss = clipped_surrogate_objective(
//!     &pi_thetas,
//!     &pi_olds,
//!     &advantages,
//!     0.2, // Epsilon (clipping range 0.8 - 1.2)
//!     0.01, // Beta (KL penalty)
//!     0.05  // KL Divergence
//! );
//!
//! println!("GRPO Loss: {:.4}", loss);
//! ```

pub mod formulas;
pub mod metrics;
pub mod rewards;
