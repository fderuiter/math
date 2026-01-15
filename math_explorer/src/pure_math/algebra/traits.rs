use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub, SubAssign};

/// A trait for types that form a Ring (with unity).
/// We also require Division and Remainder for Euclidean domain properties.
pub trait Ring:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Clone
    + PartialEq
    + Debug
    + Sized
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl Ring for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
