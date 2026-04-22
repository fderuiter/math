#![allow(warnings)]
use math_explorer::epidemiology::compartmental::SIRModel;
use math_explorer::epidemiology::stochastic::GillespieSolver;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

fn main() {
    let steps = 500_000;
    println!("Benchmarking Gillespie Solver with {} steps...", steps);

    // Setup
    let rng = StdRng::seed_from_u64(42);
    let mut solver = GillespieSolver::new(rng);

    // Large population to prevent extinction early
    let n = 1_000_000.0;
    let i0 = 1000.0;
    let model = SIRModel::new(n, i0, 2.0, 0.1).unwrap();
    let mut state = *model.state();

    // Warmup
    for _ in 0..100 {
        solver.step(&model, &mut state).unwrap();
    }

    // Reset state for actual bench
    state = *model.state();
    let rng = StdRng::seed_from_u64(42);
    solver = GillespieSolver::new(rng);

    let start = Instant::now();

    for _ in 0..steps {
        let dt = solver.step(&model, &mut state).unwrap();
        if dt.is_infinite() {
            println!("Simulation ended early (extinction)!");
            break;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:.2?}", duration);
    println!("Time per step: {:.2?}", duration / steps as u32);
}
