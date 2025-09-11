//! # Prime Numbers
//!
//! This module provides functions for generating prime numbers.

/// Generates prime numbers up to a given limit using the Sieve of Eratosthenes.
pub fn primes_up_to(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            for multiple in (i * i..=limit as usize).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(num, &is_p)| if is_p { Some(num as u64) } else { None })
        .collect()
}

/// A simple primality test.
/// This is not very efficient for large numbers, but it is correct.
/// It should be replaced with a more robust test if needed for performance-critical code.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
