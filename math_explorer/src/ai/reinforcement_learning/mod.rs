//! # Reinforcement Learning
//!
//! This module provides a framework for defining and solving Markov Decision Processes (MDPs)
//! using standard Reinforcement Learning algorithms.
//!
//! ## 🧠 Core Concepts
//!
//! Reinforcement Learning involves an **Agent** interacting with an **Environment** to maximize cumulative **Reward**.
//!
//! ```mermaid
//! graph LR
//!     Agent[🤖 Agent]
//!     Env[🌍 Environment]
//!
//!     Agent -->|Action $a_t$| Env
//!     Env -->|State $s_{t+1}$| Agent
//!     Env -->|Reward $r_{t+1}$| Agent
//!
//!     style Agent fill:#f9f,stroke:#333,stroke-width:2px
//!     style Env fill:#bbf,stroke:#333,stroke-width:2px
//! ```
//!
//! ### Key Components
//!
//! 1.  **State ($S$)**: A representation of the environment at a specific time.
//! 2.  **Action ($A$)**: A decision made by the agent.
//! 3.  **Policy ($\pi$)**: The agent's strategy, mapping states to actions ($\pi(a|s)$).
//! 4.  **Value Function ($V_\pi(s)$)**: The expected cumulative reward from state $s$ under policy $\pi$.
//!
//! ## 📐 Mathematical Foundation
//!
//! We rely on the **Bellman Equations** to solve for optimal policies.
//!
//! **The Bellman Optimality Equation for $Q^*$:**
//! $$ Q^*(s, a) = \sum_{s'} P(s'|s, a) \left[ R(s, a, s') + \gamma \max_{a'} Q^*(s', a') \right] $$
//!
//! Where:
//! *   $P(s'|s, a)$ is the transition probability.
//! *   $R(s, a, s')$ is the immediate reward.
//! *   $\gamma$ is the discount factor ($0 \le \gamma \le 1$).
//!
//! ## 🚀 Quick Start: Solving a Grid World
//!
//! Here is how to define a simple MDP and solve it using Tabular Q-Learning.
//!
//! ```rust
//! use math_explorer::ai::reinforcement_learning::types::{MarkovDecisionProcess, State, Action};
//! use math_explorer::ai::reinforcement_learning::algorithms::TabularQAgent;
//!
//! // 1. Define State and Action
//! #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//! struct GridState(usize); // Position 0..3
//!
//! #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//! enum Move { Left, Right }
//!
//! impl State for GridState {}
//! impl Action for Move {}
//!
//! // 2. Define the Environment (MDP)
//! struct CorridorEnv;
//!
//! impl MarkovDecisionProcess for CorridorEnv {
//!     type S = GridState;
//!     type A = Move;
//!
//!     fn transition_probability(&self, next: &GridState, curr: &GridState, action: &Move) -> f64 {
//!         let target = match (curr.0, action) {
//!             (0, Move::Left) => 0,
//!             (0, Move::Right) => 1,
//!             (1, Move::Left) => 0,
//!             (1, Move::Right) => 2,
//!             (2, Move::Left) => 1,
//!             (2, Move::Right) => 3, // Goal
//!             (3, _) => 3, // Terminal
//!             _ => curr.0,
//!         };
//!         if next.0 == target { 1.0 } else { 0.0 }
//!     }
//!
//!     fn reward(&self, _curr: &GridState, _action: &Move, next: &GridState) -> f64 {
//!         if next.0 == 3 { 10.0 } else { -1.0 } // Reward for goal, penalty for time
//!     }
//!
//!     fn actions(&self, _state: &GridState) -> Vec<Move> {
//!         vec![Move::Left, Move::Right]
//!     }
//!
//!     fn discount_factor(&self) -> f64 { 0.9 }
//!     fn is_terminal(&self, state: &GridState) -> bool { state.0 == 3 }
//! }
//!
//! // 3. Train the Agent
//! fn main() {
//!     let env = CorridorEnv;
//!     let mut agent = TabularQAgent::new(0.1, env.discount_factor(), 0.1);
//!
//!     // Training Loop
//!     for _episode in 0..500 {
//!         let mut state = GridState(0);
//!         while !env.is_terminal(&state) {
//!             let action = agent.select_action(&state, &env.actions(&state)).unwrap();
//!
//!             // Simulate transition (deterministic here)
//!             // Note: In a real scenario, you sample next_state based on probability.
//!             // For this doc example, we hardcode the transition logic to match the env.
//!             let next_state = match (state.0, action) {
//!                 (0, Move::Right) => GridState(1),
//!                 (1, Move::Right) => GridState(2),
//!                 (2, Move::Right) => GridState(3),
//!                 (x, Move::Left) if x > 0 => GridState(x - 1),
//!                 (x, _) => GridState(x),
//!             };
//!
//!             let reward = env.reward(&state, &action, &next_state);
//!
//!             agent.update(&state, &action, reward, &next_state, &env.actions(&next_state));
//!             state = next_state;
//!         }
//!     }
//!
//!     // 4. Verify Policy (Should go Right)
//!     let best_action = agent.select_action(&GridState(0), &[Move::Left, Move::Right]);
//!     assert_eq!(best_action, Some(Move::Right));
//! }
//! ```

pub mod algorithms;
pub mod bellman;
pub mod types;
