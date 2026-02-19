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

    fn directions_x() -> [i32; 9] {
        [0, 1, 0, -1, 0, 1, -1, -1, 1]
    }

    fn directions_y() -> [i32; 9] {
        [0, 0, 1, 0, -1, 1, 1, -1, -1]
    }

    fn opposite_indices() -> [usize; 9] {
        [0, 3, 4, 1, 2, 7, 8, 5, 6]
    }

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
    fn apply(&self, f: &mut [f64; Q], rho: f64, ux: f64, uy: f64) {
        let omega = 1.0 / self.tau;
        let eq = L::equilibrium(rho, ux, uy);
        for k in 0..Q {
            f[k] = (1.0 - omega) * f[k] + omega * eq[k];
        }
    }
}

/// Generic Lattice Boltzmann Solver.
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
        self.macroscopic();
        self.collision();
    }

    /// Streaming step: Move particles to neighboring cells.
    fn stream(&mut self) {
        let cx = L::directions_x();
        let cy = L::directions_y();
        let opp = L::opposite_indices();

        let width = self.state.width;
        let height = self.state.height;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                if self.state.obstacles[idx] {
                    // Obstacles don't stream out, handled by bounce-back in collision/macro
                    continue;
                }

                // Simplified streaming: Pull from neighbors
                for k in 0..Q {
                    // We want to find who streams INTO direction k at (x, y).
                    // This particle came from (x - cx[k], y - cy[k]) moving in direction k.
                    let prev_x = x as i32 - cx[k];
                    let prev_y = y as i32 - cy[k];

                    if prev_x >= 0 && prev_x < width as i32 && prev_y >= 0 && prev_y < height as i32
                    {
                        let prev_idx = (prev_y as usize) * width + (prev_x as usize);

                        if self.state.obstacles[prev_idx] {
                            // Bounce-back scheme:
                            // If the source was an obstacle, it means a particle going in OPPOSITE[k]
                            // hit the obstacle and came back as k.
                            // So we take f[idx][OPPOSITE[k]] (the one that was leaving us towards the obstacle)
                            self.state.f_new[idx][k] = self.state.f[idx][opp[k]];
                        } else {
                            self.state.f_new[idx][k] = self.state.f[prev_idx][k];
                        }
                    } else {
                        // Boundary handling (Periodic or Bounce)
                        // Implementing simple periodic for now to keep flow continuous
                        // Or equilibrium inlet/outlet. Let's do Bounce-back at domain walls for simplicity,
                        // except maybe periodic X.

                        // Let's implement Periodic X, Bounce Y (Channel Flow)
                        let mut src_x = prev_x;
                        let src_y = prev_y;

                        // Periodic X
                        if src_x < 0 {
                            src_x += width as i32;
                        } else if src_x >= width as i32 {
                            src_x -= width as i32;
                        }

                        // Bounce Y (Walls)
                        if src_y < 0 || src_y >= height as i32 {
                            // Wall bounce: reflecting the particle that tried to leave
                            self.state.f_new[idx][k] = self.state.f[idx][opp[k]];
                        } else {
                            let src_idx = (src_y as usize) * width + (src_x as usize);
                            if self.state.obstacles[src_idx] {
                                self.state.f_new[idx][k] = self.state.f[idx][opp[k]];
                            } else {
                                self.state.f_new[idx][k] = self.state.f[src_idx][k];
                            }
                        }
                    }
                }
            }
        }
    }

    /// Updates macroscopic variables (rho, u) from distribution functions.
    fn macroscopic(&mut self) {
        let cx = L::directions_x();
        let cy = L::directions_y();

        for i in 0..self.state.width * self.state.height {
            if self.state.obstacles[i] {
                self.state.ux[i] = 0.0;
                self.state.uy[i] = 0.0;
                continue;
            }

            let mut rho = 0.0;
            let mut jx = 0.0;
            let mut jy = 0.0;

            for k in 0..Q {
                rho += self.state.f[i][k];
                jx += self.state.f[i][k] * cx[k] as f64;
                jy += self.state.f[i][k] * cy[k] as f64;
            }

            self.state.rho[i] = rho;
            if rho > 0.0 {
                self.state.ux[i] = jx / rho;
                self.state.uy[i] = jy / rho;
            }
        }
    }

    /// Collision step: Delegates to strategy.
    fn collision(&mut self) {
        for i in 0..self.state.width * self.state.height {
            if self.state.obstacles[i] {
                continue;
            }
            self.collision_model.apply(
                &mut self.state.f[i],
                self.state.rho[i],
                self.state.ux[i],
                self.state.uy[i],
            );
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
