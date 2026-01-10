use math_explorer::physics::quantum::clebsch_gordan;

fn main() {
    // Coupling j1=1.5, m1=-0.5 with j2=1.0, m2=1.0 to J=2.5, M=0.5
    let coeff = clebsch_gordan(1.5, -0.5, 1.0, 1.0, 2.5, 0.5);
    println!("Clebsch-Gordan Coefficient: {:.4}", coeff);
}
