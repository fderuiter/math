//! # FRACTRAN and Universal Minsky Machines
//!
//! This module provides a complete, mathematically rigorous implementation of FRACTRAN,
//! an esoteric programming language developed by John Conway, and its relationship
//! to Universal Minsky Machines (UMM).
//!
//! ## Mathematical Foundations
//!
//! A **FRACTRAN** program is an ordered list of positive rational numbers:
//! $$ f_1, f_2, \dots, f_k $$
//!
//! The state of the machine is represented by a single positive integer $N$.
//! In each step, the machine evaluates the products $N \cdot f_i$ in order.
//! The state is updated to the first product that is an integer:
//! $$ N \leftarrow N \cdot f_i $$
//! The machine halts when no such product is an integer.
//!
//! ### FRACTRAN Encoding of Minsky Machines
//!
//! Minsky machines are register machines that can be simulated by FRACTRAN.
//! Given registers $r_1, r_2, \dots$ and states $s_1, s_2, \dots$, we assign:
//! - Distinct primes $p_j$ for each register $r_j$.
//! - Distinct primes $q_i$ for each state $s_i$.
//!
//! The state of the Minsky machine with register values $a_1, a_2, \dots$ and
//! state $s_i$ is encoded as:
//! $$ N = q_i \prod p_j^{a_j} $$
//!
//! Each Minsky instruction is compiled into one or more FRACTRAN fractions:
//!
//! 1.  **INC $r_j$ and go to $s_{next}$:**
//!     $$ \frac{q_{next} \cdot p_j}{q_i} $$
//!
//! 2.  **JZDEC $r_j$: If $a_j > 0$, decrement $r_j$ and go to $s_T$, else go to $s_F$:**
//!     This requires two fractions in sequence:
//!     $$ \frac{q_T}{q_i \cdot p_j} \quad \text{(Success: } a_j > 0 \text{)} $$
//!     $$ \frac{q_F}{q_i} \quad \text{(Fail/Jump: } a_j = 0 \text{)} $$

use rug::ops::Pow;
use rug::{Integer, Rational};

/// An instruction for a Minsky Machine.
///
/// Implementors define how an instruction is compiled into a list of FRACTRAN fractions.
pub trait MinskyInstruction: std::fmt::Debug + Send + Sync + MinskyInstructionClone {
    /// Compiles the instruction into FRACTRAN fractions using the given compiler.
    fn compile(&self, compiler: &FractranCompiler) -> Vec<Rational>;
}

/// Helper trait for cloning boxed MinskyInstructions.
pub trait MinskyInstructionClone {
    fn clone_box(&self) -> Box<dyn MinskyInstruction>;
}

impl<T> MinskyInstructionClone for T
where
    T: 'static + MinskyInstruction + Clone,
{
    fn clone_box(&self) -> Box<dyn MinskyInstruction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MinskyInstruction> {
    fn clone(&self) -> Box<dyn MinskyInstruction> {
        self.clone_box()
    }
}

/// Increment register `r_j` and transition to state `s_next`.
///
/// FRACTRAN encoding: $\frac{s_{next} \cdot p_j}{s_{curr}}$
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncInstruction {
    /// The current state.
    pub state: usize,
    /// The register index to increment.
    pub register: usize,
    /// The next state to transition to.
    pub next_state: usize,
}

impl MinskyInstruction for IncInstruction {
    fn compile(&self, compiler: &FractranCompiler) -> Vec<Rational> {
        // frac: (s_next * p_r) / s
        let s_next = &compiler.state_primes[self.next_state];
        let p_r = &compiler.register_primes[self.register];
        let s = &compiler.state_primes[self.state];

        let numer = s_next.clone() * p_r;
        let denom = s.clone();
        vec![Rational::from((numer, denom))]
    }
}

/// Jump-if-Zero or Decrement.
/// If register `r_j > 0`, decrement `r_j` and transition to `s_success`.
/// Else, transition to `s_fail`.
///
/// FRACTRAN encoding:
/// 1. $\frac{s_{success}}{s_{curr} \cdot p_j}$
/// 2. $\frac{s_{fail}}{s_{curr}}$
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JzdecInstruction {
    /// The current state.
    pub state: usize,
    /// The register index to check/decrement.
    pub register: usize,
    /// The state to transition to if `r_j > 0`.
    pub success_state: usize,
    /// The state to transition to if `r_j == 0`.
    pub fail_state: usize,
}

impl MinskyInstruction for JzdecInstruction {
    fn compile(&self, compiler: &FractranCompiler) -> Vec<Rational> {
        let mut fractions = Vec::with_capacity(2);

        // frac 1: s_T / (s * p_r)
        let s_success = &compiler.state_primes[self.success_state];
        let s = &compiler.state_primes[self.state];
        let p_r = &compiler.register_primes[self.register];

        let numer1 = s_success.clone();
        let denom1 = s.clone() * p_r;
        fractions.push(Rational::from((numer1, denom1)));

        // frac 2: s_F / s
        let s_fail = &compiler.state_primes[self.fail_state];
        let numer2 = s_fail.clone();
        let denom2 = s.clone();
        fractions.push(Rational::from((numer2, denom2)));

        fractions
    }
}

/// A Minsky Machine containing a list of instructions.
#[derive(Clone)]
pub struct MinskyMachine {
    pub instructions: Vec<Box<dyn MinskyInstruction>>,
}

impl std::fmt::Debug for MinskyMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinskyMachine")
            .field("instructions", &self.instructions)
            .finish()
    }
}

impl MinskyMachine {
    /// Create a new Minsky Machine.
    pub fn new(instructions: Vec<Box<dyn MinskyInstruction>>) -> Self {
        Self { instructions }
    }
}

/// A FRACTRAN program represented by a list of rational numbers.
///
/// # Example
///
/// ```
/// use oxidize_pure_math::algorithmic_information::fractran::FractranProgram;
/// use rug::{Integer, Rational};
///
/// // A simple program to add two numbers: r0 and r1
/// // r0 is represented by p=2, r1 by q=3.
/// // FRACTRAN fraction: 3/2.
/// // State N = 2^a * 3^b -> N * (3/2)^a = 3^(a+b)
/// let prog = FractranProgram::new(vec![Rational::from((3, 2))]);
///
/// use rug::ops::Pow;
/// let initial_n = Integer::from(2).pow(5) * Integer::from(3).pow(7);
/// let (final_n, steps) = prog.execute(initial_n, 100);
///
/// assert_eq!(steps, 5); // It takes exactly 5 steps to move 'a' into 'b'
/// assert_eq!(final_n, Integer::from(3).pow(12)); // 5 + 7 = 12
/// ```
#[derive(Debug, Clone)]
pub struct FractranProgram {
    pub fractions: Vec<Rational>,
}

impl FractranProgram {
    /// Create a new FRACTRAN program.
    pub fn new(fractions: Vec<Rational>) -> Self {
        Self { fractions }
    }

    /// Execute a single step of the FRACTRAN program.
    /// Returns `Some(new_n)` if a step was taken, or `None` if the program halted.
    pub fn step(&self, n: &Integer) -> Option<Integer> {
        // Bolt Optimization: Replace rational arithmetic with integer operations
        for fraction in &self.fractions {
            let num = fraction.numer();
            let den = fraction.denom();
            if n.is_divisible(den) {
                let mut next_n = n.clone();
                next_n *= num;
                next_n /= den;
                return Some(next_n);
            }
        }
        None
    }

    /// Execute the program for a maximum number of steps or until halting.
    pub fn execute(&self, initial_n: Integer, max_steps: usize) -> (Integer, usize) {
        let mut n = initial_n;
        for steps in 0..max_steps {
            if let Some(next_n) = self.step(&n) {
                n = next_n;
            } else {
                return (n, steps);
            }
        }
        (n, max_steps)
    }
}

/// Compiler to translate a Minsky Machine into a FRACTRAN program.
pub struct FractranCompiler {
    pub state_primes: Vec<Integer>,
    pub register_primes: Vec<Integer>,
}

impl FractranCompiler {
    /// Create a new FRACTRAN compiler with the given prime assignments.
    pub fn new(state_primes: Vec<u64>, register_primes: Vec<u64>) -> Self {
        Self {
            state_primes: state_primes.into_iter().map(Integer::from).collect(),
            register_primes: register_primes.into_iter().map(Integer::from).collect(),
        }
    }

    /// Compile a Minsky Machine into a FRACTRAN program.
    pub fn compile(&self, machine: &MinskyMachine) -> FractranProgram {
        let mut fractions = Vec::new();

        for instr in &machine.instructions {
            let mut compiled_fractions = instr.compile(self);
            fractions.append(&mut compiled_fractions);
        }

        FractranProgram::new(fractions)
    }

    /// Encode the state into the integer `N = s_i * p_1^r_1 * p_2^r_2 * ...`
    pub fn encode_state(&self, state: usize, registers: &[u32]) -> Integer {
        let mut n = self.state_primes[state].clone();
        for (i, &val) in registers.iter().enumerate() {
            let p_r = &self.register_primes[i];
            let p_r_pow = p_r.clone().pow(val);
            n *= p_r_pow;
        }
        n
    }
}

/// Constants defining a Universal FRACTRAN Architecture.
/// Simulates a Universal Turing Machine with minimal space requirements.
pub struct UniversalFractranArchitecture;

impl UniversalFractranArchitecture {
    /// Get the 14 state primes required for the minimal UTM simulation.
    pub fn state_primes() -> Vec<u64> {
        vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
    }

    /// Get the 2 register primes required for the minimal UTM simulation.
    pub fn register_primes() -> Vec<u64> {
        vec![2, 53]
    }

    /// Get the standard compiler for the 16-prime universal architecture.
    pub fn standard_compiler() -> FractranCompiler {
        FractranCompiler::new(Self::state_primes(), Self::register_primes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractran_addition() {
        // Simple program to add two registers: r0 and r1
        // r0 is represented by p=2, r1 by q=3.
        // FRACTRAN fraction: 3/2.
        // N = 2^a * 3^b -> N * (3/2)^a = 3^(a+b)
        let prog = FractranProgram::new(vec![Rational::from((3, 2))]);

        let a = 5;
        let b = 7;
        let initial_n = Integer::from(2).pow(a) * Integer::from(3).pow(b);

        let (final_n, steps) = prog.execute(initial_n, 100);

        // Expect exactly `a` steps
        assert_eq!(steps, a as usize);
        assert_eq!(final_n, Integer::from(3).pow(a + b));
    }

    #[test]
    fn test_minsky_to_fractran() {
        // State 0: INC r0 -> State 1
        // State 1: JZDEC r0 (success -> 0, fail -> 2)
        // state primes: 2, 3, 5
        // register primes: 7
        let compiler = FractranCompiler::new(vec![2, 3, 5], vec![7]);

        let machine = MinskyMachine::new(vec![
            Box::new(IncInstruction {
                state: 0,
                register: 0,
                next_state: 1,
            }),
            Box::new(JzdecInstruction {
                state: 1,
                register: 0,
                success_state: 0,
                fail_state: 2,
            }),
        ]);

        let prog = compiler.compile(&machine);

        // Expected fractions:
        // INC: (s1 * p0) / s0 = (3 * 7) / 2 = 21/2
        // JZDEC success: s0 / (s1 * p0) = 2 / (3 * 7) = 2/21
        // JZDEC fail: s2 / s1 = 5 / 3

        let frac1 = Rational::from((21, 2));
        let frac2 = Rational::from((2, 21));
        let frac3 = Rational::from((5, 3));

        assert_eq!(prog.fractions, vec![frac1, frac2, frac3]);
    }
}
