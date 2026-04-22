use super::q_function::TabularQFunction;
use super::strategies::{EpsilonGreedy, ExplorationStrategy};
use super::types::{Action, QFunction, State};
use rand::RngCore;
use std::hash::Hash;

/// Q-Learning Update Rule.
/// $Q(s, a) \leftarrow Q(s, a) + \alpha [R + \gamma \max_{a'} Q(s', a') - Q(s, a)]$
pub fn q_learning_update(
    current_q: f64,
    reward: f64,
    max_next_q: f64,
    alpha: f64,
    gamma: f64,
) -> f64 {
    let target = reward + gamma * max_next_q;
    current_q + alpha * (target - current_q)
}

/// A generic Q-Learning Agent using the Strategy Pattern for Q-Function storage.
///
/// This agent is decoupled from the storage mechanism, allowing for:
/// - Tabular storage (via `TabularQFunction`)
/// - Function Approximation (e.g., Linear, Neural Networks)
pub struct QLearningAgent<S, A, Q>
where
    S: State,
    A: Action,
    Q: QFunction<S, A>,
{
    pub q_func: Q,
    learning_rate: f64,
    discount_factor: f64,
    strategy: Box<dyn ExplorationStrategy<S, A>>,
}

/// Legacy alias for the Tabular Q-Learning Agent.
///
/// This preserves backward compatibility for existing code using `TabularQAgent`.
pub type TabularQAgent<S, A> = QLearningAgent<S, A, TabularQFunction<S, A>>;

impl<S, A, Q> QLearningAgent<S, A, Q>
where
    S: State,
    A: Action,
    Q: QFunction<S, A>,
{
    /// Creates a new generic Q-Learning agent.
    pub fn new_generic(
        q_func: Q,
        learning_rate: f64,
        discount_factor: f64,
        strategy: Box<dyn ExplorationStrategy<S, A>>,
    ) -> Self {
        Self {
            q_func,
            learning_rate,
            discount_factor,
            strategy,
        }
    }

    pub fn get_q_value(&self, state: &S, action: &A) -> f64 {
        self.q_func.value(state, action)
    }

    pub fn update(
        &mut self,
        state: &S,
        action: &A,
        reward: f64,
        next_state: &S,
        possible_next_actions: &[A],
    ) {
        let current_q = self.get_q_value(state, action);

        let max_next_q = if possible_next_actions.is_empty() {
            0.0
        } else {
            possible_next_actions
                .iter()
                .map(|a| self.get_q_value(next_state, a))
                .fold(f64::NEG_INFINITY, f64::max)
        };
        // Handle case where max_next_q is still NEG_INFINITY (e.g. no entries yet and fold default?)
        // actually fold with NEG_INFINITY on empty list is problematic if not checked, but we check is_empty.
        // If map returns 0.0 for defaults, then max is at least 0.0.

        let new_q = q_learning_update(
            current_q,
            reward,
            if max_next_q == f64::NEG_INFINITY {
                0.0
            } else {
                max_next_q
            },
            self.learning_rate,
            self.discount_factor,
        );

        self.q_func.update(state, action, new_q);
    }

    /// Selects an action using the injected strategy.
    /// This method uses the default thread-local RNG.
    pub fn select_action(&self, state: &S, available_actions: &[A]) -> Option<A> {
        let mut rng = rand::thread_rng();
        self.select_action_with_rng(state, available_actions, &mut rng)
    }

    /// Selects an action using the injected strategy and a provided RNG.
    /// Useful for deterministic testing.
    pub fn select_action_with_rng(
        &self,
        state: &S,
        available_actions: &[A],
        rng: &mut dyn RngCore,
    ) -> Option<A> {
        let q_values: Vec<f64> = available_actions
            .iter()
            .map(|a| self.get_q_value(state, a))
            .collect();

        self.strategy
            .select_action(state, available_actions, &q_values, rng)
    }
}

// Backward compatibility implementation for TabularQAgent
impl<S, A> QLearningAgent<S, A, TabularQFunction<S, A>>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    /// Creates a new Tabular Q-Agent.
    pub fn new(learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        Self {
            q_func: TabularQFunction::new(),
            learning_rate,
            discount_factor,
            strategy: Box::new(EpsilonGreedy::new(epsilon)),
        }
    }

    /// Creates a new Tabular Q-Agent with a custom exploration strategy.
    pub fn new_with_strategy(
        learning_rate: f64,
        discount_factor: f64,
        strategy: Box<dyn ExplorationStrategy<S, A>>,
    ) -> Self {
        Self {
            q_func: TabularQFunction::new(),
            learning_rate,
            discount_factor,
            strategy,
        }
    }
}

/// Policy Gradient Update (REINFORCE rule sketch).
/// $\theta \leftarrow \theta + \alpha \gamma^t G_t \nabla \ln \pi(a_t | s_t, \theta)$
///
/// This function calculates the gradient component for a single step.
/// * `return_gt`: The cumulative return $G_t$.
/// * `grad_log_pi`: The gradient of the log probability of the action taken.
pub fn policy_gradient_step(return_gt: f64, grad_log_pi: &[f64], learning_rate: f64) -> Vec<f64> {
    grad_log_pi
        .iter()
        .map(|g| learning_rate * return_gt * g)
        .collect()
}
