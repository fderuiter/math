//! Lattice Boltzmann Method (LBM) for fluid simulation.
//!
//! Implements the D2Q9 model with BGK collision operator.
//! This is a mesoscopic method that simulates fluid dynamics by tracking
//! distribution functions on a discrete lattice.

pub mod state;

use self::state::LatticeState;
use std::marker::PhantomData;

/// Trait defining the lattice geometry and weights.
///
/// This uses the **Strategy Pattern** to decouple the solver logic from the
/// specific lattice arrangement (D2Q9, D2Q5, etc.).
pub trait Lattice2D<const Q: usize>: Copy + Clone + Send + Sync + 'static {
    fn weights() -> [f64; Q];
    fn directions_x() -> [i32; Q];
    fn directions_y() -> [i32; Q];
    fn opposite_indices() -> [usize; Q];
    fn equilibrium(rho: f64, ux: f64, uy: f64) -> [f64; Q];
}

/// D2Q9 Lattice Model.
///
/// Standard 9-velocity lattice for 2D fluid simulation.
#[derive(Debug, Clone, Copy)]
pub struct D2Q9;

impl Lattice2D<9> for D2Q9 {
    #[inline(always)]
    fn weights() -> [f64; 9] {
        [
            4.0 / 9.0,
            1.0 / 9.0,
            1.0 / 9.0,
            1.0 / 9.0,
            1.0 / 9.0,
            1.0 / 36.0,
            1.0 / 36.0,
            1.0 / 36.0,
            1.0 / 36.0,
        ]
    }

    #[inline(always)]
    fn directions_x() -> [i32; 9] {
        [0, 1, 0, -1, 0, 1, -1, -1, 1]
    }

    #[inline(always)]
    fn directions_y() -> [i32; 9] {
        [0, 0, 1, 0, -1, 1, 1, -1, -1]
    }

    #[inline(always)]
    fn opposite_indices() -> [usize; 9] {
        [0, 3, 4, 1, 2, 7, 8, 5, 6]
    }

    #[inline(always)]
    fn equilibrium(rho: f64, ux: f64, uy: f64) -> [f64; 9] {
        let mut eq = [0.0; 9];
        let u2 = ux * ux + uy * uy;
        let cx = Self::directions_x();
        let cy = Self::directions_y();
        let w = Self::weights();

        for k in 0..9 {
            let cu = (cx[k] as f64 * ux) + (cy[k] as f64 * uy);
            eq[k] = rho * w[k] * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2);
        }
        eq
    }
}

/// Trait defining the collision operator strategy.
pub trait CollisionModel<const Q: usize, L: Lattice2D<Q>> {
    /// Applies the collision operator to the distribution function `f`.
    fn apply(&self, f: &mut [f64; Q], rho: f64, ux: f64, uy: f64);
}

/// BGK Collision Model.
#[derive(Debug, Clone, Copy)]
pub struct BgkCollision {
    /// Relaxation time.
    pub tau: f64,
}

impl<const Q: usize, L: Lattice2D<Q>> CollisionModel<Q, L> for BgkCollision {
    #[inline(always)]
    fn apply(&self, f: &mut [f64; Q], rho: f64, ux: f64, uy: f64) {
        let omega = 1.0 / self.tau;
        let eq = L::equilibrium(rho, ux, uy);
        for k in 0..Q {
            f[k] = (1.0 - omega) * f[k] + omega * eq[k];
        }
    }
}

/// Generic Lattice Boltzmann Solver.
///
/// Implements the discrete streaming and collision steps to simulate fluid dynamics.
///
/// # Examples
///
/// ```rust
/// use math_explorer::physics::fluid_dynamics::lattice_boltzmann::{LatticeBoltzmannD2Q9, BgkCollision};
///
/// // Create a 20x10 lattice with a relaxation time (tau) of 1.0.
/// // Tau determines the kinematic viscosity: nu = (tau - 0.5) / 3.
/// let mut solver = LatticeBoltzmannD2Q9::new(20, 10, 1.0);
///
/// // Set an obstacle at coordinates (10, 5)
/// solver.set_obstacle(10, 5, true);
///
/// // Set a continuous inlet velocity on the left edge (x=0)
/// solver.set_inlet(0, 4, 1, 2, 0.1, 0.0);
///
/// // Perform a single simulation step
/// solver.step();
///
/// // Velocity inside an obstacle is strictly enforced to 0.0
/// let (ux, uy) = solver.get_velocity(10, 5);
/// assert_eq!(ux, 0.0);
/// assert_eq!(uy, 0.0);
/// ```
pub struct LatticeBoltzmann<const Q: usize, L: Lattice2D<Q>, C: CollisionModel<Q, L>> {
    /// The simulation state (grids, obstacles).
    pub state: LatticeState<Q>,
    /// Collision Strategy
    pub collision_model: C,
    _marker: PhantomData<L>,
}

/// Type Alias for Backward Compatibility.
pub type LatticeBoltzmannD2Q9<C> = LatticeBoltzmann<9, D2Q9, C>;

impl LatticeBoltzmannD2Q9<BgkCollision> {
    /// Creates a new solver with the given dimensions and relaxation time.
    ///
    /// * `tau`: Relaxation time. Must be > 0.5 for stability.
    pub fn new(width: usize, height: usize, tau: f64) -> Self {
        Self::new_with_model(width, height, BgkCollision { tau: tau.max(0.51) })
    }
}

impl<const Q: usize, L: Lattice2D<Q>, C: CollisionModel<Q, L>> LatticeBoltzmann<Q, L, C> {
    /// Creates a new solver with a specific collision model.
    pub fn new_with_model(width: usize, height: usize, collision_model: C) -> Self {
        // Validation: Ensure lattice directions are within [-1, 1] range supported by stream()
        for &dx in &L::directions_x() {
            assert!(dx.abs() <= 1, "Lattice directions must be within [-1, 1]");
        }
        for &dy in &L::directions_y() {
            assert!(dy.abs() <= 1, "Lattice directions must be within [-1, 1]");
        }

        let mut solver = Self {
            state: LatticeState::new(width, height),
            collision_model,
            _marker: PhantomData,
        };
        solver.init_equilibrium();
        solver
    }

    /// Initializes the grid to a uniform equilibrium state (rho=1, u=0).
    pub fn init_equilibrium(&mut self) {
        for i in 0..self.state.width * self.state.height {
            self.state.rho[i] = 1.0;
            self.state.ux[i] = 0.0;
            self.state.uy[i] = 0.0;
            let eq = L::equilibrium(1.0, 0.0, 0.0);
            self.state.f[i] = eq;
            self.state.f_new[i] = eq;
        }
    }

    /// Sets a rectangular region of velocity (e.g., inlet).
    pub fn set_inlet(&mut self, x: usize, y: usize, w: usize, h: usize, u_x: f64, u_y: f64) {
        for j in y..(y + h).min(self.state.height) {
            for i in x..(x + w).min(self.state.width) {
                let idx = self.state.index(i, j);
                if !self.state.obstacles[idx] {
                    self.state.ux[idx] = u_x;
                    self.state.uy[idx] = u_y;
                    // Reset distributions to equilibrium for the new velocity
                    // keeping density roughly constant (1.0)
                    self.state.f[idx] = L::equilibrium(1.0, u_x, u_y);
                }
            }
        }
    }

    /// Sets an obstacle at (x, y).
    pub fn set_obstacle(&mut self, x: usize, y: usize, is_obstacle: bool) {
        if x < self.state.width && y < self.state.height {
            let idx = self.state.index(x, y);
            self.state.obstacles[idx] = is_obstacle;
            // Reset velocity inside obstacle
            if is_obstacle {
                self.state.ux[idx] = 0.0;
                self.state.uy[idx] = 0.0;
            }
        }
    }

    /// Performs one simulation step (Stream -> Collision).
    pub fn step(&mut self) {
        self.stream();
        self.state.swap_buffers();
        self.boundary_conditions(); // Apply macroscopic BCs if any
        self.macroscopic_and_collision();
    }

    /// Fused Macroscopic + Collision step.
    ///
    /// # Performance
    /// This optimization fuses the macroscopic moment calculation and the collision step
    /// into a single loop. This reduces memory bandwidth pressure by:
    /// 1. Reading `f` only once (instead of twice).
    /// 2. Keeping `rho`, `ux`, `uy` in CPU registers for the collision step, avoiding
    ///    store-then-load latency for these variables.
    fn macroscopic_and_collision(&mut self) {
        let cx = L::directions_x();
        let cy = L::directions_y();

        let collision_model = &self.collision_model;

        // Use zipping for iteration to enable better vectorization and eliminate bounds checks
        self.state
            .f
            .iter_mut()
            .zip(self.state.rho.iter_mut())
            .zip(self.state.ux.iter_mut())
            .zip(self.state.uy.iter_mut())
            .zip(self.state.obstacles.iter())
            .for_each(|((((f_cell, rho), ux), uy), is_obs)| {
                if *is_obs {
                    *ux = 0.0;
                    *uy = 0.0;
                    return;
                }

                let mut local_rho = 0.0;
                let mut local_jx = 0.0;
                let mut local_jy = 0.0;

                for k in 0..Q {
                    let val = f_cell[k];
                    local_rho += val;
                    local_jx += val * cx[k] as f64;
                    local_jy += val * cy[k] as f64;
                }

                *rho = local_rho;

                let (local_ux, local_uy) = if local_rho > 0.0 {
                    (local_jx / local_rho, local_jy / local_rho)
                } else {
                    (0.0, 0.0)
                };

                *ux = local_ux;
                *uy = local_uy;

                collision_model.apply(f_cell, local_rho, local_ux, local_uy);
            });
    }

    /// Streaming step: Move particles to neighboring cells.
    fn stream(&mut self) {
        let width = self.state.width;
        let height = self.state.height;
        let required_len = width * height;

        // Security Check: Ensure invariants hold before entering unsafe blocks.
        // Since `state` fields are public, a user could corrupt them (e.g., changing width without resizing f).
        assert_eq!(
            self.state.f.len(),
            required_len,
            "LatticeState invariant violated: f.len() != width * height"
        );
        assert_eq!(
            self.state.f_new.len(),
            required_len,
            "LatticeState invariant violated: f_new.len() != width * height"
        );
        assert_eq!(
            self.state.obstacles.len(),
            required_len,
            "LatticeState invariant violated: obstacles.len() != width * height"
        );

        let cx = L::directions_x();
        let cy = L::directions_y();
        let opp = L::opposite_indices();

        // Precompute offsets to avoid multiplication in the inner loop.
        // offset[k] = -(dx[k] + dy[k] * width)
        let mut offsets = [0isize; Q];
        for k in 0..Q {
            offsets[k] = -((cx[k] as isize) + (cy[k] as isize) * (width as isize));
        }

        // 1. Interior (Hot Path) - Loop Splitting & Unsafe optimizations
        // Avoid bounds checks for x, y, and array access in the bulk of the simulation.
        if width > 2 && height > 2 {
            for y in 1..height - 1 {
                let idx_row = y * width;
                for x in 1..width - 1 {
                    let idx = idx_row + x;

                    // SAFETY: We are within 1..W-1 and 1..H-1, so idx is valid.
                    let is_obstacle = unsafe { *self.state.obstacles.get_unchecked(idx) };
                    if is_obstacle {
                        continue;
                    }

                    for k in 0..Q {
                        // Calculate prev_idx using precomputed offset.
                        // Since we are in the interior (y >= 1, x >= 1), idx is at least width + 1.
                        // The max negative offset is -(1 + width).
                        // So idx + offset >= 0.
                        let prev_idx = (idx as isize + offsets[k]) as usize;

                        // SAFETY: prev_idx is valid. k and opp[k] are < Q.
                        unsafe {
                            let source_is_obstacle = *self.state.obstacles.get_unchecked(prev_idx);

                            if source_is_obstacle {
                                // Bounce-back
                                let bounce_val =
                                    *self.state.f.get_unchecked(idx).get_unchecked(opp[k]);
                                *self.state.f_new.get_unchecked_mut(idx).get_unchecked_mut(k) =
                                    bounce_val;
                            } else {
                                // Stream
                                let stream_val =
                                    *self.state.f.get_unchecked(prev_idx).get_unchecked(k);
                                *self.state.f_new.get_unchecked_mut(idx).get_unchecked_mut(k) =
                                    stream_val;
                            }
                        }
                    }
                }
            }
        }

        // 2. Boundary Handling (Slow Path)
        // Process top/bottom rows and left/right columns
        let process_boundary_cell = |x: usize, y: usize, state: &mut LatticeState<Q>| {
            let idx = y * width + x;
            if state.obstacles[idx] {
                return;
            }

            for k in 0..Q {
                let prev_x = x as i32 - cx[k];
                let prev_y = y as i32 - cy[k];

                if prev_x >= 0 && prev_x < width as i32 && prev_y >= 0 && prev_y < height as i32 {
                    let prev_idx = (prev_y as usize) * width + (prev_x as usize);
                    if state.obstacles[prev_idx] {
                        state.f_new[idx][k] = state.f[idx][opp[k]];
                    } else {
                        state.f_new[idx][k] = state.f[prev_idx][k];
                    }
                } else {
                    // Boundary Logic (Periodic X, Bounce Y)
                    let mut src_x = prev_x;
                    let src_y = prev_y;

                    // Periodic X
                    if src_x < 0 {
                        src_x += width as i32;
                    } else if src_x >= width as i32 {
                        src_x -= width as i32;
                    }

                    if src_y < 0 || src_y >= height as i32 {
                        // Wall Bounce
                        state.f_new[idx][k] = state.f[idx][opp[k]];
                    } else {
                        let src_idx = (src_y as usize) * width + (src_x as usize);
                        if state.obstacles[src_idx] {
                            state.f_new[idx][k] = state.f[idx][opp[k]];
                        } else {
                            state.f_new[idx][k] = state.f[src_idx][k];
                        }
                    }
                }
            }
        };

        // Top & Bottom
        for y in [0, height - 1] {
            for x in 0..width {
                process_boundary_cell(x, y, &mut self.state);
            }
        }
        // Left & Right (excluding corners)
        for y in 1..height - 1 {
            for x in [0, width - 1] {
                process_boundary_cell(x, y, &mut self.state);
            }
        }
    }

    /// Enforce specific boundary conditions (like driving velocity).
    fn boundary_conditions(&mut self) {
        // ... (Existing code is commented out, preserving it as is)
    }

    // --- Accessors ---

    pub fn width(&self) -> usize {
        self.state.width
    }

    pub fn height(&self) -> usize {
        self.state.height
    }

    pub fn get_density(&self, x: usize, y: usize) -> f64 {
        self.state.rho[self.state.index(x, y)]
    }

    pub fn get_velocity(&self, x: usize, y: usize) -> (f64, f64) {
        let idx = self.state.index(x, y);
        (self.state.ux[idx], self.state.uy[idx])
    }

    pub fn get_velocity_magnitude(&self, x: usize, y: usize) -> f64 {
        let idx = self.state.index(x, y);
        (self.state.ux[idx].powi(2) + self.state.uy[idx].powi(2)).sqrt()
    }

    pub fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.state.obstacles[self.state.index(x, y)]
    }

    pub fn clear_obstacles(&mut self) {
        for b in &mut self.state.obstacles {
            *b = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: usize = 9;

    #[test]
    fn test_bgk_collision() {
        let tau = 1.0;
        let strategy = BgkCollision { tau };
        let mut f = [0.0; Q];
        let rho = 1.0;
        let ux = 0.0;
        let uy = 0.0;

        let eq = D2Q9::equilibrium(rho, ux, uy);
        for k in 0..Q {
            f[k] = eq[k] + 0.1;
        }

        <BgkCollision as CollisionModel<9, D2Q9>>::apply(&strategy, &mut f, rho, ux, uy);

        for k in 0..Q {
            assert!((f[k] - eq[k]).abs() < 1e-9);
        }
    }

    #[test]
    fn test_solver_initialization() {
        let solver = LatticeBoltzmannD2Q9::new(10, 10, 1.0);
        assert_eq!(solver.state.rho.len(), 100);
        for i in 0..100 {
            assert!((solver.state.rho[i] - 1.0).abs() < 1e-9);
            assert!(solver.state.ux[i].abs() < 1e-9);
        }
    }

    #[test]
    fn test_gui_compliance_dynamic_inputs() {
        let width = 20;
        let height = 10;
        let tau = 1.0;

        let mut solver: LatticeBoltzmannD2Q9<BgkCollision> =
            LatticeBoltzmannD2Q9::new(width, height, tau);

        solver.set_inlet(0, 4, 1, 2, 0.1, 0.0);
        solver.step();
        let inlet_u = solver.get_velocity_magnitude(0, 4);
        assert!(inlet_u > 0.0);

        solver.collision_model.tau = 2.0;
        solver.step();
        assert!(solver.state.rho[0].is_finite());

        let obs_x = 10;
        let obs_y = 5;
        solver.set_obstacle(obs_x, obs_y, true);
        solver.step();

        assert!(solver.is_obstacle(obs_x, obs_y));
        let (ux, uy) = solver.get_velocity(obs_x, obs_y);
        assert_eq!(ux, 0.0);
        assert_eq!(uy, 0.0);

        solver.clear_obstacles();
        assert!(!solver.is_obstacle(obs_x, obs_y));

        solver = LatticeBoltzmannD2Q9::new(width, height, 0.6);
        assert!((solver.collision_model.tau - 0.6).abs() < 1e-9);
    }
}
