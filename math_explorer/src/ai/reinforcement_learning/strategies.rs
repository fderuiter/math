use super::types::Action;
use rand::{Rng, RngCore};

/// Strategy for exploring the action space.
pub trait ExplorationStrategy<A: Action> {
    /// Selects an action based on the strategy.
    ///
    /// # Arguments
    /// * `action_values` - A list of (action, q-value) pairs.
    /// * `rng` - A random number generator.
    fn select_action(
        &self,
        action_values: &[(A, f64)],
        rng: &mut dyn RngCore,
    ) -> Option<A>;
}

/// Epsilon-Greedy Exploration Strategy.
///
/// With probability `epsilon`, selects a random action.
/// With probability `1 - epsilon`, selects the action with the highest Q-value.
pub struct EpsilonGreedy {
    epsilon: f64,
}

impl EpsilonGreedy {
    pub fn new(epsilon: f64) -> Self {
        Self { epsilon }
    }
}

impl<A: Action> ExplorationStrategy<A> for EpsilonGreedy {
    fn select_action(
        &self,
        action_values: &[(A, f64)],
        rng: &mut dyn RngCore,
    ) -> Option<A> {
        if action_values.is_empty() {
            return None;
        }

        // Use r#gen because gen is a reserved keyword in 2024 edition
        if rng.r#gen::<f64>() < self.epsilon {
            // Explore: Random action
            let index = rng.gen_range(0..action_values.len());
            Some(action_values[index].0.clone())
        } else {
            // Exploit: Best action
            action_values
                .iter()
                .max_by(|(_, qa), (_, qb)| qa.partial_cmp(qb).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(a, _)| a.clone())
        }
    }
}
