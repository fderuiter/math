use crate::math_types::{Integer, Rational};

pub struct DyadicRational {
    pub m: i64,
    pub r: u32,
}

impl DyadicRational {
    pub fn new(m: i64, r: u32) -> Self { Self { m, r } }
}
