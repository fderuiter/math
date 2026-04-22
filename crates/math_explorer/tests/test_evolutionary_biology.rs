#![allow(warnings)]
use math_explorer::biology::evolution::HawkDovePopulation;

#[test]
fn test_hawk_dove_update() {
    let pop = HawkDovePopulation::new(2.0, 4.0); // v=2, c=4
    let p_h = 0.8;
    let dt = 0.1;

    let next_p_h = pop.update_frequencies(p_h, dt).unwrap();
    println!("Next p_h: {:.10}", next_p_h);

    // Captured value from manual inspection (conceptually)
    // Payoff:
    // H-H: (2-4)/2 = -1
    // H-D: 2
    // D-H: 0
    // D-D: 1
    // Matrix: [[-1, 2], [0, 1]]
    // p = [0.8, 0.2]
    // fitness = A * p = [-1*0.8 + 2*0.2, 0*0.8 + 1*0.2] = [-0.8 + 0.4, 0.2] = [-0.4, 0.2]
    // avg_fitness = p . fitness = 0.8 * -0.4 + 0.2 * 0.2 = -0.32 + 0.04 = -0.28
    // dp_H = p_H * (f_H - avg_f) = 0.8 * (-0.4 - (-0.28)) = 0.8 * (-0.12) = -0.096
    // new_p_H = 0.8 + (-0.096) * 0.1 = 0.8 - 0.0096 = 0.7904

    assert!(
        (next_p_h - 0.7904).abs() < 1e-6,
        "Expected 0.7904, got {}",
        next_p_h
    );
}
