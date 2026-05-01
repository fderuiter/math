//! # Finite Fields
//!
//! Implementation of Finite Fields (Integers Modulo Prime P).

use crate::pure_math::algebra::traits::{Field, Ring};
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
    #[allow(clippy::suspicious_arithmetic_impl)]
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
