use math_explorer::physics::quantum::clebsch_gordan;

fn main() {
    println!("Hello World from Math Explorer!");
    println!("-------------------------------");
    println!("Calculating Clebsch-Gordan Coefficient for Quantum Angular Momentum...");

    // Coupling j1=1.5, m1=-0.5 with j2=1.0, m2=1.0 to J=2.5, M=0.5
    // Note: These are standard quantum mechanics values.
    let j1 = 1.5;
    let m1 = -0.5;
    let j2 = 1.0;
    let m2 = 1.0;
    let j = 2.5;
    let m = 0.5;

    let coeff = clebsch_gordan(j1, m1, j2, m2, j, m);

    println!("Parameters:");
    println!("  j1: {}, m1: {}", j1, m1);
    println!("  j2: {}, m2: {}", j2, m2);
    println!("  J:  {}, M:  {}", j, m);
    println!("Result:");
    println!("  <j1 m1; j2 m2 | J M> = {:.6}", coeff);

    // Check expectation:
    // This specific combination should be non-zero if valid.
    if coeff.abs() > 1e-10 {
        println!("\n✅ Calculation successful.");
    } else {
        println!("\n⚠️  Result is zero (possibly forbidden transition).");
    }
}
