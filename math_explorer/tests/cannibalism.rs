use math_explorer::cannibalism;

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
fn test_dcdt() {
    let n = 100.0;
    let c = 10.0;
    let k_n = 0.05;
    let mu_c = 0.1;
    let result = cannibalism::dcdt(n, c, k_n, mu_c);
    assert_eq!(result, 4.0);
}
