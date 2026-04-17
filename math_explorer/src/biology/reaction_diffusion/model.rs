use crate::pure_math::analysis::ode::traits::{OdeSystem, VectorOperations};
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

/// Defines the local reaction kinetics for N species.
pub trait ReactionModel {
    /// Computes the reaction rates for a single spatial point.
    ///
    /// # Arguments
    /// * `concentrations`: The current concentrations of all species at this point.
    /// * `rates`: Output buffer for the computed reaction rates (dC/dt).
    fn reaction(&self, concentrations: &[f64], rates: &mut [f64]);

    /// Computes and accumulates reaction rates for the entire grid.
    ///
    /// This method allows for vectorized implementations that process multiple grid points
    /// efficiently. The default implementation iterates over grid points and calls `reaction`.
    ///
    /// # Arguments
    /// * `state`: Current chemical state (concentrations).
    /// * `out_rates`: Chemical state buffer to accumulate reaction rates into.
    fn add_reaction_batch(&self, state: &ChemicalState, out_rates: &mut ChemicalState) {
        let n_species = state.num_species();
        if n_species == 0 {
            return;
        }
        let n_grid = state.grid_size();

        let mut local_concs = vec![0.0; n_species];
        let mut local_rates = vec![0.0; n_species];

        for i in 0..n_grid {
            // Gather
            for (s, conc) in local_concs.iter_mut().enumerate().take(n_species) {
                *conc = state.species(s)[i];
            }

            self.reaction(&local_concs, &mut local_rates);

            // Scatter (Accumulate)
            for (s, rate) in local_rates.iter().enumerate().take(n_species) {
                out_rates.species_mut(s)[i] += rate;
            }
        }
    }
}

/// Defines the spatial diffusion strategy for N species.
pub trait DiffusionModel {
    /// Applies the diffusion operator (Laplacian) to the full state.
    ///
    /// # Arguments
    /// * `state`: Current chemical state.
    /// * `out`: Output buffer for the diffusion term (D * Laplacian).
    /// * `coeffs`: Diffusion coefficients for each species.
    fn apply(&self, state: &ChemicalState, out: &mut ChemicalState, coeffs: &[f64]);
}

/// A pure physics definition of a Reaction-Diffusion system.
///
/// This struct encapsulates the immutable laws of the system (reaction kinetics, diffusion strategy,
/// and coefficients), separating them from the mutable simulation state.
///
/// It implements `OdeSystem` to calculate the time derivative ($dC/dt$) given a state.
pub struct ReactionDiffusionModel<R, D> {
    pub reaction: R,
    pub diffusion: D,
    pub diffusion_coeffs: Vec<f64>,
}

impl<R: ReactionModel, D: DiffusionModel> OdeSystem<ChemicalState>
    for ReactionDiffusionModel<R, D>
{
    fn derivative(&self, _t: f64, state: &ChemicalState) -> ChemicalState {
        let mut out = ChemicalState::new(state.num_species(), state.grid_size());
        self.derivative_in_place(_t, state, &mut out);
        out
    }

    fn derivative_in_place(&self, _t: f64, state: &ChemicalState, out: &mut ChemicalState) {
        // Compute Diffusion
        self.diffusion.apply(state, out, &self.diffusion_coeffs);

        // Add Reaction
        // Optimized: Use batch processing to allow vectorization and avoid gather/scatter
        self.reaction.add_reaction_batch(state, out);
    }
}
