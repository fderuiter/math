use math_explorer::applied::cannibalism::{CannibalismModel, CannibalismParams};
use math_explorer::pure_math::analysis::ode::OdeSystem;
use nalgebra::Vector2;
use approx::assert_relative_eq;

#[test]
fn test_cannibalism_model_derivative() {
    let params = CannibalismParams {
        beta_n: 0.1,
        beta_c: 0.05,
        k_n: 0.02,
        alpha: 0.01,
        mu_n: 0.01,
        mu_c: 0.1,
    };
    let model = CannibalismModel::new(params);

    // Test Case 1: No Cannibals, some Normals
    // dN/dt = beta_n * N - k_n * N - mu_n * N = (0.1 - 0.02 - 0.01) * 100 = 0.07 * 100 = 7.0
    // dC/dt = k_n * N - mu_c * 0 = 0.02 * 100 = 2.0
    let state = Vector2::new(100.0, 0.0);
    let deriv = model.derivative(0.0, &state);

    assert_relative_eq!(deriv.x, 7.0, epsilon = 1e-10);
    assert_relative_eq!(deriv.y, 2.0, epsilon = 1e-10);
}

#[test]
fn test_cannibalism_integration_step() {
    let params = CannibalismParams::default();
    let model = CannibalismModel::new(params);
    let initial_state = Vector2::new(100.0, 10.0);

    let next_state = model.step(&initial_state, 0.1);

    // Ensure state changes but doesn't explode
    assert!(next_state.x.is_finite());
    assert!(next_state.y.is_finite());
    assert_ne!(initial_state, next_state);
}

#[test]
#[allow(deprecated)]
fn test_legacy_functions_compatibility() {
    // Verify that the logic in the new model matches the legacy functions if we assume phi_n_c matches the model's calculation
    use math_explorer::applied::cannibalism::{dndt, dcdt};

    let params = CannibalismParams::default();
    let n = 100.0;
    let c = 50.0;

    let phi_n_c = params.alpha * n * c;

    let legacy_dndt = dndt(n, c, params.beta_n, params.beta_c, params.k_n, phi_n_c, params.mu_n);
    let legacy_dcdt = dcdt(n, c, params.k_n, params.mu_c);

    let model = CannibalismModel::new(params);
    let state = Vector2::new(n, c);
    let deriv = model.derivative(0.0, &state);

    assert_relative_eq!(deriv.x, legacy_dndt, epsilon = 1e-10);
    assert_relative_eq!(deriv.y, legacy_dcdt, epsilon = 1e-10);
}
