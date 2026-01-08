use std::collections::HashMap;
use std::hash::Hash;
use super::types::{Action, State};

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

/// Simple Tabular Q-Agent for discrete states and actions.
pub struct TabularQAgent<S, A>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    q_table: HashMap<(S, A), f64>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64, // For epsilon-greedy exploration
}

impl<S, A> TabularQAgent<S, A>
where
    S: State + Hash + Eq + Copy,
    A: Action + Hash + Eq + Copy,
{
    pub fn new(learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    pub fn get_q_value(&self, state: &S, action: &A) -> f64 {
        *self.q_table.get(&(*state, *action)).unwrap_or(&0.0)
    }

    pub fn update(&mut self, state: &S, action: &A, reward: f64, next_state: &S, possible_next_actions: &[A]) {
        let current_q = self.get_q_value(state, action);

        let max_next_q = if possible_next_actions.is_empty() {
            0.0
        } else {
            possible_next_actions.iter()
                .map(|a| self.get_q_value(next_state, a))
                .fold(f64::NEG_INFINITY, f64::max)
        };
        // Handle case where max_next_q is still NEG_INFINITY (e.g. no entries yet and fold default?)
        // actually fold with NEG_INFINITY on empty list is problematic if not checked, but we check is_empty.
        // If map returns 0.0 for defaults, then max is at least 0.0.

        let new_q = q_learning_update(
            current_q,
            reward,
            if max_next_q == f64::NEG_INFINITY { 0.0 } else { max_next_q },
            self.learning_rate,
            self.discount_factor,
        );

        self.q_table.insert((*state, *action), new_q);
    }

    /// Selects an action using Epsilon-Greedy strategy.
    pub fn select_action(&self, state: &S, available_actions: &[A]) -> Option<A> {
        if available_actions.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        use rand::Rng;

        if rng.r#gen::<f64>() < self.epsilon {
            // Explore: Random action
            let index = rng.gen_range(0..available_actions.len());
            Some(available_actions[index])
        } else {
            // Exploit: Best action
            // Shuffle to break ties randomly? Or just take first best.
            // For simplicity, take first best.
            available_actions.iter()
                .max_by(|a, b| {
                    let qa = self.get_q_value(state, a);
                    let qb = self.get_q_value(state, b);
                    qa.partial_cmp(&qb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .copied()
        }
    }
}

/// Policy Gradient Update (REINFORCE rule sketch).
/// $\theta \leftarrow \theta + \alpha \gamma^t G_t \nabla \ln \pi(a_t | s_t, \theta)$
///
/// This function calculates the gradient component for a single step.
/// * `return_gt`: The cumulative return $G_t$.
/// * `grad_log_pi`: The gradient of the log probability of the action taken.
pub fn policy_gradient_step(
    return_gt: f64,
    grad_log_pi: &[f64],
    learning_rate: f64,
) -> Vec<f64> {
    grad_log_pi.iter().map(|g| learning_rate * return_gt * g).collect()
}
