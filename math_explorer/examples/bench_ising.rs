use math_explorer::physics::stat_mech::ising::SpinLattice;
use math_explorer::physics::stat_mech::KB;
use std::time::Instant;

fn main() {
    let width = 100;
    let height = 100;
    let j_coupling = 1.0;
    let h_field = 0.0;
    let temp = 2.3 * j_coupling / KB;

    let iterations = 10_000_000;

    println!("Benchmarking Ising Model Metropolis Steps...");
    println!("Grid: {}x{}", width, height);
    println!("Iterations: {}", iterations);

    // Benchmark 1: Calling metropolis_step in a loop.
    // This measures the impact of Modulo Removal + Inlining, but still pays for RNG initialization per step.
    let mut lattice1 = SpinLattice::new(width, height);
    let start = Instant::now();
    for _ in 0..iterations {
        lattice1.metropolis_step(temp, j_coupling, h_field);
    }
    let duration1 = start.elapsed();
    println!("metropolis_step loop: {:.4}s", duration1.as_secs_f64());
    println!("Throughput: {:.2} steps/sec", iterations as f64 / duration1.as_secs_f64());

    // Benchmark 2: Calling evolve.
    // This measures Modulo Removal + Inlining + RNG Reuse (Hoisting).
    let mut lattice2 = SpinLattice::new(width, height);
    let start = Instant::now();
    lattice2.evolve(iterations, temp, j_coupling, h_field);
    let duration2 = start.elapsed();
    println!("evolve (batched): {:.4}s", duration2.as_secs_f64());
    println!("Throughput: {:.2} steps/sec", iterations as f64 / duration2.as_secs_f64());

    println!("Speedup (evolve vs loop): {:.2}x", duration1.as_secs_f64() / duration2.as_secs_f64());
}
