//! Compartmental models for epidemiology (SIR, SEIR, etc).
//!
//! This module implements standard compartmental models using the Builder pattern
//! for safe construction and validation.

/// Macro to implement arithmetic operations for compartmental states.
macro_rules! impl_compartmental_ops {
    ($type:ty, $($field:ident),+) => {
        impl Add for $type {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($field: self.$field + rhs.$field),+
                }
            }
        }

        impl AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)+
            }
        }

        impl Mul<f64> for $type {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self {
                Self {
                    $($field: self.$field * scalar),+
                }
            }
        }

        impl MulAssign<f64> for $type {
            fn mul_assign(&mut self, scalar: f64) {
                $(self.$field *= scalar;)+
            }
        }

        impl VectorOperations for $type {
            fn scale_add(&mut self, other: &Self, scale: f64) {
                $(self.$field += other.$field * scale;)+
            }

            fn copy_from(&mut self, other: &Self) {
                *self = *other;
            }
        }
    };
}

pub mod seir;
pub mod sir;
pub mod validation;

pub use seir::{SEIRModel, SEIRModelBuilder, SEIRState};
pub use sir::{SIRModel, SIRModelBuilder, SIRState};

/// Calculates the Basic Reproduction Number ($R_0$).
///
/// $R_0 = \beta / \gamma$
pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
    if gamma == 0.0 {
        f64::INFINITY
    } else {
        beta / gamma
    }
}
