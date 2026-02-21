use math_explorer::physics::fluid_dynamics::lattice_boltzmann::state::LatticeState;

#[test]
#[should_panic(expected = "Grid dimensions overflow")]
fn test_lbm_state_overflow_protection() {
    // Attempt to create a state with dimensions that overflow usize.
    // On 64-bit systems, usize::BITS is 64. 1 << 32 * 1 << 32 = 0 (mod 2^64).
    // On 32-bit systems, usize::BITS is 32. 1 << 16 * 1 << 16 = 0 (mod 2^32).

    // We use usize::BITS / 2 to find the square root of the max usize value + 1 (roughly).
    let shift = usize::BITS / 2;
    let w = 1usize << shift;
    let h = 1usize << shift;

    // This should panic if overflow protection is in place.
    // Without protection, it wraps to 0, allocates a 0-sized vector, and returns successfully.
    // This creates a vulnerable state where subsequent accesses are out-of-bounds.
    let _ = LatticeState::<9>::new(w, h);
}
