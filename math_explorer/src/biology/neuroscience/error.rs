use thiserror::Error;

/// Errors related to Hodgkin-Huxley neuron modeling.
#[derive(Error, Debug, PartialEq)]
pub enum HodgkinHuxleyError {
    #[error("Invalid gating variable value: {0} (must be between 0 and 1)")]
    InvalidGatingVariable(f64),
    #[error("Invalid conductance value: {0} (must be non-negative)")]
    InvalidConductance(f64),
}
