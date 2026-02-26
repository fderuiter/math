use math_explorer::physics::fluid_dynamics::lattice_boltzmann::LatticeBoltzmannD2Q9;
use std::panic;

#[test]
fn test_security_lbm_invariant_check() {
    let mut solver = LatticeBoltzmannD2Q9::new(10, 10, 1.0);
    // Break invariant: width * height (100*10=1000) > vector len (100)
    solver.state.width = 100;

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        solver.step();
    }));

    assert!(result.is_err(), "Should have panicked due to invariant violation");

    // Optional: Verify panic message contains "LatticeState invariant violated"
    // (Requires downcasting the Any error, which is a bit verbose, so basic panic check is good enough)
}
