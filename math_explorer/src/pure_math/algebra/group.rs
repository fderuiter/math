//! # Group Theory
//!
//! This module provides implementations of common groups and tools for group theory analysis.

use crate::pure_math::algebra::traits::{Semigroup, Monoid, Group};
use std::collections::HashSet;
use std::hash::Hash;

// ============================================================================
// Cyclic Group (Integers Modulo n)
// ============================================================================

/// An element of the Cyclic Group $Z_n$ under addition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CyclicElement {
    pub value: usize,
    pub modulo: usize,
}

impl CyclicElement {
    pub fn new(value: usize, modulo: usize) -> Self {
        CyclicElement {
            value: value % modulo,
            modulo,
        }
    }
}

impl Semigroup for CyclicElement {
    fn operate(a: &Self, b: &Self) -> Self {
        assert_eq!(a.modulo, b.modulo, "Cannot operate on elements from different groups");
        CyclicElement {
            value: (a.value + b.value) % a.modulo,
            modulo: a.modulo,
        }
    }
}

impl Monoid for CyclicElement {
    fn identity() -> Self {
        // This is tricky because identity needs context (modulo).
        // The trait method `identity()` doesn't take self.
        // This suggests the `Group` trait as defined in traits.rs is for types where identity is unique globally
        // (like i64, f64) or the type captures the group structure fully.
        // For CyclicElement, we can't implement `Monoid` correctly without knowing the modulus if we use `identity() -> Self`.
        // However, for the purpose of this exercise, we might assume a fixed global context or panic,
        // OR we change the design.
        //
        // A better design might be to have a `Group` struct that acts on `Element` types.
        // But adhering to the requested "Group Theory" overview, and the trait I wrote:
        // `fn identity() -> Self`.
        // This limits us to types where identity is constant.

        // workaround: return a placeholder or use a specific type per n (const generics).
        // For this implementation, I will use a panic for the static method and encourage using instance methods if possible,
        // but `Monoid` requires `identity()`.

        // Let's use 0 mod 1 as a dummy default, but this is flawed.
        // BETTER APPROACH: Use const generics for CyclicGroup<N>.
        panic!("Use CyclicGroup::<N>::identity() or specific constructors. Generic trait identity cannot infer modulo.")
    }
}

// To properly implement the traits as defined, we should use const generics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Zn<const N: usize> {
    pub value: usize,
}

impl<const N: usize> Zn<N> {
    pub fn new(value: usize) -> Self {
        Zn { value: value % N }
    }
}

impl<const N: usize> Semigroup for Zn<N> {
    fn operate(a: &Self, b: &Self) -> Self {
        Zn { value: (a.value + b.value) % N }
    }
}

impl<const N: usize> Monoid for Zn<N> {
    fn identity() -> Self {
        Zn { value: 0 }
    }
}

impl<const N: usize> Group for Zn<N> {
    fn inverse(&self) -> Self {
        if self.value == 0 {
            Zn { value: 0 }
        } else {
            Zn { value: N - self.value }
        }
    }
}

// ============================================================================
// Symmetric Group (Permutations)
// ============================================================================

/// An element of the Symmetric Group $S_n$ (Permutations).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Permutation {
    pub map: Vec<usize>, // map[i] is the image of i
}

impl Permutation {
    pub fn new(map: Vec<usize>) -> Self {
        // Validation: map must contain 0..n
        let n = map.len();
        let mut seen = vec![false; n];
        for &x in &map {
            if x >= n || seen[x] {
                panic!("Invalid permutation");
            }
            seen[x] = true;
        }
        Permutation { map }
    }

    pub fn identity(n: usize) -> Self {
        Permutation { map: (0..n).collect() }
    }
}

impl Semigroup for Permutation {
    fn operate(a: &Self, b: &Self) -> Self {
        // Function composition: (a * b)(x) = a(b(x))
        assert_eq!(a.map.len(), b.map.len(), "Permutations must be of same size");
        let n = a.map.len();
        let mut new_map = vec![0; n];
        for i in 0..n {
            new_map[i] = a.map[b.map[i]];
        }
        Permutation { map: new_map }
    }
}

// Again, Monoid for Permutation is hard without const generics or knowing size.
// I will implement a wrapper for Fixed size permutations or just utility functions.
// But let's implementing the logic for analysis.

// ============================================================================
// Group Analysis Tools
// ============================================================================

/// Checks if a subset H is a subgroup of G.
/// Requires G to be finite and represented by a list of all elements, or we just check closure/inverse for H.
/// Here we check if H is closed under operation and inverses.
pub fn is_subgroup<G: Group + Eq + Hash>(subgroup: &[G]) -> bool {
    if subgroup.is_empty() {
        return false;
    }

    // Check identity presence
    // Actually, simply checking closure under ab^-1 (One-Step Subgroup Test) is enough for nonempty subsets.

    // We need to know if elements are in subgroup.
    let set: HashSet<&G> = subgroup.iter().collect();

    for a in subgroup {
        for b in subgroup {
            // Check a * b^-1 in H
            let b_inv = b.inverse();
            let product = G::operate(a, &b_inv);
            if !set.contains(&product) {
                return false;
            }
        }
    }
    true
}

/// Generates a right coset Ha = { h * a : h in H }.
pub fn generate_right_coset<G: Group>(subgroup: &[G], a: &G) -> Vec<G> {
    subgroup.iter().map(|h| G::operate(h, a)).collect()
}

/// Generates a left coset aH = { a * h : h in H }.
pub fn generate_left_coset<G: Group>(subgroup: &[G], a: &G) -> Vec<G> {
    subgroup.iter().map(|h| G::operate(a, h)).collect()
}

/// Checks if a subgroup is normal.
/// A subgroup N is normal if aN = Na for all a in G.
/// We need the full group G for this check.
pub fn is_normal_subgroup<G: Group + Eq + Hash>(group: &[G], subgroup: &[G]) -> bool {
    if !is_subgroup(subgroup) {
        return false;
    }

    // Convert subgroup to Set for fast lookup
    let sub_set: HashSet<&G> = subgroup.iter().collect();

    for a in group {
        // Check aNa^-1 <= N
        let a_inv = a.inverse();
        for n in subgroup {
            let conj = G::operate(&G::operate(a, n), &a_inv);
            if !sub_set.contains(&conj) {
                return false;
            }
        }
    }
    true
}
