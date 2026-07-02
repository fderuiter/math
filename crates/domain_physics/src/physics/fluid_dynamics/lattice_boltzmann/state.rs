use oxidize_core::grid::Grid2D;
use pure_math::pure_math::analysis::evolution::DoubleBufferedState;

/// State container for Lattice Boltzmann Simulation.
///
/// Holds the distribution functions and macroscopic variables.
/// This struct is purely a data holder and does not implement the solver logic.
#[derive(Debug, Clone)]
pub struct LatticeState<const Q: usize> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    /// Distribution functions (flattened: y * width + x). Each cell holds [f64; Q].
    pub(crate) f: Grid2D<[f64; Q]>,
    /// Buffer for streaming step.
    pub(crate) f_new: Grid2D<[f64; Q]>,
    /// Macroscopic density.
    pub(crate) rho: Grid2D<f64>,
    /// Macroscopic velocity X.
    pub(crate) ux: Grid2D<f64>,
    /// Macroscopic velocity Y.
    pub(crate) uy: Grid2D<f64>,
    /// Boolean grid for obstacles (true = solid).
    pub(crate) obstacles: Grid2D<bool>,
}

impl<const Q: usize> DoubleBufferedState for LatticeState<Q> {
    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.f, &mut self.f_new);
    }
}

impl<const Q: usize> LatticeState<Q> {
    /// Creates a new state with the given dimensions.
    /// Initialized with:
    /// - Density = 1.0
    /// - Velocity = 0.0
    /// - f = 0.0 (Caller must initialize equilibrium)
    #[verified_engine::verified]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            f: Grid2D::new(width, height, [0.0; Q]),
            f_new: Grid2D::new(width, height, [0.0; Q]),
            rho: Grid2D::new(width, height, 1.0),
            ux: Grid2D::new(width, height, 0.0),
            uy: Grid2D::new(width, height, 0.0),
            obstacles: Grid2D::new(width, height, false),
        }
    }

    /// Returns the linear index for coordinates (x, y).
    /// Does not check bounds (use with caution or check bounds externally).
    #[inline]
    #[verified_engine::verified]
    pub fn index(&self, x: usize, y: usize) -> usize {
        self.f.index_1d(x, y)
    }

    /// Checks if the coordinates are within the grid.
    #[inline]
    #[verified_engine::verified]
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // --- Public Accessors ---

    /// Returns the width of the lattice grid.
    #[verified_engine::verified]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the lattice grid.
    #[verified_engine::verified]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a slice of the macroscopic density field.
    #[verified_engine::verified]
    pub fn density(&self) -> &[f64] {
        &self.rho.data
    }

    /// Returns a slice of the macroscopic X-velocity field.
    #[verified_engine::verified]
    pub fn velocity_x(&self) -> &[f64] {
        &self.ux.data
    }

    /// Returns a slice of the macroscopic Y-velocity field.
    #[verified_engine::verified]
    pub fn velocity_y(&self) -> &[f64] {
        &self.uy.data
    }

    /// Returns a slice of the obstacle mask (true = obstacle).
    #[verified_engine::verified]
    pub fn obstacles(&self) -> &[bool] {
        &self.obstacles.data
    }
}
