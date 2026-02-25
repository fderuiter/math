use math_explorer::physics::fluid_dynamics::lattice_boltzmann::{
    BgkCollision, LatticeBoltzmannD2Q9,
};
use std::time::Instant;

fn main() {
    let width = 256;
    let height = 256;
    let steps = 500;

    println!(
        "Benchmarking LBM D2Q9 on {}x{} grid for {} steps...",
        width, height, steps
    );

    // Initialize Solver
    // tau = 0.6 is a common value for low viscosity fluids (Re ~ high)
    let mut solver = LatticeBoltzmannD2Q9::new(width, height, 0.6);

    // Set up a standard benchmark case: Lid Driven Cavity (sort of)
    // Moving top wall is often used, but here we just set an inlet to create motion.
    // Let's set a velocity on the top row to drive flow.
    for x in 1..width - 1 {
        solver.set_inlet(x, 1, 1, 1, 0.1, 0.0);
    }

    // Warmup
    for _ in 0..10 {
        solver.step();
    }

    let start = Instant::now();

    for _ in 0..steps {
        solver.step();
    }

    let duration = start.elapsed();

    // Compute checksum (total density) to prevent optimization
    let mut total_rho = 0.0;
    for y in 0..height {
        for x in 0..width {
            total_rho += solver.get_density(x, y);
        }
    }

    println!("Done in {:.4}s", duration.as_secs_f64());
    println!(
        "Average time per step: {:.4}ms",
        duration.as_secs_f64() * 1000.0 / steps as f64
    );
    println!(
        "Throughput: {:.2} MLUPS (Million Lattice Updates Per Second)",
        (width * height * steps) as f64 / duration.as_secs_f64() / 1_000_000.0
    );
    println!("Checksum (Total Rho): {:.4}", total_rho);
}
