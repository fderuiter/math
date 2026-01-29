//! # Turing Pattern Simulation Example
//!
//! This example simulates a 1D Reaction-Diffusion system using Schnakenberg kinetics.
//! It demonstrates how to set up the system, perturb the initial state, and evolve it over time.
//!
//! Run with: `cargo run --example turing_patterns`

use math_explorer::biology::morphogenesis::{TuringSystem, SchnakenbergKinetics};

fn main() {
    println!("🧪 Initializing Turing Pattern Simulation...");

    // 1. Configuration
    // Domain size: 60 points
    // Diffusion coefficients: D_u = 1.0, D_v = 40.0 (Ratio 40)
    let size = 60;
    let d_u = 1.0;
    let d_v = 40.0;
    let dx = 1.0;

    // Kinetics parameters
    let a = 0.1;
    let b = 0.9;
    let kinetics = SchnakenbergKinetics::new(a, b);

    // Calculate Homogeneous Steady State
    // u* = a + b
    // v* = b / (a + b)^2
    let u_star = a + b;
    let v_star = b / (u_star * u_star);

    println!("   Steady State -> u*: {:.2}, v*: {:.2}", u_star, v_star);

    let mut system = TuringSystem::<SchnakenbergKinetics>::new_with_kinetics(size, d_u, d_v, dx, kinetics);

    // 2. Initialize to Steady State + Perturbation
    for i in 0..size {
        system.u_mut()[i] = u_star;
        system.v_mut()[i] = v_star;
    }

    // Perturb the center
    let center = size / 2;
    // Add random-ish noise (deterministic for this example)
    for i in 0..size {
        let noise = ((i as f64 * 0.1).sin()) * 0.01;
        system.u_mut()[i] += noise;
        system.v_mut()[i] += noise;
    }
    system.u_mut()[center] += 0.1; // Strong perturbation

    println!("   Perturbation applied.");

    // 3. Simulation Loop
    let dt = 0.01;
    let steps = 10000;

    println!("   Simulating for {} steps (dt={})...", steps, dt);

    for _ in 0..steps {
        system.step(dt);
    }

    // 4. Output Analysis
    println!("✅ Simulation Complete.");

    // Simple ASCII visualization of the Activator (U)
    println!("\n   Activator (U) Profile:");
    print_ascii_profile(system.u());
}

fn print_ascii_profile(data: &[f64]) {
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);

    println!("   Min: {:.4}, Max: {:.4}", min_val, max_val);

    if (max_val - min_val).abs() < 1e-6 {
        println!("   [Flat Profile]");
        return;
    }

    let range = max_val - min_val;
    print!("   |");
    for &val in data {
        // Map value to 0..=9
        let normalized = ((val - min_val) / range * 9.0).round() as usize;
        let char = match normalized {
            0 => ' ',
            1 => '.',
            2 => ':',
            3 => '-',
            4 => '=',
            5 => '+',
            6 => '*',
            7 => '#',
            8 => '%',
            _ => '@',
        };
        print!("{}", char);
    }
    println!("|");
}
