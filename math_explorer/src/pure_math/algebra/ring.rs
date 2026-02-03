//! # Rings and Fields
//!
//! This module provides implementations of Rings and Fields, including Polynomial Rings and Finite Fields.

use crate::pure_math::algebra::traits::{EuclideanDomain, Field, Ring};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

// ============================================================================
// Finite Fields (Integers Modulo Prime P)
// ============================================================================

/// A generic finite field $\mathbb{F}_p$.
/// P must be prime. This is not checked at compile time but expected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fp<const P: i64> {
    pub value: i64,
}

impl<const P: i64> Fp<P> {
    pub fn new(value: i64) -> Self {
        Fp {
            value: value.rem_euclid(P),
        }
    }
}

// Implement Operations
impl<const P: i64> Add for Fp<P> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Fp::new(self.value + other.value)
    }
}

impl<const P: i64> AddAssign for Fp<P> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const P: i64> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Fp::new(self.value - other.value)
    }
}

impl<const P: i64> SubAssign for Fp<P> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const P: i64> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Fp::new(self.value * other.value)
    }
}

impl<const P: i64> MulAssign for Fp<P> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const P: i64> Neg for Fp<P> {
    type Output = Self;
    fn neg(self) -> Self {
        Fp::new(-self.value)
    }
}

impl<const P: i64> Ring for Fp<P> {
    fn zero() -> Self {
        Fp::new(0)
    }
    fn one() -> Self {
        Fp::new(1)
    }
}

impl<const P: i64> Div for Fp<P> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.multiplicative_inverse()
    }
}

impl<const P: i64> Field for Fp<P> {
    fn multiplicative_inverse(&self) -> Self {
        if self.is_zero() {
            panic!("Division by zero in Fp");
        }
        // Extended Euclidean Algorithm to find inverse
        let (g, x, _) = extended_gcd(self.value, P);
        if g != 1 {
            panic!("Element has no inverse (P might not be prime)");
        }
        Fp::new(x)
    }
}

// Helper for Extended Euclidean Algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (g, x, y)
    }
}

impl<const P: i64> fmt::Display for Fp<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ============================================================================
// Polynomial Ring K[t]
// ============================================================================

/// A polynomial with coefficients in a Ring T.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<T: Ring> {
    pub coeffs: Vec<T>, // coeffs[i] is coefficient of x^i
}

impl<T: Ring> Polynomial<T> {
    pub fn new(coeffs: Vec<T>) -> Self {
        let mut p = Polynomial { coeffs };
        p.trim();
        p
    }

    /// Removes trailing zeros.
    fn trim(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last().map_or(false, |c| c.is_zero()) {
            self.coeffs.pop();
        }
        // Ensure at least one element (0) exists if empty?
        // Or representation: empty vec = 0?
        // Let's say empty vec = 0.
        if self.coeffs.len() == 1 && self.coeffs[0].is_zero() {
            self.coeffs.pop();
        }
    }

    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None // Degree of zero polynomial is -infinity (represented as None)
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Evaluation at x using Horner's method.
    pub fn eval(&self, x: T) -> T {
        if self.coeffs.is_empty() {
            return T::zero();
        }
        let mut result = T::zero();
        for c in self.coeffs.iter().rev() {
            result = result * x.clone() + c.clone();
        }
        result
    }
}

impl<T: Ring> Add for Polynomial<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut new_coeffs = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coeffs.get(i).cloned().unwrap_or(T::zero());
            let b = other.coeffs.get(i).cloned().unwrap_or(T::zero());
            new_coeffs.push(a + b);
        }
        Polynomial::new(new_coeffs)
    }
}

impl<T: Ring> Mul for Polynomial<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return Polynomial::new(vec![]);
        }
        let deg1 = self.coeffs.len() - 1;
        let deg2 = other.coeffs.len() - 1;
        let mut new_coeffs = vec![T::zero(); deg1 + deg2 + 1];

        for (i, c1) in self.coeffs.iter().enumerate() {
            for (j, c2) in other.coeffs.iter().enumerate() {
                new_coeffs[i + j] += c1.clone() * c2.clone();
            }
        }
        Polynomial::new(new_coeffs)
    }
}

// Implement other Ring traits for Polynomial (omitted for brevity unless strictly needed by generic Ring bounds)
// Since we want Polynomial<T> to be a Ring, we must implement AddAssign, Sub, SubAssign, MulAssign, Neg, One, Zero.
// To satisfy the generic constraints for testing, I'll implement a subset or all if possible.
// For now, let's stick to basic operations demonstrated above. The user can use basic operators.
// Note: To implement `Ring` trait for `Polynomial<T>`, we need all of them.

impl<T: Ring> Ring for Polynomial<T> {
    fn zero() -> Self {
        Polynomial::new(vec![])
    }
    fn one() -> Self {
        Polynomial::new(vec![T::one()])
    }
}

impl<T: Ring> AddAssign for Polynomial<T> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T: Ring> Sub for Polynomial<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<T: Ring> SubAssign for Polynomial<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T: Ring> MulAssign for Polynomial<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: Ring> Neg for Polynomial<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Polynomial::new(self.coeffs.into_iter().map(|c| -c).collect())
    }
}
