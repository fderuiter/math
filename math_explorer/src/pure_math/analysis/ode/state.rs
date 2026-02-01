use super::traits::VectorOperations;
use nalgebra::{DVector, Vector2, Vector3};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// A wrapper around `Vec<f64>` that implements `VectorOperations`.
/// Use this when you need a heap-allocated state vector.
#[derive(Debug, Clone, PartialEq)]
pub struct VecState(pub Vec<f64>);

impl Add for VecState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        let len = std::cmp::min(self.0.len(), rhs.0.len());
        self.0.truncate(len);
        for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
            *a += b;
        }
        self
    }
}

impl AddAssign for VecState {
    fn add_assign(&mut self, rhs: Self) {
        let len = std::cmp::min(self.0.len(), rhs.0.len());
        // Use zip to avoid bounds checks and handle length mismatch gracefully
        for (a, b) in self.0.iter_mut().zip(rhs.0.iter()).take(len) {
            *a += b;
        }
    }
}

impl Mul<f64> for VecState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for val in self.0.iter_mut() {
            *val *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for VecState {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.0.iter_mut() {
            *val *= scalar;
        }
    }
}

impl VectorOperations for VecState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        let len = std::cmp::min(self.0.len(), other.0.len());
        for (a, b) in self.0.iter_mut().zip(other.0.iter()).take(len) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        // Reuse buffer if possible
        if self.0.len() != other.0.len() {
            self.0.resize(other.0.len(), 0.0);
        }
        self.0.copy_from_slice(&other.0);
    }
}

/// A zero-overhead wrapper for fixed-size arrays implementing `VectorOperations`.
///
/// Use this for small systems where stack allocation is preferred.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayState<const N: usize>(pub [f64; N]);

impl<const N: usize> Add for ArrayState<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut arr = [0.0; N];
        for (i, val) in arr.iter_mut().enumerate() {
            *val = self.0[i] + rhs.0[i];
        }
        Self(arr)
    }
}

impl<const N: usize> Mul<f64> for ArrayState<N> {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        let mut arr = [0.0; N];
        for (i, val) in arr.iter_mut().enumerate() {
            *val = self.0[i] * scalar;
        }
        Self(arr)
    }
}

impl<const N: usize> AddAssign for ArrayState<N> {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
            *a += b;
        }
    }
}

impl<const N: usize> MulAssign<f64> for ArrayState<N> {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.0.iter_mut() {
            *val *= scalar;
        }
    }
}

impl<const N: usize> VectorOperations for ArrayState<N> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.0 = other.0;
    }
}

// Implementations for nalgebra types

impl VectorOperations for Vector2<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        *self += other * scale;
    }

    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl VectorOperations for Vector3<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        *self += other * scale;
    }

    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl VectorOperations for DVector<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        // Use slice iteration to avoid temporary allocations from 'other * scale'
        // This assumes DVector storage is contiguous which it is for standard DVector.
        for (a, b) in self.as_mut_slice().iter_mut().zip(other.as_slice().iter()) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.copy_from(other);
    }
}
