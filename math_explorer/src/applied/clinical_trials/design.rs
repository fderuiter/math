use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;

/// Strategy for allocating patients to groups.
pub trait AllocationStrategy {
    /// Generates assignments for `n` patients.
    fn assign<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String>;
}

/// Simple Randomization strategy.
pub struct SimpleRandomizer;

impl AllocationStrategy for SimpleRandomizer {
    fn assign<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String> {
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

/// Block Randomization strategy.
pub struct BlockRandomizer {
    pub block_size: usize,
}

impl BlockRandomizer {
    pub fn new(block_size: usize) -> Self {
        Self { block_size }
    }
}

impl AllocationStrategy for BlockRandomizer {
    fn assign<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Result<Vec<Group>, String> {
         // usize::is_multiple_of is experimental or non-existent in stable rust without traits.
         // checking if I can use remainder operator
        if self.block_size % 2 != 0 {
            return Err("Block size must be even for 1:1 allocation.".to_string());
        }
        // Strict block randomization logic similar to original function

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

/// Performs Simple Randomization.
/// Each patient is assigned to Treatment or Control with equal probability (0.5).
/// Note: This does not guarantee equal group sizes, especially for small sample sizes.
#[deprecated(note = "Use `SimpleRandomizer` instead for dependency injection.")]
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    let strat = SimpleRandomizer;
    let mut rng = thread_rng();
    strat.assign(n_patients, &mut rng).unwrap()
}

/// Performs Block Randomization.
/// Ensures that the number of patients in each group is balanced within blocks of `block_size`.
/// `block_size` must be a multiple of 2 (assuming 1:1 allocation).
#[deprecated(note = "Use `BlockRandomizer` instead for dependency injection.")]
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    let strat = BlockRandomizer::new(block_size);
    let mut rng = thread_rng();
    strat.assign(n_patients, &mut rng)
}

/// Strategy for stratified randomization.
pub struct StratifiedRandomizer<F, S>
where
    F: Fn() -> S,
    S: AllocationStrategy,
{
    strategy_factory: F,
}

impl<F, S> StratifiedRandomizer<F, S>
where
    F: Fn() -> S,
    S: AllocationStrategy,
{
    /// Creates a new StratifiedRandomizer.
    /// `strategy_factory` is a function that returns a new instance of the strategy for each stratum.
    /// This is necessary because some strategies (like BlockRandomizer) might have state or configuration.
    /// Although in this simplified model they are stateless or simple, passing a factory is safer.
    pub fn new(strategy_factory: F) -> Self {
        Self { strategy_factory }
    }

    pub fn randomize<R: Rng + ?Sized>(
        &self,
        patients: &[Patient],
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

        for (_stratum, patients_in_stratum) in strata_map {
            let n = patients_in_stratum.len();
            // Create a new strategy instance for this stratum
            let strategy = (self.strategy_factory)();
            let assignments = strategy.assign(n, rng)?;

            for (i, p) in patients_in_stratum.iter().enumerate() {
                final_assignments.insert(p.id.clone(), assignments[i]);
            }
        }

        Ok(final_assignments)
    }
}

/// Performs Stratified Randomization.
/// Separates patients into strata and performs block randomization within each stratum.
/// Returns a map of Patient ID to Group assignment.
#[deprecated(note = "Use `StratifiedRandomizer` instead for dependency injection.")]
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    let factory = || BlockRandomizer::new(block_size);
    let randomizer = StratifiedRandomizer::new(factory);
    let mut rng = thread_rng();
    randomizer.randomize(patients, &mut rng)
}
