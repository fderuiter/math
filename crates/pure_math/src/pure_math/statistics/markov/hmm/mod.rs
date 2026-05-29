pub mod forward_backward;
pub mod generation;
pub mod model;
pub mod viterbi;

#[cfg(test)]
mod tests;

pub use model::HiddenMarkovModel;

// [cite:clinical_trials_statistics]
