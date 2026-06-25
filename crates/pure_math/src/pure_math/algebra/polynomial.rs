//! # Polynomial Ring
//!
//! Implementation of polynomials with coefficients in a Ring.

use crate::pure_math::algebra::traits::Ring;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A polynomial with coefficients in a Ring T.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<T: Ring> {
    pub coeffs: Vec<T>, // coeffs[i] is coefficient of x^i
}

impl<T: Ring> Polynomial<T> {
    #[verified_engine::verified]
    pub fn new(coeffs: Vec<T>) -> Self {
        let mut p = Polynomial { coeffs };
        p.trim();
        p
    }

    /// Removes trailing zeros.
    #[verified_engine::verified]
    fn trim(&mut self) {
        while self.coeffs.len() > 1 && self.coeffs.last().is_some_and(|c| c.is_zero()) {
            self.coeffs.pop();
        }
        // Ensure at least one element (0) exists if empty?
        // Or representation: empty vec = 0?
        // Let's say empty vec = 0.
        if self.coeffs.len() == 1 && self.coeffs[0].is_zero() {
            self.coeffs.pop();
        }
    }

    #[verified_engine::verified]
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None // Degree of zero polynomial is -infinity (represented as None)
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Evaluation at x using Horner's method.
    #[verified_engine::verified]
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
    #[verified_engine::verified]
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
    #[verified_engine::verified]
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
    #[verified_engine::verified]
    fn zero() -> Self {
        Polynomial::new(vec![])
    }
    #[verified_engine::verified]
    fn one() -> Self {
        Polynomial::new(vec![T::one()])
    }
}

impl<T: Ring> AddAssign for Polynomial<T> {
    #[verified_engine::verified]
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T: Ring> Sub for Polynomial<T> {
    type Output = Self;
    #[verified_engine::verified]
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<T: Ring> SubAssign for Polynomial<T> {
    #[verified_engine::verified]
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T: Ring> MulAssign for Polynomial<T> {
    #[verified_engine::verified]
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T: Ring> Neg for Polynomial<T> {
    type Output = Self;
    #[verified_engine::verified]
    fn neg(self) -> Self {
        Polynomial::new(self.coeffs.into_iter().map(|c| -c).collect())
    }
}
