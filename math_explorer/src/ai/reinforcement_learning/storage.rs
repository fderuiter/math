use super::types::{Action, State};
use std::collections::HashMap;
use std::hash::Hash;

/// Abstract interface for storing Q-values.
///
/// This trait decouples the Q-Learning algorithm from the underlying storage mechanism.
/// Implementations can use HashMaps, Arrays, or even Function Approximators (with some adaptation).
pub trait QFunction<S: State, A: Action> {
    /// Retrieves the Q-value for a given state-action pair.
    fn get_value(&self, state: &S, action: &A) -> f64;

    /// Updates or sets the Q-value for a given state-action pair.
    fn set_value(&mut self, state: S, action: A, value: f64);
}

/// A standard tabular Q-function implemented using a HashMap.
///
/// Requires states and actions to implement `Hash`, `Eq`, and `Copy`.
pub struct HashMapQFunction<S, A>
where
    S: State + Hash + Eq + Copy,
    A: Action + Hash + Eq + Copy,
{
    table: HashMap<(S, A), f64>,
}

impl<S, A> HashMapQFunction<S, A>
where
    S: State + Hash + Eq + Copy,
    A: Action + Hash + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<S, A> Default for HashMapQFunction<S, A>
where
    S: State + Hash + Eq + Copy,
    A: Action + Hash + Eq + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, A> QFunction<S, A> for HashMapQFunction<S, A>
where
    S: State + Hash + Eq + Copy,
    A: Action + Hash + Eq + Copy,
{
    fn get_value(&self, state: &S, action: &A) -> f64 {
        *self.table.get(&(*state, *action)).unwrap_or(&0.0)
    }

    fn set_value(&mut self, state: S, action: A, value: f64) {
        self.table.insert((state, action), value);
    }
}
