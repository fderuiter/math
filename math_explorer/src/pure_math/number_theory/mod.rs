//! # Number Theory
//!
//! This module focuses on concepts from number theory, the study of integers.
//! It will include algorithms for primality testing, factorization, modular arithmetic,
//! and other number-theoretic functions.

pub mod class_number;
pub mod hurwitz_kronecker;
pub mod primes;

pub use primes::is_prime;
pub use primes::primes_up_to;
