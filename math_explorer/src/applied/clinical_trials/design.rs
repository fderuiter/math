use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::marker::PhantomData;

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

/// Trait defining a strategy for allocating patients to groups.
///
/// Complies with the Strategy Pattern (OCP) and allows for Dependency Injection (DIP).
pub trait AllocationStrategy {
    /// Generates assignments for a given number of patients.
    fn assign(&mut self, n_patients: usize) -> Result<Vec<Group>, String>;
}

/// Strategy for Simple Randomization (Coin Toss).
pub struct SimpleRandomizer<R: Rng> {
    rng: R,
}

impl<R: Rng> SimpleRandomizer<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> AllocationStrategy for SimpleRandomizer<R> {
    fn assign(&mut self, n_patients: usize) -> Result<Vec<Group>, String> {
        let mut assignments = Vec::with_capacity(n_patients);
        for _ in 0..n_patients {
            if self.rng.gen_bool(0.5) {
                assignments.push(Group::Treatment);
            } else {
                assignments.push(Group::Control);
            }
        }
        Ok(assignments)
    }
}

/// Strategy for Block Randomization.
pub struct BlockRandomizer<R: Rng> {
    block_size: usize,
    rng: R,
}

impl<R: Rng> BlockRandomizer<R> {
    pub fn new(block_size: usize, rng: R) -> Self {
        Self { block_size, rng }
    }
}

impl<R: Rng> AllocationStrategy for BlockRandomizer<R> {
    fn assign(&mut self, n_patients: usize) -> Result<Vec<Group>, String> {
        if !self.block_size.is_multiple_of(2) {
            return Err("Block size must be even for 1:1 allocation.".to_string());
        }

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

        assignments.truncate(n_patients);
        Ok(assignments)
    }
}

/// Strategy for Stratified Randomization.
///
/// Composes another strategy to apply within each stratum.
pub struct StratifiedRandomizer<S, F>
where
    S: AllocationStrategy,
    F: Fn() -> S,
{
    strategy_factory: F,
    _marker: PhantomData<S>,
}

impl<S, F> StratifiedRandomizer<S, F>
where
    S: AllocationStrategy,
    F: Fn() -> S,
{
    pub fn new(strategy_factory: F) -> Self {
        Self {
            strategy_factory,
            _marker: PhantomData,
        }
    }

    pub fn assign_stratified(
        &self,
        patients: &[Patient],
    ) -> Result<HashMap<String, Group>, String> {
        let mut strata_map: HashMap<String, Vec<&Patient>> = HashMap::new();
        for p in patients {
            strata_map.entry(p.stratum.clone()).or_default().push(p);
        }

        let mut final_assignments = HashMap::new();

        for (_stratum, patients_in_stratum) in strata_map {
            let n = patients_in_stratum.len();
            let mut strategy = (self.strategy_factory)();
            let assignments = strategy.assign(n)?;

            for (i, p) in patients_in_stratum.iter().enumerate() {
                final_assignments.insert(p.id.clone(), assignments[i]);
            }
        }

        Ok(final_assignments)
    }
}

// --- Legacy API Wrappers ---

#[deprecated(note = "Use SimpleRandomizer struct instead")]
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    let rng = rand::thread_rng();
    let mut strategy = SimpleRandomizer::new(rng);
    strategy.assign(n_patients).unwrap()
}

#[deprecated(note = "Use BlockRandomizer struct instead")]
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    let rng = rand::thread_rng();
    let mut strategy = BlockRandomizer::new(block_size, rng);
    strategy.assign(n_patients)
}

#[deprecated(note = "Use StratifiedRandomizer struct instead")]
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
) -> Result<HashMap<String, Group>, String> {
    let factory = move || {
        let rng = rand::thread_rng();
        BlockRandomizer::new(block_size, rng)
    };
    let randomizer = StratifiedRandomizer::new(factory);
    randomizer.assign_stratified(patients)
}
