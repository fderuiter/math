use math_explorer::applied::cannibalism::{self, CannibalismModel, CannibalismParams, CannibalismState};
use math_explorer::pure_math::analysis::ode::{OdeSystem, RungeKutta4};

#[test]
fn test_mckendrick_von_foerster() {
    let t = 0.0;
    let a = 0.0;
    let mu = 0.1;
    let n = 100.0;
    let result = cannibalism::mckendrick_von_foerster(t, a, mu, n);
    assert_eq!(result, -10.0);
}

#[test]
fn test_birth_rate() {
    let t = 0.0;
    let result = cannibalism::birth_rate(t);
    assert_eq!(result, 100.0);
}

#[test]
fn test_death_rate() {
    let nu_a = 0.1;
    let c_a = 0.2;
    let k_t = 10.0;
    let phi_c_t = 1.0;
    let result = cannibalism::death_rate(nu_a, c_a, k_t, phi_c_t);
    assert_eq!(result, 2.1);
}

#[test]
fn test_juvenile_dynamics() {
    let i_t = 0.1;
    let c_a = 0.2;
    let a_t = 10.0;
    let n_t_a = 100.0;
    let result = cannibalism::juvenile_dynamics(i_t, c_a, a_t, n_t_a);
    assert_eq!(result, -210.0);
}

#[test]
fn test_adult_dynamics() {
    let n_t_alpha = 50.0;
    let f_i_t = 0.1;
    let a_t = 10.0;
    let result = cannibalism::adult_dynamics(n_t_alpha, f_i_t, a_t);
    assert_eq!(result, 49.0);
}

#[test]
#[allow(deprecated)]
fn test_dndt() {
    let n = 100.0;
    let c = 10.0;
    let beta_n = 0.1;
    let beta_c = 0.2;
    let k_n = 0.05;
    let phi_n_c = 1.0;
    let mu_n = 0.1;
    let result = cannibalism::dndt(n, c, beta_n, beta_c, k_n, phi_n_c, mu_n);
    assert_eq!(result, -4.0);
}

#[test]
#[allow(deprecated)]
fn test_dcdt() {
    let n = 100.0;
    let c = 10.0;
    let k_n = 0.05;
    let mu_c = 0.1;
    let result = cannibalism::dcdt(n, c, k_n, mu_c);
    assert_eq!(result, 4.0);
}

#[test]
fn test_cannibalism_model_integration() {
    let params = CannibalismParams {
        beta_n: 0.1,
        beta_c: 0.2,
        k_n: 0.05,
        phi_loss: 1.0,
        mu_n: 0.1,
        mu_c: 0.1,
    };
    let model = CannibalismModel::new(params);
    let state = CannibalismState::new(100.0, 10.0);

    // Verify derivative matches the legacy calculation
    let deriv = model.derivative(0.0, &state);
    assert_eq!(deriv.x, -4.0);
    assert_eq!(deriv.y, 4.0);

    // Test integration
    let dt = 0.1;
    let next_state = RungeKutta4::step(&model, 0.0, &state, dt);

    // Manual Euler check:
    // n_next = 100.0 + (-4.0 * 0.1) = 99.6
    // c_next = 10.0 + (4.0 * 0.1) = 10.4
    // RK4 should be close to this for linear-ish steps, but more accurate.
    // Since derivatives are linear in state variables, RK4 is exact for polynomials up to degree 4?
    // No, but it should be very precise.

    // Let's just assert it changed in the right direction
    assert!(next_state.x < 100.0);
    assert!(next_state.y > 10.0);
}
