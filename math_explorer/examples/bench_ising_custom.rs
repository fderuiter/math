use math_explorer::physics::stat_mech::ising::SpinLattice;
use math_explorer::physics::stat_mech::KB;
use std::time::Instant;

fn main() {
    let width = 200;
    let height = 200;
    let j_coupling = 1.0;
    let h_field = 0.0;
    // Critical temp is ~2.269. We'll use 2.5 to be in the paramagnetic phase (active flipping).
    let temp = 2.5 * j_coupling / KB;

    let mut lattice = SpinLattice::new(width, height);

    // Warmup
    for _ in 0..10_000 {
        lattice.metropolis_step(temp, j_coupling, h_field);
    }

    let steps = 1_000_000;
    println!("Benchmarking {} Metropolis steps on {}x{} lattice...", steps, width, height);

    let start = Instant::now();
    lattice.evolve(steps, temp, j_coupling, h_field);
    let duration = start.elapsed();

    println!("Time: {:.2?}", duration);
    println!("Steps/sec: {:.2} M", steps as f64 / duration.as_secs_f64() / 1e6);
}
