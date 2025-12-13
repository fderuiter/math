use rand::seq::SliceRandom;
use rand::thread_rng;
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

/// Performs Simple Randomization.
/// Each patient is assigned to Treatment or Control with equal probability (0.5).
/// Note: This does not guarantee equal group sizes, especially for small sample sizes.
pub fn simple_randomization(n_patients: usize) -> Vec<Group> {
    let mut assignments = Vec::with_capacity(n_patients);
    for _ in 0..n_patients {
        if rand::random() {
            assignments.push(Group::Treatment);
        } else {
            assignments.push(Group::Control);
        }
    }
    assignments
}

/// Performs Block Randomization.
/// Ensures that the number of patients in each group is balanced within blocks of `block_size`.
/// `block_size` must be a multiple of 2 (assuming 1:1 allocation).
pub fn block_randomization(n_patients: usize, block_size: usize) -> Result<Vec<Group>, String> {
    if !block_size.is_multiple_of(2) {
        return Err("Block size must be even for 1:1 allocation.".to_string());
    }
    if !n_patients.is_multiple_of(block_size) {
        // We can either return an error or handle the remainder.
        // For strict block randomization, let's assume n must fit blocks or we fill the last partial block?
        // Usually, trials recruit until a number. Let's just generate enough blocks to cover n_patients.
    }

    let mut rng = thread_rng();
    let num_blocks = (n_patients as f64 / block_size as f64).ceil() as usize;
    let mut assignments = Vec::with_capacity(num_blocks * block_size);

    for _ in 0..num_blocks {
        let mut block = Vec::with_capacity(block_size);
        for _ in 0..(block_size / 2) {
            block.push(Group::Treatment);
            block.push(Group::Control);
        }
        block.shuffle(&mut rng);
        assignments.extend(block);
    }

    // Truncate to exact number of patients if n_patients is not a multiple of block_size
    assignments.truncate(n_patients);
    Ok(assignments)
}

/// Performs Stratified Randomization.
/// Separates patients into strata and performs block randomization within each stratum.
/// Returns a map of Patient ID to Group assignment.
pub fn stratified_randomization(
    patients: &[Patient],
    block_size: usize,
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
        // Generate assignments for this stratum
        let assignments = block_randomization(n, block_size)?;

        for (i, p) in patients_in_stratum.iter().enumerate() {
            final_assignments.insert(p.id.clone(), assignments[i]);
        }
    }

    Ok(final_assignments)
}
