use math_explorer::applied::game_theory::mean_field::MeanFieldGame1D;
use std::time::Instant;

fn main() {
    // Large grid to make it slow enough to measure
    // dt must satisfy stability condition: viscosity * dt / dx^2 < 0.5
    // dx = 4.0 / 200 = 0.02. dx^2 = 0.0004.
    // viscosity = 0.1.
    // 0.1 * dt / 0.0004 < 0.5 => dt < 0.002
    // T = 1.0. dt = 1.0 / nt.
    // 1.0 / nt < 0.002 => nt > 500.
    // Using nt=2000 for extra stability margin (dt=0.0005, ratio=0.125).

    let grid_points = 200;
    let time_steps = 2000;
    let iterations = 100;

    println!(
        "Setting up MFG with nx={}, nt={}, iterations={}...",
        grid_points, time_steps, iterations
    );

    let mfg = MeanFieldGame1D::new(0.1, 1.0, grid_points, time_steps, -2.0, 2.0);

    let cost_fn = |x: f64, m: f64| -> f64 { m + x * x };
    let term_fn = |x: f64, _m: f64| -> f64 { x * x };
    let init_dist = |x: f64| -> f64 { (-x * x * 5.0).exp() };

    println!("Starting solve...");
    let start = Instant::now();
    let (u, m) = mfg.solve(cost_fn, term_fn, init_dist, iterations);
    let duration = start.elapsed();

    println!("Solve completed in {:.2?}", duration);
    println!(
        "Final dimensions: u=({},{}), m=({},{})",
        u.nrows(),
        u.ncols(),
        m.nrows(),
        m.ncols()
    );

    // Simple checksum to ensure optimization doesn't change result
    let u_sum: f64 = u.sum();
    let m_sum: f64 = m.sum();
    println!("Checksums: u_sum={:.6}, m_sum={:.6}", u_sum, m_sum);
}
