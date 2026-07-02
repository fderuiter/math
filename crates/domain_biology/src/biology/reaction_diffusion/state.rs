use oxidize_core::grid::Grid2D;
use pure_math::pure_math::analysis::ode::traits::VectorOperations;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents the state of a multi-species chemical system.
///
/// Stores concentrations in a flattened "Structure of Arrays" format using a Grid2D
/// to ensure contiguous memory allocation and zero double-indirection overhead.
/// The layout is `[Species 0 (0..N), Species 1 (0..N), ...]`, where width = grid_size and height = num_species.
#[derive(Debug, Clone, PartialEq)]
pub struct ChemicalState {
    pub grid: Grid2D<f64>,
}

impl ChemicalState {
    /// Creates a new zero-initialized chemical state.
    #[verified_engine::verified]
    pub fn new(num_species: usize, grid_size: usize) -> Self {
        Self {
            grid: Grid2D::new(grid_size, num_species, 0.0),
        }
    }

    /// Returns the number of chemical species.
    #[inline]
    #[verified_engine::verified]
    pub fn num_species(&self) -> usize {
        self.grid.height
    }

    /// Returns the size of the spatial grid.
    #[inline]
    #[verified_engine::verified]
    pub fn grid_size(&self) -> usize {
        self.grid.width
    }

    /// Returns a reference to the concentration slice for a specific species.
    #[inline]
    #[verified_engine::verified]
    pub fn species(&self, index: usize) -> &[f64] {
        let start = self.grid.index_1d(0, index);
        &self.grid.data[start..start + self.grid.width]
    }

    /// Returns a mutable reference to the concentration slice for a specific species.
    #[inline]
    #[verified_engine::verified]
    pub fn species_mut(&mut self, index: usize) -> &mut [f64] {
        let start = self.grid.index_1d(0, index);
        let width = self.grid.width;
        &mut self.grid.data[start..start + width]
    }
}

// Implement standard ops for ODE integration compatibility
impl Add for ChemicalState {
    type Output = Self;

    #[verified_engine::verified]
    fn add(mut self, rhs: Self) -> Self {
        for (val, r_val) in self.grid.data.iter_mut().zip(rhs.grid.data.iter()) {
            *val += r_val;
        }
        self
    }
}

impl AddAssign for ChemicalState {
    #[verified_engine::verified]
    fn add_assign(&mut self, rhs: Self) {
        for (val, r_val) in self.grid.data.iter_mut().zip(rhs.grid.data.iter()) {
            *val += r_val;
        }
    }
}

impl Mul<f64> for ChemicalState {
    type Output = Self;

    #[verified_engine::verified]
    fn mul(mut self, scalar: f64) -> Self {
        for val in self.grid.data.iter_mut() {
            *val *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for ChemicalState {
    #[verified_engine::verified]
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.grid.data.iter_mut() {
            *val *= scalar;
        }
    }
}

impl VectorOperations for ChemicalState {
    #[verified_engine::verified]
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (val, r_val) in self.grid.data.iter_mut().zip(other.grid.data.iter()) {
            *val += r_val * scale;
        }
    }

    #[verified_engine::verified]
    fn copy_from(&mut self, other: &Self) {
        if self.grid.height != other.grid.height || self.grid.width != other.grid.width {
            self.grid = other.grid.clone();
            return;
        }
        self.grid.data.copy_from_slice(&other.grid.data);
    }

    #[verified_engine::verified]
    fn copy_from_scaled(&mut self, source: &Self, other: &Self, scale: f64) {
        if self.grid.height != source.grid.height || self.grid.width != source.grid.width {
            self.grid = Grid2D::new(source.grid.width, source.grid.height, 0.0);
        }

        for ((dst, src), oth) in self
            .grid
            .data
            .iter_mut()
            .zip(source.grid.data.iter())
            .zip(other.grid.data.iter())
        {
            *dst = *src + *oth * scale;
        }
    }
}
