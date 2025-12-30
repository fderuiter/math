use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

/// Represents the group assignment for a patient in a clinical trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Treatment,
    Control,
}

/// Represents a patient with an ID and a stratum identifier (e.g., "Male-Young").
#[derive(Debug, Clone)]
pub struct Patient {
    pub id: String,
    pub stratum: String,
}

/// Strategy for allocating patients to groups.
///
/// This trait allows for swapping different randomization algorithms (Simple, Block, Minimization)
/// without changing the core trial execution logic.
pub trait AllocationStrategy {
    /// Allocates `n_patients` to groups according to the strategy.
    fn allocate(&mut self, n_patients: usize) -> Result<Vec<Group>, String>;
}

/// Simple Randomization Strategy.
///
/// Assigns each patient to Treatment or Control with equal probability (0.5), independently.
/// Does not guarantee balanced group sizes.
pub struct SimpleRandomizer<R> {
    rng: R,
}

impl SimpleRandomizer<rand::rngs::ThreadRng> {
    /// Creates a new SimpleRandomizer using the default thread-local RNG.
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Default for SimpleRandomizer<rand::rngs::ThreadRng> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Rng> SimpleRandomizer<R> {
    /// Creates a new SimpleRandomizer with a specific RNG (useful for deterministic testing).
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> AllocationStrategy for SimpleRandomizer<R> {
    fn allocate(&mut self, n_patients: usize) -> Result<Vec<Group>, String> {
        let mut assignments = Vec::with_capacity(n_patients);
        for _ in 0..n_patients {
            if self.rng.r#gen::<bool>() {
                assignments.push(Group::Treatment);
            } else {
                assignments.push(Group::Control);
            }
        }
        Ok(assignments)
    }
}

/// Block Randomization Strategy.
///
/// Ensures balanced allocation within blocks of a fixed size.
/// `block_size` must be even.
pub struct BlockRandomizer<R> {
    block_size: usize,
    rng: R,
}

impl BlockRandomizer<rand::rngs::ThreadRng> {
    /// Creates a new BlockRandomizer using the default thread-local RNG.
    pub fn new(block_size: usize) -> Self {
        Self { block_size, rng: thread_rng() }
    }
}

impl<R: Rng> BlockRandomizer<R> {
    /// Creates a new BlockRandomizer with a specific RNG.
    pub fn with_rng(block_size: usize, rng: R) -> Self {
        Self { block_size, rng }
    }
}

impl<R: Rng> AllocationStrategy for BlockRandomizer<R> {
    fn allocate(&mut self, n_patients: usize) -> Result<Vec<Group>, String> {
        if self.block_size % 2 != 0 {
            return Err("Block size must be even for 1:1 allocation.".to_string());
        }

        // Note: The original implementation didn't strictly fail if n_patients % block_size != 0,
        // it just truncated. We preserve this behavior but ensure we generate enough blocks.

        let num_blocks = (n_patients as f64 / self.block_size as f64).ceil() as usize;
        let mut assignments = Vec::with_capacity(num_blocks * self.block_size);

        for _ in 0..num_blocks {
            let mut block = Vec::with_capacity(self.block_size);
            for _ in 0..(self.block_size / 2) {
                block.push(Group::Treatment);
                block.push(Group::Control);
            }
            block.shuffle(&mut self.rng);
            assignments.extend(block);
        }

        // Truncate to exact number of patients
        if assignments.len() > n_patients {
            assignments.truncate(n_patients);
        }

        Ok(assignments)
    }
}

/// Stratified Randomization Wrapper.
///
/// Uses an underlying `AllocationStrategy` (like BlockRandomizer) within each stratum.
pub struct StratifiedRandomizer<S> {
    strategy: S,
}

impl<S: AllocationStrategy> StratifiedRandomizer<S> {
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    /// Performs randomization for a list of patients, respecting strata.
    pub fn randomize(&mut self, patients: &[Patient]) -> Result<HashMap<String, Group>, String> {
        // Group patients by stratum
        // We use indices to map back to patients to avoid cloning everything immediately
        let mut strata_map: HashMap<&str, Vec<usize>> = HashMap::new();
        for (i, p) in patients.iter().enumerate() {
            strata_map
                .entry(&p.stratum)
                .or_default()
                .push(i);
        }

        let mut final_assignments = HashMap::new();

        for (_stratum, patient_indices) in strata_map {
            let n = patient_indices.len();
            // Delegate to the strategy to get assignments for this stratum
            let assignments = self.strategy.allocate(n)?;

            for (i, &patient_idx) in patient_indices.iter().enumerate() {
                final_assignments.insert(patients[patient_idx].id.clone(), assignments[i]);
            }
        }

        Ok(final_assignments)
    }
}

// --- Legacy API Wrappers (Deprecated) ---

/// Performs Simple Randomization.
#[deprecated(since = "0.2.0", note = "Use `SimpleRandomizer::new().allocate(n)` instead")]
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    SimpleRandomizer::new().allocate(n_patients).unwrap_or_else(|_| vec![])
}

/// Performs Block Randomization.
#[deprecated(since = "0.2.0", note = "Use `BlockRandomizer::new(block_size).allocate(n)` instead")]
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    BlockRandomizer::new(block_size).allocate(n_patients)
}

/// Performs Stratified Randomization.
#[deprecated(since = "0.2.0", note = "Use `StratifiedRandomizer` with `BlockRandomizer` instead")]
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    let strategy = BlockRandomizer::new(block_size);
    let mut strat_randomizer = StratifiedRandomizer::new(strategy);
    strat_randomizer.randomize(patients)
}
