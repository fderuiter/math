//! Lattice Boltzmann Method (LBM) for fluid simulation.
//!
//! Implements the D2Q9 model with BGK collision operator.
//! This is a mesoscopic method that simulates fluid dynamics by tracking
//! distribution functions on a discrete lattice.

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
    pub width: usize,
    pub height: usize,
    /// Distribution functions (flattened: y * width + x). Each cell holds [f64; 9].
    f: Vec<[f64; Q]>,
    /// Buffer for streaming step.
    f_new: Vec<[f64; Q]>,
    /// Macroscopic density.
    rho: Vec<f64>,
    /// Macroscopic velocity X.
    ux: Vec<f64>,
    /// Macroscopic velocity Y.
    uy: Vec<f64>,
    /// Boolean grid for obstacles (true = solid).
    obstacles: Vec<bool>,
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
        let size = width * height;
        let mut solver = Self {
            width,
            height,
            f: vec![[0.0; Q]; size],
            f_new: vec![[0.0; Q]; size],
            rho: vec![1.0; size],
            ux: vec![0.0; size],
            uy: vec![0.0; size],
            obstacles: vec![false; size],
            collision_model,
            _marker: PhantomData,
        };
        solver.init_equilibrium();
        solver
    }

    /// Initializes the grid to a uniform equilibrium state (rho=1, u=0).
    pub fn init_equilibrium(&mut self) {
        for i in 0..self.width * self.height {
            self.rho[i] = 1.0;
            self.ux[i] = 0.0;
            self.uy[i] = 0.0;
            let eq = L::equilibrium(1.0, 0.0, 0.0);
            self.f[i] = eq;
            self.f_new[i] = eq;
        }
    }

    /// Sets a rectangular region of velocity (e.g., inlet).
    pub fn set_inlet(&mut self, x: usize, y: usize, w: usize, h: usize, u_x: f64, u_y: f64) {
        for j in y..(y + h).min(self.height) {
            for i in x..(x + w).min(self.width) {
                let idx = j * self.width + i;
                if !self.obstacles[idx] {
                    self.ux[idx] = u_x;
                    self.uy[idx] = u_y;
                    // Reset distributions to equilibrium for the new velocity
                    // keeping density roughly constant (1.0)
                    self.f[idx] = L::equilibrium(1.0, u_x, u_y);
                }
            }
        }
    }

    /// Sets an obstacle at (x, y).
    pub fn set_obstacle(&mut self, x: usize, y: usize, is_obstacle: bool) {
        if x < self.width && y < self.height {
            self.obstacles[y * self.width + x] = is_obstacle;
            // Reset velocity inside obstacle
            if is_obstacle {
                let idx = y * self.width + x;
                self.ux[idx] = 0.0;
                self.uy[idx] = 0.0;
            }
        }
    }

    /// Performs one simulation step (Stream -> Collision).
    pub fn step(&mut self) {
        self.stream();
        // Swap buffers
        std::mem::swap(&mut self.f, &mut self.f_new);
        self.boundary_conditions(); // Apply macroscopic BCs if any (like continuous inlet)
        self.macroscopic();
        self.collision();
    }

    /// Streaming step: Move particles to neighboring cells.
    fn stream(&mut self) {
        let cx = L::directions_x();
        let cy = L::directions_y();
        let opp = L::opposite_indices();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                if self.obstacles[idx] {
                    // Obstacles don't stream out, handled by bounce-back in collision/macro
                    continue;
                }

                // Simplified streaming: Pull from neighbors
                for k in 0..Q {
                    // We want to find who streams INTO direction k at (x, y).
                    // This particle came from (x - cx[k], y - cy[k]) moving in direction k.
                    let prev_x = x as i32 - cx[k];
                    let prev_y = y as i32 - cy[k];

                    if prev_x >= 0
                        && prev_x < self.width as i32
                        && prev_y >= 0
                        && prev_y < self.height as i32
                    {
                        let prev_idx = (prev_y as usize) * self.width + (prev_x as usize);

                        if self.obstacles[prev_idx] {
                            // Bounce-back scheme:
                            // If the source was an obstacle, it means a particle going in OPPOSITE[k]
                            // hit the obstacle and came back as k.
                            // So we take f[idx][OPPOSITE[k]] (the one that was leaving us towards the obstacle)
                            self.f_new[idx][k] = self.f[idx][opp[k]];
                        } else {
                            self.f_new[idx][k] = self.f[prev_idx][k];
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
                            src_x += self.width as i32;
                        } else if src_x >= self.width as i32 {
                            src_x -= self.width as i32;
                        }

                        // Bounce Y (Walls)
                        if src_y < 0 || src_y >= self.height as i32 {
                            // Wall bounce: reflecting the particle that tried to leave
                            self.f_new[idx][k] = self.f[idx][opp[k]];
                        } else {
                            let src_idx = (src_y as usize) * self.width + (src_x as usize);
                            if self.obstacles[src_idx] {
                                self.f_new[idx][k] = self.f[idx][opp[k]];
                            } else {
                                self.f_new[idx][k] = self.f[src_idx][k];
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

        for i in 0..self.width * self.height {
            if self.obstacles[i] {
                self.ux[i] = 0.0;
                self.uy[i] = 0.0;
                continue;
            }

            let mut rho = 0.0;
            let mut jx = 0.0;
            let mut jy = 0.0;

            for k in 0..Q {
                rho += self.f[i][k];
                jx += self.f[i][k] * cx[k] as f64;
                jy += self.f[i][k] * cy[k] as f64;
            }

            self.rho[i] = rho;
            if rho > 0.0 {
                self.ux[i] = jx / rho;
                self.uy[i] = jy / rho;
            }
        }
    }

    /// Collision step: Delegates to strategy.
    fn collision(&mut self) {
        for i in 0..self.width * self.height {
            if self.obstacles[i] {
                continue;
            }
            self.collision_model
                .apply(&mut self.f[i], self.rho[i], self.ux[i], self.uy[i]);
        }
    }

    /// Enforce specific boundary conditions (like driving velocity).
    fn boundary_conditions(&mut self) {
        // Example: Drive flow at the inlet (Left side)
        // Just setting macroscopic velocity isn't enough, we need to reset f to eq
        // But let's leave this flexible. The user can call set_inlet manually.
        // For a demo, let's enforce a constant flow at x=0
        /*
        let u_inlet = 0.1;
        for y in 0..self.height {
            let idx = y * self.width;
             if !self.obstacles[idx] {
                 self.ux[idx] = u_inlet;
                 self.uy[idx] = 0.0;
                 self.rho[idx] = 1.0;
                 self.f[idx] = equilibrium(1.0, u_inlet, 0.0);
             }
        }
        */
    }

    // --- Accessors ---

    pub fn get_density(&self, x: usize, y: usize) -> f64 {
        self.rho[y * self.width + x]
    }

    pub fn get_velocity(&self, x: usize, y: usize) -> (f64, f64) {
        let idx = y * self.width + x;
        (self.ux[idx], self.uy[idx])
    }

    pub fn get_velocity_magnitude(&self, x: usize, y: usize) -> f64 {
        let idx = y * self.width + x;
        (self.ux[idx].powi(2) + self.uy[idx].powi(2)).sqrt()
    }

    pub fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.obstacles[y * self.width + x]
    }

    pub fn clear_obstacles(&mut self) {
        for b in &mut self.obstacles {
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

        // Equilibrium for rho=1, u=0
        let eq = D2Q9::equilibrium(rho, ux, uy);

        // Start far from equilibrium
        for k in 0..Q {
            f[k] = eq[k] + 0.1;
        }

        // Apply collision
        <BgkCollision as CollisionModel<9, D2Q9>>::apply(&strategy, &mut f, rho, ux, uy);

        // For tau=1.0, omega=1.0. f should relax to equilibrium instantly.
        // f_new = (1-1)*f + 1*eq = eq
        for k in 0..Q {
            assert!((f[k] - eq[k]).abs() < 1e-9);
        }
    }

    #[test]
    fn test_bgk_relaxation() {
        let tau = 2.0; // omega = 0.5
        let strategy = BgkCollision { tau };
        let mut f = [0.0; Q];
        let rho = 1.0;
        let ux = 0.0;
        let uy = 0.0;

        let eq = D2Q9::equilibrium(rho, ux, uy);

        // Start far from equilibrium
        for k in 0..Q {
            f[k] = eq[k] + 0.2;
        }

        <BgkCollision as CollisionModel<9, D2Q9>>::apply(&strategy, &mut f, rho, ux, uy);

        // f_new = 0.5 * (eq + 0.2) + 0.5 * eq = eq + 0.1
        for k in 0..Q {
            assert!((f[k] - (eq[k] + 0.1)).abs() < 1e-9);
        }
    }

    #[test]
    fn test_solver_initialization() {
        let solver = LatticeBoltzmannD2Q9::new(10, 10, 1.0);
        assert_eq!(solver.rho.len(), 100);
        // Initial state should be equilibrium rho=1, u=0
        for i in 0..100 {
            assert!((solver.rho[i] - 1.0).abs() < 1e-9);
            assert!(solver.ux[i].abs() < 1e-9);
            assert!(solver.uy[i].abs() < 1e-9);
        }
    }
}
