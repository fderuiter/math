/// State container for Lattice Boltzmann Simulation.
///
/// Holds the distribution functions and macroscopic variables.
/// This struct is purely a data holder and does not implement the solver logic.
#[derive(Debug, Clone)]
pub struct LatticeState<const Q: usize> {
    pub width: usize,
    pub height: usize,
    /// Distribution functions (flattened: y * width + x). Each cell holds [f64; Q].
    pub f: Vec<[f64; Q]>,
    /// Buffer for streaming step.
    pub f_new: Vec<[f64; Q]>,
    /// Macroscopic density.
    pub rho: Vec<f64>,
    /// Macroscopic velocity X.
    pub ux: Vec<f64>,
    /// Macroscopic velocity Y.
    pub uy: Vec<f64>,
    /// Boolean grid for obstacles (true = solid).
    pub obstacles: Vec<bool>,
}

impl<const Q: usize> LatticeState<Q> {
    /// Creates a new state with the given dimensions.
    /// Initialized with:
    /// - Density = 1.0
    /// - Velocity = 0.0
    /// - f = 0.0 (Caller must initialize equilibrium)
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.checked_mul(height).expect("Grid dimensions overflow usize");
        Self {
            width,
            height,
            f: vec![[0.0; Q]; size],
            f_new: vec![[0.0; Q]; size],
            rho: vec![1.0; size],
            ux: vec![0.0; size],
            uy: vec![0.0; size],
            obstacles: vec![false; size],
        }
    }

    /// Swaps the current and new distribution function buffers.
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.f, &mut self.f_new);
    }

    /// Returns the linear index for coordinates (x, y).
    /// Does not check bounds (use with caution or check bounds externally).
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Checks if the coordinates are within the grid.
    #[inline]
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
