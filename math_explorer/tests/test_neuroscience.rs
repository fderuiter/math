use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;

#[test]
fn test_neuron_update() {
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
    let dt = 0.01;
    let i_ext = 10.0;

    // Simulate a few steps
    for _ in 0..100 {
        neuron.update(dt, i_ext);
    }

    // Check if potential has changed (simple check)
    assert_ne!(neuron.v, -65.0);
    // Check if variables are within reasonable bounds
    assert!(neuron.n >= 0.0 && neuron.n <= 1.0);
    assert!(neuron.m >= 0.0 && neuron.m <= 1.0);
    assert!(neuron.h >= 0.0 && neuron.h <= 1.0);
}
