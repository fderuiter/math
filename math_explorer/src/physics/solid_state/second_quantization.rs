/// Particle statistics trait for defining quantum behavior.
/// This allows extension to other particles (e.g., Anyons) without modifying core logic.
pub trait ParticleStatistics {
    /// Checks the commutation/anti-commutation relation between two operators.
    fn check_commutation(&self, op1: &Operator, op2: &Operator) -> f64;

    /// Checks if setting a specific occupation count is valid.
    fn validate_occupation(&self, count: u8) -> Result<(), String>;

    /// Calculates the new occupation number after adding a particle.
    fn apply_creation(&self, current: u8) -> Result<u8, String>;
}

/// Fermion statistics implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fermion;

impl ParticleStatistics for Fermion {
    fn check_commutation(&self, op1: &Operator, op2: &Operator) -> f64 {
        // {c_i, c_j^\dagger} = delta_{ij}
        match (op1.op_type, op2.op_type) {
            (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) |
            (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                if op1.index == op2.index { 1.0 } else { 0.0 }
            }
            _ => 0.0,
        }
    }

    fn validate_occupation(&self, count: u8) -> Result<(), String> {
        if count > 1 {
            Err("Pauli Exclusion Principle: Fermions cannot occupy same state > 1".to_string())
        } else {
            Ok(())
        }
    }

    fn apply_creation(&self, current: u8) -> Result<u8, String> {
        if current >= 1 {
            Err("Pauli Exclusion: State already occupied".to_string())
        } else {
            Ok(1)
        }
    }
}

/// Boson statistics implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct Boson;

impl ParticleStatistics for Boson {
    fn check_commutation(&self, op1: &Operator, op2: &Operator) -> f64 {
        // [a_i, a_j^\dagger] = delta_{ij} => a_i a_j^\dagger - a_j^\dagger a_i = delta
        match (op1.op_type, op2.op_type) {
            (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) => {
                 if op1.index == op2.index { 1.0 } else { 0.0 }
            },
            (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                 if op1.index == op2.index { -1.0 } else { 0.0 }
            },
            _ => 0.0,
        }
    }

    fn validate_occupation(&self, _count: u8) -> Result<(), String> {
        Ok(())
    }

    fn apply_creation(&self, current: u8) -> Result<u8, String> {
        if current == u8::MAX {
            Err("Boson saturation (u8 max)".to_string())
        } else {
            Ok(current + 1)
        }
    }
}

/// Legacy Particle statistics type.
/// Kept for backward compatibility, delegates to the Strategy implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    /// Fermions follow Fermi-Dirac statistics and Pauli Exclusion Principle.
    Fermion,
    /// Bosons follow Bose-Einstein statistics.
    Boson,
}

impl ParticleStatistics for ParticleType {
    fn check_commutation(&self, op1: &Operator, op2: &Operator) -> f64 {
        match self {
            ParticleType::Fermion => Fermion.check_commutation(op1, op2),
            ParticleType::Boson => Boson.check_commutation(op1, op2),
        }
    }

    fn validate_occupation(&self, count: u8) -> Result<(), String> {
        match self {
            ParticleType::Fermion => Fermion.validate_occupation(count),
            ParticleType::Boson => Boson.validate_occupation(count),
        }
    }

    fn apply_creation(&self, current: u8) -> Result<u8, String> {
        match self {
            ParticleType::Fermion => Fermion.apply_creation(current),
            ParticleType::Boson => Boson.apply_creation(current),
        }
    }
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
    /// Supports generic ParticleStatistics (Fermion, Boson, or custom).
    pub fn set_occupation<S: ParticleStatistics>(&mut self, index: usize, count: u8, strategy: S) -> Result<(), String> {
        if index >= self.occupations.len() {
            return Err(format!("Index {} out of bounds", index));
        }
        strategy.validate_occupation(count)?;
        self.occupations[index] = count;
        Ok(())
    }

    /// Tries to add a particle to the state (Apply creation operator).
    /// Supports generic ParticleStatistics.
    pub fn create_particle<S: ParticleStatistics>(&mut self, index: usize, strategy: S) -> Result<(), String> {
        if index >= self.occupations.len() {
            return Err(format!("Index {} out of bounds", index));
        }
        let current = self.occupations[index];
        let new_val = strategy.apply_creation(current)?;
        self.occupations[index] = new_val;
        Ok(())
    }
}

/// Checks the canonical commutation relations.
/// Supports generic ParticleStatistics.
pub fn check_commutation<S: ParticleStatistics>(op1: &Operator, op2: &Operator, strategy: S) -> f64 {
    strategy.check_commutation(op1, op2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fermion_exclusion() {
        let mut state = FockState::new(2);
        // First addition should succeed
        assert!(state.create_particle(0, ParticleType::Fermion).is_ok());
        // Second addition to same state must fail
        assert!(state.create_particle(0, ParticleType::Fermion).is_err());
    }

    #[test]
    fn test_strategy_usage() {
        let mut state = FockState::new(2);
        // Using Struct Strategy directly
        assert!(state.create_particle(0, Fermion).is_ok());
        assert!(state.create_particle(0, Fermion).is_err());
    }

    #[test]
    fn test_commutation_logic() {
        let c_k = Operator::new(QuantumOperatorType::Annihilation, 1);
        let c_k_dag = Operator::new(QuantumOperatorType::Creation, 1);
        let c_q = Operator::new(QuantumOperatorType::Annihilation, 2);

        // Fermion: {c_k, c_k^dag} = 1
        let val = check_commutation(&c_k, &c_k_dag, ParticleType::Fermion);
        assert!((val - 1.0).abs() < 1e-9);

        // Fermion: {c_k, c_q} = 0
        let val2 = check_commutation(&c_k, &c_q, ParticleType::Fermion);
        assert!(val2.abs() < 1e-9);

        // Boson: [a_k, a_k^dag] = 1
        let val3 = check_commutation(&c_k, &c_k_dag, ParticleType::Boson);
        assert!((val3 - 1.0).abs() < 1e-9);
    }
}
