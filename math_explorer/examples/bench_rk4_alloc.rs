use math_explorer::pure_math::analysis::ode::{OdeSystem, TimeStepper, VectorOperations};
use std::time::Instant;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Debug)]
struct SimpleState {
    data: Vec<f64>,
}

impl SimpleState {
    fn new(size: usize) -> Self {
        Self { data: vec![0.0; size] }
    }
}

impl VectorOperations for SimpleState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b * scale;
        }
    }
    fn copy_from(&mut self, other: &Self) {
        // self.data.copy_from_slice(&other.data); // VectorOperations doesn't imply Vec, but we know it here.
        // Wait, copy_from_slice panics if lengths differ.
        if self.data.len() != other.data.len() {
            self.data.resize(other.data.len(), 0.0);
        }
        self.data.copy_from_slice(&other.data);
    }
}

impl Add for SimpleState {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.scale_add(&rhs, 1.0);
        self
    }
}
impl Mul<f64> for SimpleState {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self {
        for a in self.data.iter_mut() {
            *a *= rhs;
        }
        self
    }
}
impl AddAssign for SimpleState {
    fn add_assign(&mut self, rhs: Self) {
        self.scale_add(&rhs, 1.0);
    }
}
impl MulAssign<f64> for SimpleState {
    fn mul_assign(&mut self, rhs: f64) {
        for a in self.data.iter_mut() {
            *a *= rhs;
        }
    }
}

struct BenchmarkSystem {
    state: SimpleState,
}

impl OdeSystem<SimpleState> for BenchmarkSystem {
    fn derivative(&self, _t: f64, state: &SimpleState) -> SimpleState {
        let mut d = state.clone();
        for x in d.data.iter_mut() {
            *x = -*x;
        }
        d
    }
    fn derivative_in_place(&self, _t: f64, state: &SimpleState, out: &mut SimpleState) {
        if out.data.len() != state.data.len() {
            out.data.resize(state.data.len(), 0.0);
        }
        for (o, i) in out.data.iter_mut().zip(state.data.iter()) {
            *o = -*i;
        }
    }
}

impl TimeStepper<SimpleState> for BenchmarkSystem {
    fn get_state(&self) -> &SimpleState {
        &self.state
    }
    fn get_state_mut(&mut self) -> &mut SimpleState {
        &mut self.state
    }
}

fn main() {
    let size = 10000;
    let mut system = BenchmarkSystem {
        state: SimpleState { data: vec![1.0; size] },
    };

    // Warmup
    for _ in 0..10 {
        system.step(0.01);
    }

    let start = Instant::now();
    let steps = 1000;
    let dt = 0.01;

    for _ in 0..steps {
        system.step(dt);
    }

    let duration = start.elapsed();
    println!("Time for {} steps of size {}: {:?}", steps, size, duration);
}
