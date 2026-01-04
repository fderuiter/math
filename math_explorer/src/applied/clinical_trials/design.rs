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

/// A strategy for assigning patients to groups.
///
/// This trait allows for different randomization algorithms (Simple, Block, Adaptive)
/// to be used interchangeably.
pub trait AllocationStrategy {
    /// Generates the next group assignment.
    ///
    /// # Arguments
    /// * `stratum` - Optional stratum identifier for stratified randomization.
    fn assign(&mut self, stratum: Option<&str>) -> Result<Group, String>;
}

/// Simple Randomization Strategy.
///
/// Assigns Treatment/Control with equal probability (coin flip).
/// Stateless and does not guarantee balance.
pub struct SimpleRandomizer<R> {
    rng: R,
}

impl<R: Rng> SimpleRandomizer<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> AllocationStrategy for SimpleRandomizer<R> {
    fn assign(&mut self, _stratum: Option<&str>) -> Result<Group, String> {
        if self.rng.r#gen() {
            Ok(Group::Treatment)
        } else {
            Ok(Group::Control)
        }
    }
}

/// Block Randomization Strategy.
///
/// Ensures balance within blocks of a fixed size.
/// State is maintained in a buffer.
pub struct BlockRandomizer<R> {
    block_size: usize,
    buffer: Vec<Group>,
    rng: R,
}

impl<R: Rng> BlockRandomizer<R> {
    pub fn new(block_size: usize, rng: R) -> Result<Self, String> {
        if block_size == 0 || block_size % 2 != 0 {
            return Err("Block size must be a positive even number.".to_string());
        }
        Ok(Self {
            block_size,
            buffer: Vec::with_capacity(block_size),
            rng,
        })
    }

    fn refill_buffer(&mut self) {
        self.buffer.clear();
        for _ in 0..(self.block_size / 2) {
            self.buffer.push(Group::Treatment);
            self.buffer.push(Group::Control);
        }
        self.buffer.shuffle(&mut self.rng);
    }
}

impl<R: Rng> AllocationStrategy for BlockRandomizer<R> {
    fn assign(&mut self, _stratum: Option<&str>) -> Result<Group, String> {
        if self.buffer.is_empty() {
            self.refill_buffer();
        }
        // pop() returns None only if empty, but we just refilled.
        Ok(self.buffer.pop().unwrap())
    }
}

/// Stratified Randomization Strategy.
///
/// Maintains a separate strategy instance for each stratum.
/// Uses a factory closure to instantiate new strategies as needed.
pub struct StratifiedRandomizer<S, F> {
    factory: F,
    strata: HashMap<String, S>,
}

impl<S: AllocationStrategy, F: Fn() -> S> StratifiedRandomizer<S, F> {
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            strata: HashMap::new(),
        }
    }
}

impl<S: AllocationStrategy, F: Fn() -> S> AllocationStrategy for StratifiedRandomizer<S, F> {
    fn assign(&mut self, stratum: Option<&str>) -> Result<Group, String> {
        let key = stratum.ok_or("Stratum is required for StratifiedRandomizer")?;
        let strategy = self
            .strata
            .entry(key.to_string())
            .or_insert_with(&self.factory);
        strategy.assign(Some(key))
    }
}

// --- Legacy Wrappers (Deprecated) ---

/// Performs Simple Randomization.
#[deprecated(since = "0.2.0", note = "Use `SimpleRandomizer` instead.")]
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    let mut randomizer = SimpleRandomizer::new(thread_rng());
    let mut assignments = Vec::with_capacity(n_patients);
    for _ in 0..n_patients {
        assignments.push(randomizer.assign(None).unwrap());
    }
    assignments
}

/// Performs Block Randomization.
#[deprecated(since = "0.2.0", note = "Use `BlockRandomizer` instead.")]
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    let mut randomizer = BlockRandomizer::new(block_size, thread_rng())?;
    let mut assignments = Vec::with_capacity(n_patients);
    for _ in 0..n_patients {
        assignments.push(randomizer.assign(None)?);
    }
    Ok(assignments)
}

/// Performs Stratified Randomization.
#[deprecated(since = "0.2.0", note = "Use `StratifiedRandomizer` instead.")]
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    // Note: The legacy implementation grouped by stratum first, then randomized.
    // This affects the order in which the RNG is consumed.
    // To preserve "logic" (if we consider RNG stream consumption order part of logic),
    // we should replicate that grouping.

    let mut strata_map: HashMap<String, Vec<&Patient>> = HashMap::new();
    for p in patients {
        strata_map.entry(p.stratum.clone()).or_default().push(p);
    }

    let mut final_assignments = HashMap::new();

    // We can't use StratifiedRandomizer here easily because we need to
    // process stratum by stratum to match legacy behavior of consuming RNG blocks.
    // So we just use BlockRandomizer per stratum.

    for (_stratum, patients_in_stratum) in strata_map {
        // Create a new BlockRandomizer for this stratum
        let mut randomizer = BlockRandomizer::new(block_size, thread_rng())?;

        for p in patients_in_stratum {
             final_assignments.insert(p.id.clone(), randomizer.assign(None)?);
        }
    }

    Ok(final_assignments)
}
