//! Second Quantization framework.

/// Particle statistics type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    /// Fermions follow Fermi-Dirac statistics and Pauli Exclusion Principle.
    /// {c_i, c_j^\dagger} = \delta_{ij}
    Fermion,
    /// Bosons follow Bose-Einstein statistics.
    /// [a_i, a_j^\dagger] = \delta_{ij}
    Boson,
}

/// Type of Quantum Operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumOperatorType {
    /// Creates a particle in a state (raising operator).
    Creation,
    /// Annihilates a particle from a state (lowering operator).
    Annihilation,
}

/// A quantum operator acting on a specific state index (k-vector or site).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operator {
    pub op_type: QuantumOperatorType,
    pub index: usize,
}

impl Operator {
    pub fn new(op_type: QuantumOperatorType, index: usize) -> Self {
        Self { op_type, index }
    }
}

/// Fock State representation using occupation numbers.
/// |n_1, n_2, ..., n_M>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FockState {
    /// Occupation numbers for each mode/site.
    /// For Fermions, valid values are 0 or 1.
    /// For Bosons, values can be any non-negative integer (limited by u8 here).
    pub occupations: Vec<u8>,
}

impl FockState {
    /// Creates a new vacuum state |0, 0, ...> with given size.
    pub fn new(size: usize) -> Self {
        Self {
            occupations: vec![0; size],
        }
    }

    /// Sets the occupation number of a specific state directly.
    pub fn set_occupation(&mut self, index: usize, count: u8, p_type: ParticleType) -> Result<(), String> {
        if index >= self.occupations.len() {
            return Err(format!("Index {} out of bounds", index));
        }
        if p_type == ParticleType::Fermion && count > 1 {
            return Err("Pauli Exclusion Principle: Fermions cannot occupy same state > 1".to_string());
        }
        self.occupations[index] = count;
        Ok(())
    }

    /// Tries to add a particle to the state (Apply creation operator).
    pub fn create_particle(&mut self, index: usize, p_type: ParticleType) -> Result<(), String> {
        if index >= self.occupations.len() {
            return Err(format!("Index {} out of bounds", index));
        }
        let current = self.occupations[index];
        match p_type {
            ParticleType::Fermion => {
                if current >= 1 {
                    return Err("Pauli Exclusion: State already occupied".to_string());
                }
                self.occupations[index] = 1;
            },
            ParticleType::Boson => {
                if current == u8::MAX {
                    return Err("Boson saturation (u8 max)".to_string());
                }
                self.occupations[index] += 1;
            }
        }
        Ok(())
    }
}

/// Checks the canonical commutation (Boson) or anti-commutation (Fermion) relations.
///
/// Returns the value of:
/// * `{op1, op2}` for Fermions. Expected to be `delta_{ij}` for {c, c^\dagger}.
/// * `[op1, op2]` for Bosons. Expected to be `delta_{ij}` for [a, a^\dagger].
pub fn check_commutation(op1: &Operator, op2: &Operator, p_type: ParticleType) -> f64 {
    match p_type {
        ParticleType::Fermion => {
            // Fermions: Anti-commutator {A, B} = AB + BA
            // {c_i, c_j^\dagger} = delta_{ij}
            match (op1.op_type, op2.op_type) {
                (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) |
                (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                    if op1.index == op2.index { 1.0 } else { 0.0 }
                }
                _ => 0.0, // {c, c} = 0, {c^\dagger, c^\dagger} = 0
            }
        }
        ParticleType::Boson => {
            // Bosons: Commutator [A, B] = AB - BA
            // [a_i, a_j^\dagger] = delta_{ij}
            // [a_i^\dagger, a_j] = -delta_{ij}
            match (op1.op_type, op2.op_type) {
                (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) => {
                     if op1.index == op2.index { 1.0 } else { 0.0 }
                },
                (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                     if op1.index == op2.index { -1.0 } else { 0.0 }
                },
                _ => 0.0, // [a, a] = 0, [a^\dagger, a^\dagger] = 0
            }
        }
    }
}
