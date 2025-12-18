#[cfg(test)]
mod tests {
    use math_explorer::applied::battery_degradation::{n70, capacity, cycles_to_capacity};
    use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge, Capacity, Cycles};

    #[test]
    fn test_legacy_functions() {
        // Test values from the docstring to verify behavior
        let d = 60.0; // 60% DoD
        let n = n70(d);

        // Assert it's close to 600 as per docs
        assert!((n - 600.0).abs() < 50.0);

        let cap = capacity(n, d);
        assert!((cap - 0.7).abs() < 1e-6);

        let cycles = cycles_to_capacity(0.7, d);
        assert!((cycles - n).abs() < 1e-6);
    }

    #[test]
    fn test_strong_types() {
        let dod = DepthOfDischarge::new(60.0);
        let model = PowerLawModel::standard();

        let cycles = model.n70(dod);
        assert!((cycles.as_f64() - 600.0).abs() < 50.0);

        let cap = model.capacity(cycles, dod);
        assert!((cap.as_f64() - 0.7).abs() < 1e-6);

        let calculated_cycles = model.cycles_to_capacity(Capacity::new(0.7), dod);
        assert!((calculated_cycles.as_f64() - cycles.as_f64()).abs() < 1e-6);
    }

    #[test]
    #[should_panic]
    fn test_dod_bounds_upper() {
        DepthOfDischarge::new(100.1);
    }

    #[test]
    #[should_panic]
    fn test_dod_bounds_lower() {
        DepthOfDischarge::new(-0.1);
    }

    #[test]
    fn test_custom_model() {
        // Create a fictional better battery that lasts twice as long
        let standard = PowerLawModel::standard();
        // alpha * d^beta. Doubling alpha should double cycles if beta stays same
        let improved = PowerLawModel::new(standard.n70(DepthOfDischarge::new(100.0)).as_f64() * 2.0 / 100.0_f64.powf(-1.2639), -1.2639);

        // A simpler test: just make a new model manually
        let improved = PowerLawModel::new(2.0e5, -1.2);
        let dod = DepthOfDischarge::new(50.0);

        let cycles_std = standard.n70(dod).as_f64();
        let cycles_imp = improved.n70(dod).as_f64();

        assert!(cycles_imp != cycles_std);
    }
}
