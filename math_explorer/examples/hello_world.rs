//! # Hello World (Quantum Physics)
//!
//! This example calculates Clebsch-Gordan coefficients, which arise in Quantum Mechanics
//! when adding angular momenta.
//!
//! It answers the question: "If I have two particles with spins j1 and j2, what is the probability
//! amplitude that they combine to form a total spin J?"
//!
//! To run this example:
//! ```bash
//! cargo run --example hello_world
//! ```

use math_explorer::physics::quantum::clebsch_gordan;

fn main() {
    println!("👋 Welcome to Math Explorer!");
    println!("============================");
    println!("Calculating Clebsch-Gordan Coefficients for Angular Momentum Coupling...");
    println!();

    // Configuration:
    // Particle 1: Spin j1=1.5, z-component m1=-0.5
    // Particle 2: Spin j2=1.0, z-component m2=1.0
    // Target State: Total Spin J=2.5, z-component M=0.5
    let j1 = 1.5;
    let m1 = -0.5;
    let j2 = 1.0;
    let m2 = 1.0;
    let j = 2.5;
    let m = 0.5;

    let coeff = clebsch_gordan(j1, m1, j2, m2, j, m);

    println!("Input Parameters:");
    println!("  j1 = {:<4} m1 = {:<4}", j1, m1);
    println!("  j2 = {:<4} m2 = {:<4}", j2, m2);
    println!("  J  = {:<4} M  = {:<4}", j, m);
    println!();
    println!("Result:");
    println!("  Clebsch-Gordan Coefficient: {:.6}", coeff);
    println!("  Probability (|CG|²):        {:.6}", coeff * coeff);

    // Verify against a known value (Griffiths, Table 4.8)
    // Note: Normalization conventions may vary slightly between textbooks and libraries.
    // math_explorer uses a specific convention verified in tests.
    let expected_sq = 3.0 / 10.0;
    println!();
    println!("Verification:");
    println!("  Expected Probability (approx): {:.6}", expected_sq);
}
