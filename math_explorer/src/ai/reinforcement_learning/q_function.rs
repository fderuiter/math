use std::collections::HashMap;
use std::hash::Hash;
use super::types::{Action, State};

/// A trait for representing the Q-Function $Q(s, a)$.
/// This allows for different implementations: Tabular (HashMap), Linear Approximation, Neural Network, etc.
pub trait QFunction<S, A> {
    /// Returns the Q-value for a given state-action pair.
    fn get(&self, state: &S, action: &A) -> f64;

    /// Updates the Q-value for a given state-action pair.
    /// In tabular methods, this sets the value directly.
    /// In function approximation, this might trigger a gradient step or buffer update.
    fn update(&mut self, state: &S, action: &A, value: f64);
}

/// A standard tabular implementation using a HashMap.
#[derive(Debug, Clone)]
pub struct HashMapQFunction<S, A>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    table: HashMap<(S, A), f64>,
}

impl<S, A> Default for HashMapQFunction<S, A>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    fn default() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl<S, A> HashMapQFunction<S, A>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S, A> QFunction<S, A> for HashMapQFunction<S, A>
where
    S: State + Hash + Eq,
    A: Action + Hash + Eq,
{
    fn get(&self, state: &S, action: &A) -> f64 {
        // We clone keys because HashMap requires ownership or references, but our trait
        // takes references. The tuple key (S, A) must be constructed.
        // Since S and A are Clone (from State/Action traits), this is fine.
        *self.table.get(&(state.clone(), action.clone())).unwrap_or(&0.0)
    }

    fn update(&mut self, state: &S, action: &A, value: f64) {
        self.table.insert((state.clone(), action.clone()), value);
    }
}
