use crate::pure_math::analysis::ode::VectorOperations;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents the state of a Turing system at a point in time.
///
/// This struct encapsulates the concentration vectors for the activator and inhibitor,
/// protecting them from invalid resizing while providing safe access.
#[derive(Debug, Clone, PartialEq)]
pub struct TuringState {
    pub(crate) u: Vec<f64>,
    pub(crate) v: Vec<f64>,
}

impl TuringState {
    /// Creates a new zero-initialized state of a given size.
    pub fn new(size: usize) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
        }
    }

    /// Returns a slice of the activator concentrations.
    pub fn u(&self) -> &[f64] {
        &self.u
    }

    /// Returns a slice of the inhibitor concentrations.
    pub fn v(&self) -> &[f64] {
        &self.v
    }

    /// Returns a mutable slice of the activator concentrations.
    pub fn u_mut(&mut self) -> &mut [f64] {
        &mut self.u
    }

    /// Returns a mutable slice of the inhibitor concentrations.
    pub fn v_mut(&mut self) -> &mut [f64] {
        &mut self.v
    }

    /// Returns the length of the grid.
    pub fn len(&self) -> usize {
        self.u.len()
    }

    /// Returns true if the grid is empty.
    pub fn is_empty(&self) -> bool {
        self.u.is_empty()
    }
}

impl Add for TuringState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for (u, r) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += r;
        }
        for (v, r) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += r;
        }
        self
    }
}

impl AddAssign for TuringState {
    fn add_assign(&mut self, rhs: Self) {
        for (u, r) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += r;
        }
        for (v, r) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += r;
        }
    }
}

impl Mul<f64> for TuringState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for u in self.u.iter_mut() {
            *u *= scalar;
        }
        for v in self.v.iter_mut() {
            *v *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for TuringState {
    fn mul_assign(&mut self, scalar: f64) {
        for u in self.u.iter_mut() {
            *u *= scalar;
        }
        for v in self.v.iter_mut() {
            *v *= scalar;
        }
    }
}

impl VectorOperations for TuringState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (u, r) in self.u.iter_mut().zip(other.u.iter()) {
            *u += r * scale;
        }
        for (v, r) in self.v.iter_mut().zip(other.v.iter()) {
            *v += r * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.u.len() != other.u.len() {
            self.u.resize(other.u.len(), 0.0);
            self.v.resize(other.v.len(), 0.0);
        }
        self.u.copy_from_slice(&other.u);
        self.v.copy_from_slice(&other.v);
    }
}
