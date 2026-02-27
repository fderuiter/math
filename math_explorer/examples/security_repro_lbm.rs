use math_explorer::physics::fluid_dynamics::lattice_boltzmann::LatticeBoltzmannD2Q9;

fn main() {
    let mut solver = LatticeBoltzmannD2Q9::new(10, 10, 1.0);
    // Break invariant: width * height (100*10=1000) > vector len (100)
    solver.state.width = 100;

    println!("Attempting step with broken invariant...");
    // This should trigger UB (segfault or garbage read) in stream()
    solver.step();
    println!("Survived step (this means we read garbage memory without crashing, which is UB)");
}
