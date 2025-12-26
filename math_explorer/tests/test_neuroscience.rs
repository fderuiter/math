
use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
use approx::assert_relative_eq;

#[test]
fn test_hh_neuron_pulse() {
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
    let dt = 0.01;

    // Inject current for 10ms
    for _ in 0..1000 {
        neuron.update(dt, 10.0);
    }

    // Check if it fired (v should be high positive, then repolarize)
    println!("Final V: {}", neuron.v());
    assert!(neuron.v() > -70.0);
    assert!(neuron.v() < 50.0);
}

#[test]
fn test_hh_neuron_resting() {
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
    let dt = 0.01;

    // No current
    for _ in 0..100 {
        neuron.update(dt, 0.0);
    }

    assert_relative_eq!(neuron.v(), -65.0, epsilon = 1.0);
}

#[test]
fn test_hh_neuron_rk4() {
    use math_explorer::pure_math::analysis::ode::RungeKutta4;
    let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
    let dt = 0.01;

    // Use RK4 solver
    for _ in 0..100 {
        neuron.update_with(dt, 0.0, &RungeKutta4);
    }

    assert_relative_eq!(neuron.v(), -65.0, epsilon = 1.0);
}
