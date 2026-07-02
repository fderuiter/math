/// Classification of a state in a Markov chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateType {
    /// Transient state (can leave and never return).
    Transient,
    /// Absorbing state (once entered, cannot leave).
    Absorbing,
}
