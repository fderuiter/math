#![doc = include_str!("README.md")]

pub mod formulas;
pub mod metrics;
pub mod rewards;

// [cite:grpo]

use pure_math::theory_verification;

theory_verification!(
    module = "grpo",
    epsilon = math_commons::registry::TOLERANCE_FAST,
    constants = {
        BETA = 0.01;
    },
    test = {
        assert_relative_eq!(BETA, 0.01, epsilon = math_commons::registry::TOLERANCE_FAST);
    }
);
