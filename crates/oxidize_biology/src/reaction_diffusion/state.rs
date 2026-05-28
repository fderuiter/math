use oxidize_pure_math::analysis::ode::traits::VectorOperations;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents the state of a multi-species chemical system.
///
/// Stores concentrations in a flattened "Structure of Arrays" format: `Vec<f64>`
/// to ensure contiguous memory allocation and zero double-indirection overhead.
/// The layout is `[Species 0 (0..N), Species 1 (0..N), ...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChemicalState {
    num_species: usize,
    grid_size: usize,
    /// Flattened concentrations of each species across the spatial grid.
    pub concentrations: Vec<f64>,
}

impl ChemicalState {
    /// Creates a new zero-initialized chemical state.
    pub fn new(num_species: usize, grid_size: usize) -> Self {
        Self {
            num_species,
            grid_size,
            concentrations: vec![0.0; num_species * grid_size],
        }
    }

    /// Returns the number of chemical species.
    #[inline]
    pub fn num_species(&self) -> usize {
        self.num_species
    }

    /// Returns the size of the spatial grid.
    #[inline]
    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    /// Returns a reference to the concentration slice for a specific species.
    #[inline]
    pub fn species(&self, index: usize) -> &[f64] {
        let start = index * self.grid_size;
        &self.concentrations[start..start + self.grid_size]
    }

    /// Returns a mutable reference to the concentration slice for a specific species.
    #[inline]
    pub fn species_mut(&mut self, index: usize) -> &mut [f64] {
        let start = index * self.grid_size;
        &mut self.concentrations[start..start + self.grid_size]
    }
}

// Implement standard ops for ODE integration compatibility
impl Add for ChemicalState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            *val += r_val;
        }
        self
    }
}

impl AddAssign for ChemicalState {
    fn add_assign(&mut self, rhs: Self) {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            *val += r_val;
        }
    }
}

impl Mul<f64> for ChemicalState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for val in self.concentrations.iter_mut() {
            *val *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for ChemicalState {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.concentrations.iter_mut() {
            *val *= scalar;
        }
    }
}

impl VectorOperations for ChemicalState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(other.concentrations.iter())
        {
            *val += r_val * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.num_species != other.num_species || self.grid_size != other.grid_size {
            // Reallocate if dimensions mismatch
            self.concentrations = other.concentrations.clone();
            self.num_species = other.num_species;
            self.grid_size = other.grid_size;
            return;
        }
        self.concentrations.copy_from_slice(&other.concentrations);
    }

    fn copy_from_scaled(&mut self, source: &Self, other: &Self, scale: f64) {
        if self.num_species != source.num_species || self.grid_size != source.grid_size {
            // Reallocate if dimensions mismatch.
            self.concentrations = vec![0.0; source.concentrations.len()];
            self.num_species = source.num_species;
            self.grid_size = source.grid_size;
        }

        // Fused 1D loop: self = source + other * scale (highly vectorizable)
        for ((dst, src), oth) in self
            .concentrations
            .iter_mut()
            .zip(source.concentrations.iter())
            .zip(other.concentrations.iter())
        {
            *dst = *src + *oth * scale;
        }
    }
}
