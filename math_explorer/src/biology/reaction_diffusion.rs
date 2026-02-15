//! Generic Reaction-Diffusion System
//!
//! This module provides a flexible framework for simulating N-species reaction-diffusion systems.
//! It abstracts over the state representation (`ChemicalState`), reaction kinetics (`ReactionModel`),
//! and spatial diffusion (`DiffusionModel`).

use crate::pure_math::analysis::ode::traits::{OdeSystem, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents the state of a multi-species chemical system.
///
/// Stores concentrations in a "Structure of Arrays" format: `Vec<Vec<f64>>`
/// where the outer vector indexes species, and the inner vector indexes spatial points.
#[derive(Debug, Clone, PartialEq)]
pub struct ChemicalState {
    /// Concentrations of each species across the spatial grid.
    /// Outer index: Species ID. Inner index: Spatial Grid Point ID.
    pub concentrations: Vec<Vec<f64>>,
}

impl ChemicalState {
    /// Creates a new zero-initialized chemical state.
    pub fn new(num_species: usize, grid_size: usize) -> Self {
        Self {
            concentrations: vec![vec![0.0; grid_size]; num_species],
        }
    }

    /// Returns the number of chemical species.
    pub fn num_species(&self) -> usize {
        self.concentrations.len()
    }

    /// Returns the size of the spatial grid.
    pub fn grid_size(&self) -> usize {
        if self.concentrations.is_empty() {
            0
        } else {
            self.concentrations[0].len()
        }
    }

    /// Returns a reference to the concentration slice for a specific species.
    pub fn species(&self, index: usize) -> &[f64] {
        &self.concentrations[index]
    }

    /// Returns a mutable reference to the concentration slice for a specific species.
    pub fn species_mut(&mut self, index: usize) -> &mut [f64] {
        &mut self.concentrations[index]
    }
}

// Implement standard ops for ODE integration compatibility
impl Add for ChemicalState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for (s, r) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            for (val, r_val) in s.iter_mut().zip(r.iter()) {
                *val += r_val;
            }
        }
        self
    }
}

impl AddAssign for ChemicalState {
    fn add_assign(&mut self, rhs: Self) {
        for (s, r) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            for (val, r_val) in s.iter_mut().zip(r.iter()) {
                *val += r_val;
            }
        }
    }
}

impl Mul<f64> for ChemicalState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for s in self.concentrations.iter_mut() {
            for val in s.iter_mut() {
                *val *= scalar;
            }
        }
        self
    }
}

impl MulAssign<f64> for ChemicalState {
    fn mul_assign(&mut self, scalar: f64) {
        for s in self.concentrations.iter_mut() {
            for val in s.iter_mut() {
                *val *= scalar;
            }
        }
    }
}

impl VectorOperations for ChemicalState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (s, r) in self
            .concentrations
            .iter_mut()
            .zip(other.concentrations.iter())
        {
            for (val, r_val) in s.iter_mut().zip(r.iter()) {
                *val += r_val * scale;
            }
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.num_species() != other.num_species() || self.grid_size() != other.grid_size() {
            // Reallocate if dimensions mismatch
            self.concentrations = other.concentrations.clone();
            return;
        }
        for (s, r) in self
            .concentrations
            .iter_mut()
            .zip(other.concentrations.iter())
        {
            s.copy_from_slice(r);
        }
    }

    fn copy_from_scaled(&mut self, source: &Self, other: &Self, scale: f64) {
        if self.num_species() != source.num_species() || self.grid_size() != source.grid_size() {
            // Reallocate if dimensions mismatch.
            // Efficient reallocation: initialize with zeros, then loop will fill it.
            self.concentrations = vec![vec![0.0; source.grid_size()]; source.num_species()];
        }

        // Fused loop: self = source + other * scale
        for ((s_self, s_src), s_oth) in self
            .concentrations
            .iter_mut()
            .zip(source.concentrations.iter())
            .zip(other.concentrations.iter())
        {
            for ((dst, src), oth) in s_self.iter_mut().zip(s_src.iter()).zip(s_oth.iter()) {
                *dst = *src + *oth * scale;
            }
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

    /// Computes reaction rates for the entire grid and adds them to `rates`.
    ///
    /// The default implementation iterates over grid points, gathers local concentrations,
    /// calls `reaction`, and scatters the result back.
    /// Override this for performance (e.g., using SIMD or GPU kernels).
    fn add_reaction_batch(&self, state: &ChemicalState, rates: &mut ChemicalState) {
        let n_species = state.num_species();
        let n_grid = state.grid_size();
        let mut local_concs = vec![0.0; n_species];
        let mut local_rates = vec![0.0; n_species];

        for i in 0..n_grid {
            // Gather
            for (s, conc) in local_concs.iter_mut().enumerate().take(n_species) {
                *conc = state.concentrations[s][i];
            }

            // Compute
            self.reaction(&local_concs, &mut local_rates);

            // Scatter-Add
            for (s, rate) in local_rates.iter().enumerate().take(n_species) {
                rates.concentrations[s][i] += *rate;
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

/// A generic Reaction-Diffusion model for N species.
/// This struct holds the physics definition but no simulation state.
pub struct ReactionDiffusionModel<R: ReactionModel, D: DiffusionModel> {
    pub reaction: R,
    pub diffusion: D,
    pub diffusion_coeffs: Vec<f64>,
}

impl<R: ReactionModel, D: DiffusionModel> ReactionDiffusionModel<R, D> {
    pub fn new(reaction: R, diffusion: D, diffusion_coeffs: Vec<f64>) -> Self {
        Self {
            reaction,
            diffusion,
            diffusion_coeffs,
        }
    }
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

        // Add Reaction (using the new batch method)
        self.reaction.add_reaction_batch(state, out);
    }
}

/// A generic Reaction-Diffusion system for N species.
/// This struct wraps the model and state to provide a convenient simulation interface.
pub struct ReactionDiffusionSystem<R: ReactionModel, D: DiffusionModel> {
    pub state: ChemicalState,
    /// Internal buffer for storing the time derivative ($dC/dt$).
    pub derivative_buffer: ChemicalState,
    /// The underlying physics model.
    pub model: ReactionDiffusionModel<R, D>,
}

impl<R: ReactionModel, D: DiffusionModel> ReactionDiffusionSystem<R, D> {
    pub fn new(
        num_species: usize,
        grid_size: usize,
        reaction: R,
        diffusion: D,
        diffusion_coeffs: Vec<f64>,
    ) -> Self {
        assert_eq!(diffusion_coeffs.len(), num_species);
        Self {
            state: ChemicalState::new(num_species, grid_size),
            derivative_buffer: ChemicalState::new(num_species, grid_size),
            model: ReactionDiffusionModel::new(reaction, diffusion, diffusion_coeffs),
        }
    }

    /// Advances the system by `dt` using a simple Euler step.
    /// For more advanced integration, use `pure_math::analysis::ode::solvers`.
    pub fn step(&mut self, dt: f64) {
        // Use the model to compute the derivative into the buffer
        self.model
            .derivative_in_place(0.0, &self.state, &mut self.derivative_buffer);
        self.state.scale_add(&self.derivative_buffer, dt);
    }
}

impl<R: ReactionModel, D: DiffusionModel> OdeSystem<ChemicalState>
    for ReactionDiffusionSystem<R, D>
{
    fn derivative(&self, t: f64, state: &ChemicalState) -> ChemicalState {
        self.model.derivative(t, state)
    }

    fn derivative_in_place(&self, t: f64, state: &ChemicalState, out: &mut ChemicalState) {
        self.model.derivative_in_place(t, state, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::diffusion::FiniteDifference1D;
    use crate::biology::morphogenesis::SchnakenbergKinetics;

    #[test]
    fn test_reaction_diffusion_system_equivalence() {
        // Setup a small system
        let n = 10;
        let d_u = 1.0;
        let d_v = 0.5;
        let dx = 1.0;

        let kinetics = SchnakenbergKinetics::default();
        let diffusion = FiniteDifference1D::new(dx);
        let diffusion_coeffs = vec![d_u, d_v];

        let mut system = ReactionDiffusionSystem::new(2, n, kinetics, diffusion, diffusion_coeffs);

        // Initialize with same pattern as in morphogenesis test
        // u = 1.0 + 0.1 * i
        // v = 0.5 - 0.05 * i
        for i in 0..n {
            system.state.species_mut(0)[i] = 1.0 + 0.1 * (i as f64);
            system.state.species_mut(1)[i] = 0.5 - 0.05 * (i as f64);
        }

        // Run for a few steps
        let dt = 0.01;
        for _ in 0..5 {
            system.step(dt);
        }

        // Capture output
        let u_out = system.state.species(0).to_vec();
        let v_out = system.state.species(1).to_vec();

        // Expected values (same as in morphogenesis.rs)
        let expected_u = vec![
            0.9798926377401955,
            1.0722504645444493,
            1.1685990805783317,
            1.2642647090938448,
            1.359028327357602,
            1.4527705800845148,
            1.5453811790730032,
            1.6367576303434268,
            1.7267186541725483,
            1.8109737170223916,
        ];
        let expected_v = vec![
            0.47709091921002866,
            0.4263770084483741,
            0.3750152156844884,
            0.32443296992262166,
            0.2747722006954079,
            0.22615405798594523,
            0.1786914141249832,
            0.1324883911255509,
            0.08765106523936222,
            0.04531981611374585,
        ];

        // Assert with tolerance
        let tolerance = 1e-10;
        for i in 0..n {
            assert!(
                (u_out[i] - expected_u[i]).abs() < tolerance,
                "U mismatch at {}: {} vs {}",
                i,
                u_out[i],
                expected_u[i]
            );
            assert!(
                (v_out[i] - expected_v[i]).abs() < tolerance,
                "V mismatch at {}: {} vs {}",
                i,
                v_out[i],
                expected_v[i]
            );
        }
    }
}
