use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
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

/// A strategy for allocating patients to groups.
/// This trait allows for swapping randomization algorithms (e.g., Simple, Block, Minimization)
/// and dependency injection of the RNG for deterministic testing.
pub trait AllocationStrategy {
    /// Allocates `n` patients to groups.
    fn allocate<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String>;
}

/// Simple Randomization Strategy.
/// Each patient is assigned to Treatment or Control with equal probability (0.5).
pub struct SimpleRandomization;

impl AllocationStrategy for SimpleRandomization {
    fn allocate<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String> {
        let mut assignments = Vec::with_capacity(n);
        for _ in 0..n {
            if rng.r#gen() {
                assignments.push(Group::Treatment);
            } else {
                assignments.push(Group::Control);
            }
        }
        Ok(assignments)
    }
}

/// Block Randomization Strategy.
/// Ensures balance within blocks of `block_size`.
pub struct BlockRandomization {
    pub block_size: usize,
}

impl BlockRandomization {
    pub fn new(block_size: usize) -> Self {
        Self { block_size }
    }
}

impl AllocationStrategy for BlockRandomization {
    fn allocate<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String> {
        if self.block_size % 2 != 0 {
            return Err("Block size must be even for 1:1 allocation.".to_string());
        }

        let num_blocks = (n as f64 / self.block_size as f64).ceil() as usize;
        let mut assignments = Vec::with_capacity(num_blocks * self.block_size);

        for _ in 0..num_blocks {
            let mut block = Vec::with_capacity(self.block_size);
            for _ in 0..(self.block_size / 2) {
                block.push(Group::Treatment);
                block.push(Group::Control);
            }
            block.shuffle(rng);
            assignments.extend(block);
        }

        assignments.truncate(n);
        Ok(assignments)
    }
}

/// Performs Simple Randomization using the thread-local RNG.
/// Wrapper around `SimpleRandomization`.
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    SimpleRandomization.allocate(n_patients, &mut thread_rng()).unwrap()
}

/// Performs Block Randomization using the thread-local RNG.
/// Wrapper around `BlockRandomization`.
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    BlockRandomization::new(block_size).allocate(n_patients, &mut thread_rng())
}

/// Performs Stratified Randomization.
/// Separates patients into strata and performs randomization within each stratum using the provided strategy.
pub fn stratified_randomization_with_strategy<S: AllocationStrategy, R: Rng + ?Sized>(
    patients: &[Patient],
    strategy: &S,
    rng: &mut R,
) -> Result<HashMap<String, Group>, String> {
    let mut strata_map: HashMap<String, Vec<&Patient>> = HashMap::new();
    for p in patients {
        strata_map
            .entry(p.stratum.clone())
            .or_default()
            .push(p);
    }

    let mut final_assignments = HashMap::new();

    for (_stratum, patients_in_stratum) in strata_map {
        let n = patients_in_stratum.len();
        let assignments = strategy.allocate(n, rng)?;

        for (i, p) in patients_in_stratum.iter().enumerate() {
            final_assignments.insert(p.id.clone(), assignments[i]);
        }
    }

    Ok(final_assignments)
}

/// Performs Stratified Randomization using Block Randomization and thread-local RNG.
/// Kept for backward compatibility.
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    let strategy = BlockRandomization::new(block_size);
    let mut rng = thread_rng();
    stratified_randomization_with_strategy(patients, &strategy, &mut rng)
}
