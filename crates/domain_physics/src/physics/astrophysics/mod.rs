/// The astrophysics module contains implementations of astrophysical formulas.
pub mod galaxies;

// [cite:astrophysics]

use pure_math::theory_verification;

theory_verification!(
    module = "astrophysics",
    epsilon = math_commons::registry::TOLERANCE_FAST,
    constants = {
        DUMMY = 1.0;
    },
    test = {
        assert_relative_eq!(DUMMY, 1.0, epsilon = math_commons::registry::TOLERANCE_FAST);
    }
);
