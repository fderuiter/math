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
///
/// Implements the Strategy Pattern to allow dynamic selection of randomization methods
/// (e.g., Simple, Block, Urn) and dependency injection of the Random Number Generator (RNG).
pub trait AllocationStrategy {
    /// Generates a sequence of group assignments for `n` patients.
    ///
    /// # Arguments
    /// * `rng` - A mutable reference to a random number generator.
    /// * `n` - The number of assignments to generate.
    ///
    /// # Returns
    /// * `Ok(Vec<Group>)` - The sequence of assignments.
    /// * `Err(String)` - If configuration is invalid (e.g., uneven block size).
    fn allocate<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Result<Vec<Group>, String>;
}

/// Simple Randomization Strategy.
///
/// Each patient is assigned to Treatment or Control with equal probability (0.5),
/// independent of previous assignments.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleRandomization;

impl SimpleRandomization {
    pub fn new() -> Self {
        Self
    }
}

impl AllocationStrategy for SimpleRandomization {
    fn allocate<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Result<Vec<Group>, String> {
        let mut assignments = Vec::with_capacity(n);
        for _ in 0..n {
            if rng.r#gen::<bool>() {
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
/// Ensures balance between groups within small "blocks" of patients.
/// This prevents significant imbalance in group sizes over time.
#[derive(Debug, Clone, Copy)]
pub struct BlockRandomization {
    block_size: usize,
}

impl BlockRandomization {
    /// Creates a new BlockRandomization strategy.
    ///
    /// # Arguments
    /// * `block_size` - The size of each block. Must be even for 1:1 allocation.
    pub fn new(block_size: usize) -> Result<Self, String> {
        if block_size == 0 {
            return Err("Block size cannot be zero.".to_string());
        }
        if block_size % 2 != 0 {
            return Err("Block size must be even for 1:1 allocation.".to_string());
        }
        Ok(Self { block_size })
    }
}

impl AllocationStrategy for BlockRandomization {
    fn allocate<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Result<Vec<Group>, String> {
        // Note: Logic copied from original function but adapted for the trait
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

        // Truncate to exact number of patients if n is not a multiple of block_size
        assignments.truncate(n);
        Ok(assignments)
    }
}

/// Performs Simple Randomization (Legacy Wrapper).
///
/// **Deprecated**: Use `SimpleRandomization` struct instead.
#[deprecated(since = "0.2.0", note = "Use `SimpleRandomization` struct directly.")]
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    // We unwrap here because SimpleRandomization never fails
    SimpleRandomization::new()
        .allocate(&mut thread_rng(), n_patients)
        .unwrap()
}

/// Performs Block Randomization (Legacy Wrapper).
///
/// **Deprecated**: Use `BlockRandomization` struct instead.
#[deprecated(since = "0.2.0", note = "Use `BlockRandomization` struct directly.")]
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    let strategy = BlockRandomization::new(block_size)?;
    strategy.allocate(&mut thread_rng(), n_patients)
}

/// Performs Stratified Randomization.
///
/// Separates patients into strata and applies the provided `strategy` within each stratum.
///
/// # Arguments
/// * `patients` - List of patients with stratum information.
/// * `strategy` - The allocation strategy to apply (e.g., `BlockRandomization`).
///
/// # Returns
/// * `Result<HashMap<String, Group>, String>` - Map of Patient ID to Group.
pub fn stratified_randomization_with<S: AllocationStrategy>(
    patients: &[Patient],
    strategy: &S,
) -> Result<HashMap<String, Group>, String> {
    let mut rng = thread_rng();
    stratified_randomization_with_rng(patients, strategy, &mut rng)
}

/// Performs Stratified Randomization with a specific RNG.
///
/// Useful for deterministic testing.
pub fn stratified_randomization_with_rng<S: AllocationStrategy, R: Rng + ?Sized>(
    patients: &[Patient],
    strategy: &S,
    rng: &mut R,
) -> Result<HashMap<String, Group>, String> {
    // Group patients by stratum
    let mut strata_map: HashMap<String, Vec<&Patient>> = HashMap::new();
    for p in patients {
        strata_map
            .entry(p.stratum.clone())
            .or_default()
            .push(p);
    }

    let mut final_assignments = HashMap::new();

    // To ensure determinism with the passed RNG, we should sort keys or iterate deterministically
    // HashMap iteration order is random.
    let mut strata_keys: Vec<_> = strata_map.keys().collect();
    strata_keys.sort();

    for stratum in strata_keys {
        let patients_in_stratum = &strata_map[stratum];
        let n = patients_in_stratum.len();
        // Generate assignments for this stratum using the provided strategy
        let assignments = strategy.allocate(rng, n)?;

        for (i, p) in patients_in_stratum.iter().enumerate() {
            final_assignments.insert(p.id.clone(), assignments[i]);
        }
    }

    Ok(final_assignments)
}

/// Performs Stratified Randomization (Legacy Wrapper).
///
/// Defaults to Block Randomization with the given `block_size`.
///
/// **Deprecated**: Use `stratified_randomization_with` and pass a strategy.
#[deprecated(since = "0.2.0", note = "Use `stratified_randomization_with` and pass a strategy.")]
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    let strategy = BlockRandomization::new(block_size)?;
    stratified_randomization_with(patients, &strategy)
}
