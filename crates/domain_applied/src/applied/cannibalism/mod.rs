pub mod death_rate;
pub mod juvenile_adult_dynamics;
pub mod mckendrick_von_foerster;
pub mod two_dimensional_ode;

pub use death_rate::*;
pub use juvenile_adult_dynamics::*;
pub use mckendrick_von_foerster::*;
pub use two_dimensional_ode::*;

use pure_math::theory_verification;

theory_verification!(
    module = cannibalism,
    epsilon = 1e-6,
    constants = {
        BETA_N = 1.0;
        BETA_C = 0.5;
        K_N = 0.1;
        PHI = 0.05;
        MU_N = 0.2;
        MU_C = 0.3;
    },
    test = {
        let model = CannibalismModel::new(BETA_N, BETA_C, K_N, PHI, MU_N, MU_C);
        assert_relative_eq!(model.beta_n, BETA_N, epsilon = 1e-6);
        assert_relative_eq!(model.beta_c, BETA_C, epsilon = 1e-6);
        assert_relative_eq!(model.k_n, K_N, epsilon = 1e-6);
        assert_relative_eq!(model.phi_n_c, PHI, epsilon = 1e-6);
        assert_relative_eq!(model.mu_n, MU_N, epsilon = 1e-6);
        assert_relative_eq!(model.mu_c, MU_C, epsilon = 1e-6);
    }
);
// [cite:cannibalism]
