#![doc = include_str!("README.md")]

pub mod ensemble;
pub mod strategies;
pub mod types;

pub use ensemble::LoraEnsemble;
pub use types::LoraStateDict;

// [cite:lorahub]

use pure_math::theory_verification;

theory_verification!(
    module = "lorahub",
    epsilon = math_commons::registry::TOLERANCE_FAST,
    constants = {
        ALPHA = 32.0;
    },
    test = {
        assert_relative_eq!(ALPHA, 32.0, epsilon = math_commons::registry::TOLERANCE_FAST);
    }
);
