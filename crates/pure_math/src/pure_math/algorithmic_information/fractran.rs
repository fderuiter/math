use crate::math_types::Rational;

pub struct FractranProgram {
    pub fractions: Vec<Rational>,
}

impl FractranProgram {
    pub fn new(fractions: Vec<Rational>) -> Self { Self { fractions } }
}

pub struct FractranState {
    pub state_primes: Vec<u64>,
    pub register_primes: Vec<u64>,
}
