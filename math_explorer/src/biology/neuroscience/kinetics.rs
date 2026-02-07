//! Ion channel gating kinetics strategies.

use std::fmt::Debug;

/// Defines the voltage-dependent kinetics for an ion channel gating variable.
///
/// Implementations calculate the forward ($\alpha$) and backward ($\beta$) rate constants
/// as a function of membrane potential ($V$).
pub trait GatingKinetics: Debug + Send + Sync {
    /// Calculates the forward rate constant $\alpha$ ($ms^{-1}$).
    ///
    /// # Arguments
    /// * `v` - Membrane potential (mV).
    /// * `v_rest` - Resting potential (mV).
    fn alpha(&self, v: f64, v_rest: f64) -> f64;

    /// Calculates the backward rate constant $\beta$ ($ms^{-1}$).
    ///
    /// # Arguments
    /// * `v` - Membrane potential (mV).
    /// * `v_rest` - Resting potential (mV).
    fn beta(&self, v: f64, v_rest: f64) -> f64;
}

/// Standard Hodgkin-Huxley Potassium (K+) activation kinetics ($n$).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardPotassiumKinetics;

impl GatingKinetics for StandardPotassiumKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }
}

/// Standard Hodgkin-Huxley Sodium (Na+) activation kinetics ($m$).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumActivationKinetics;

impl GatingKinetics for StandardSodiumActivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }
}

/// Standard Hodgkin-Huxley Sodium (Na+) inactivation kinetics ($h$).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumInactivationKinetics;

impl GatingKinetics for StandardSodiumInactivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Legacy implementations for regression testing
    fn legacy_alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn legacy_beta_n(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    fn legacy_alpha_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn legacy_beta_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    fn legacy_alpha_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn legacy_beta_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }

    #[test]
    fn test_potassium_kinetics_regression() {
        let kinetics = StandardPotassiumKinetics;
        let v_rest = -65.0;
        let test_potentials = vec![-100.0, -65.0, -50.0, 0.0, 50.0];

        for v in test_potentials {
            assert!(
                (kinetics.alpha(v, v_rest) - legacy_alpha_n(v, v_rest)).abs() < 1e-9,
                "Alpha_n mismatch at v={}",
                v
            );
            assert!(
                (kinetics.beta(v, v_rest) - legacy_beta_n(v, v_rest)).abs() < 1e-9,
                "Beta_n mismatch at v={}",
                v
            );
        }

        // Test edge case for singularity (v - v_rest = 10.0 -> x=0)
        let v_singular = v_rest + 10.0;
        assert!(
             (kinetics.alpha(v_singular, v_rest) - 0.1).abs() < 1e-9,
             "Alpha_n singularity check failed"
        );
    }

    #[test]
    fn test_sodium_activation_kinetics_regression() {
        let kinetics = StandardSodiumActivationKinetics;
        let v_rest = -65.0;
        let test_potentials = vec![-100.0, -65.0, -50.0, 0.0, 50.0];

        for v in test_potentials {
            assert!(
                (kinetics.alpha(v, v_rest) - legacy_alpha_m(v, v_rest)).abs() < 1e-9,
                "Alpha_m mismatch at v={}",
                v
            );
            assert!(
                (kinetics.beta(v, v_rest) - legacy_beta_m(v, v_rest)).abs() < 1e-9,
                "Beta_m mismatch at v={}",
                v
            );
        }

        // Test edge case for singularity (v - v_rest = 25.0 -> x=0)
        let v_singular = v_rest + 25.0;
         assert!(
             (kinetics.alpha(v_singular, v_rest) - 1.0).abs() < 1e-9,
             "Alpha_m singularity check failed"
        );
    }

    #[test]
    fn test_sodium_inactivation_kinetics_regression() {
        let kinetics = StandardSodiumInactivationKinetics;
        let v_rest = -65.0;
        let test_potentials = vec![-100.0, -65.0, -50.0, 0.0, 50.0];

        for v in test_potentials {
            assert!(
                (kinetics.alpha(v, v_rest) - legacy_alpha_h(v, v_rest)).abs() < 1e-9,
                "Alpha_h mismatch at v={}",
                v
            );
            assert!(
                (kinetics.beta(v, v_rest) - legacy_beta_h(v, v_rest)).abs() < 1e-9,
                "Beta_h mismatch at v={}",
                v
            );
        }
    }
}
