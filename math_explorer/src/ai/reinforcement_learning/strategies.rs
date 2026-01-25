use super::types::Action;
use rand::Rng;

/// Strategy for selecting actions based on exploration/exploitation trade-off.
pub trait ExplorationStrategy<A: Action> {
    /// Selects an action from the available actions.
    ///
    /// # Arguments
    /// * `action_values` - A function that returns the value (e.g., Q-value) of an action.
    /// * `available_actions` - The list of actions to choose from.
    fn select_action(
        &mut self,
        action_values: impl Fn(&A) -> f64,
        available_actions: &[A],
    ) -> Option<A>;
}

/// Epsilon-Greedy Exploration Strategy.
///
/// With probability `epsilon`, selects a random action (exploration).
/// With probability `1 - epsilon`, selects the action with the highest value (exploitation).
pub struct EpsilonGreedy<R: Rng> {
    epsilon: f64,
    rng: R,
}

impl<R: Rng> EpsilonGreedy<R> {
    pub fn new(epsilon: f64, rng: R) -> Self {
        Self { epsilon, rng }
    }
}

impl<A: Action, R: Rng> ExplorationStrategy<A> for EpsilonGreedy<R> {
    fn select_action(
        &mut self,
        action_values: impl Fn(&A) -> f64,
        available_actions: &[A],
    ) -> Option<A> {
        if available_actions.is_empty() {
            return None;
        }

        if self.rng.r#gen::<f64>() < self.epsilon {
            // Explore: Random action
            let index = self.rng.gen_range(0..available_actions.len());
            Some(available_actions[index].clone())
        } else {
            // Exploit: Best action
            available_actions
                .iter()
                .max_by(|a, b| {
                    let val_a = action_values(a);
                    let val_b = action_values(b);
                    val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned()
        }
    }
}
