use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoraHubError {
    EmptyEnsemble,
    EmptyWeights,
    LengthMismatch,
    ShapeMismatch,
    StrategyError(String),
}

impl fmt::Display for LoraHubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEnsemble => write!(f, "Ensemble is empty; cannot combine."),
            Self::EmptyWeights => write!(f, "Weights cannot be empty."),
            Self::LengthMismatch => write!(
                f,
                "The number of weights must match the number of modules in the ensemble."
            ),
            Self::ShapeMismatch => write!(f, "Mismatched tensor shapes for the same key."),
            Self::StrategyError(msg) => write!(f, "Strategy error: {}", msg),
        }
    }
}

impl std::error::Error for LoraHubError {}
