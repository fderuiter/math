#[cfg(test)]
mod tests {
    use math_explorer::applied::cannibalism::two_dimensional_ode::CannibalismModel;
    use math_explorer::pure_math::analysis::ode::OdeSystem;
    use nalgebra::Vector2;

    #[allow(deprecated)]
    use math_explorer::applied::cannibalism::two_dimensional_ode::{dcdt, dndt};

    #[test]
    fn test_legacy_functions() {
        let n = 100.0;
        let c = 10.0;
        let beta_n = 0.5;
        let beta_c = 0.2;
        let k_n = 0.1;
        let phi_n_c = 5.0;
        let mu_n = 0.3;
        let mu_c = 0.4;

        #[allow(deprecated)]
        let dn = dndt(n, c, beta_n, beta_c, k_n, phi_n_c, mu_n);
        #[allow(deprecated)]
        let dc = dcdt(n, c, k_n, mu_c);

        assert!((dn - 7.0).abs() < 1e-10);
        assert!((dc - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_struct_implementation_matches_legacy() {
        let n = 100.0;
        let c = 10.0;
        let beta_n = 0.5;
        let beta_c = 0.2;
        let k_n = 0.1;
        let phi_n_c = 5.0;
        let mu_n = 0.3;
        let mu_c = 0.4;

        let model = CannibalismModel::new(beta_n, beta_c, k_n, phi_n_c, mu_n, mu_c);
        let state = Vector2::new(n, c);

        // Time t is unused in this autonomous system
        let derivative = model.derivative(0.0, &state);

        #[allow(deprecated)]
        let legacy_dn = dndt(n, c, beta_n, beta_c, k_n, phi_n_c, mu_n);
        #[allow(deprecated)]
        let legacy_dc = dcdt(n, c, k_n, mu_c);

        assert!(
            (derivative[0] - legacy_dn).abs() < 1e-10,
            "Model derivative dN/dt should match legacy"
        );
        assert!(
            (derivative[1] - legacy_dc).abs() < 1e-10,
            "Model derivative dC/dt should match legacy"
        );
    }
}
