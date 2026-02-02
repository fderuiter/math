#![allow(deprecated)]

#[cfg(test)]
mod tests {
    use math_explorer::applied::battery_degradation::types::DepthOfDischarge;
    use math_explorer::applied::battery_degradation::{PowerLawModel, capacity, cycles_to_capacity, n70};

    #[test]
    fn test_legacy_api() {
        // Just ensure they run and return sane values
        let d = 80.0;
        let n = n70(d);
        assert!(n > 0.0);

        // Capacity after n cycles
        // Formula is N70 -> 70%.
        // So after N70 cycles, capacity should be 0.70 * Q_nominal.
        // Q_nominal default is 100.0 Ah (implied, or actually factor is 1.0 relative).
        // Let's check implementation. capacity returns relative capacity factor (0.0 to 1.0).
        let cap = capacity(n, d);
        assert!((cap - 0.70).abs() < 1e-2);

        let cycles = cycles_to_capacity(0.7, d);
        assert!((cycles - n).abs() < 1.0);
    }

    #[test]
    fn test_power_law_model_standard() {
        let model = PowerLawModel::standard();
        let dod = DepthOfDischarge::new(80.0);

        let n = model.n70(dod);
        let cap = model.capacity(n, dod);

        assert!((cap.as_f64() - 0.70).abs() < 1e-2);
    }

    #[test]
    fn test_power_law_model_custom() {
        // Create a model that degrades twice as fast (A is half)
        // Standard A = 13190.0 (implied from legacy)
        // Let's use A = 6000.0
        let _improved = PowerLawModel::new(
            20000.0, // Better battery
            1.483,   // Same exponent
        );
        // We just check it compiles and runs
    }
}
