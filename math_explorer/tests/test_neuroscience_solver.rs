use math_explorer::biology::neuroscience::neuron::HodgkinHuxleyNeuron;
use math_explorer::pure_math::analysis::ode::RungeKutta4;

#[test]
fn test_neuron_rk4() {
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);

    // Step with RK4
    neuron.update_with(0.01, 10.0, &RungeKutta4);

    // Check if values changed (simple smoke test)
    assert_ne!(neuron.v(), -65.0);
}
