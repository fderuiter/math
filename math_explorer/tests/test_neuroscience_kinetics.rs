use math_explorer::biology::neuroscience::{
    GatingKinetics, HodgkinHuxleyNeuron, HodgkinHuxleyParameters,
};
use std::sync::Arc;

/// A simple constant kinetics strategy for testing.
/// Alpha = 0.5, Beta = 0.5 independent of voltage.
/// This implies steady state x_inf = 0.5 and tau = 1.0 ms.
#[derive(Debug)]
struct ConstantKinetics;

impl GatingKinetics for ConstantKinetics {
    fn alpha(&self, _v: f64, _v_rest: f64) -> f64 {
        0.5
    }

    fn beta(&self, _v: f64, _v_rest: f64) -> f64 {
        0.5
    }
}

#[test]
fn test_standard_kinetics_smoke() {
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
    // Run a few steps
    for _ in 0..10 {
        neuron.update(0.01, 10.0);
    }
    // Just ensure it doesn't panic and values are valid
    assert!(neuron.v().is_finite());
    assert!(neuron.n() >= 0.0 && neuron.n() <= 1.0);
}

#[test]
fn test_custom_kinetics_injection() {
    let mut params = HodgkinHuxleyParameters::default();

    // Inject constant kinetics for Potassium (n)
    params.n_kinetics = Arc::new(ConstantKinetics);

    // Create neuron
    let mut neuron = HodgkinHuxleyNeuron::try_new_with_params(-65.0, params.clone()).unwrap();

    // Force n to 0.0
    neuron.set_n(0.0).unwrap();

    // Evolve. n should approach alpha/(alpha+beta) = 0.5
    // tau = 1/(alpha+beta) = 1.0 ms.
    // We step for 10 ms.
    let dt = 0.1;
    for _ in 0..100 {
        // We use 0.0 current to minimize V changes, though V will drift due to I_K being weird.
        // But n dynamics depend only on V (which affects alpha/beta, but ours are constant)
        // So n should evolve to 0.5 regardless of V.
        neuron.update(dt, 0.0);
    }

    println!("Final n: {}", neuron.n());
    assert!(
        (neuron.n() - 0.5).abs() < 1e-2,
        "n should approach 0.5, got {}",
        neuron.n()
    );
}

#[test]
fn test_parameter_cloning_persistence() {
    // Verify that cloning parameters works and independent neurons don't share mutable state (though they share immutable strategy)
    let params = HodgkinHuxleyParameters::default();
    let mut neuron1 = HodgkinHuxleyNeuron::try_new_with_params(-65.0, params.clone()).unwrap();
    let neuron2 = HodgkinHuxleyNeuron::try_new_with_params(-65.0, params).unwrap();

    neuron1.update(0.1, 100.0); // Drive neuron1 hard

    assert_ne!(neuron1.v(), neuron2.v());
    // n_kinetics is Arc, so they share the strategy.
    // But behavior is stateless in the strategy.
}
