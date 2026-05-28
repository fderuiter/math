#![doc = include_str!("README.md")]

pub mod ensemble;
pub mod error;
pub mod strategies;
pub mod types;

pub use ensemble::LoraEnsemble;
pub use error::LoraError;
pub use types::LoraStateDict;

// [cite:lorahub]

use crate::theory_verification;

theory_verification!(
    module = "lorahub",
    paper = "lorahub.tex",
    epsilon = 1e-6,
    constants = {
        ALPHA = 32.0;
    },
    test = {
        assert_relative_eq!(ALPHA, 32.0, epsilon = 1e-6);
    }
);
