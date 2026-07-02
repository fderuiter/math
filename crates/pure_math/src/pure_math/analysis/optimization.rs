//! Optimization Algorithms.
//!
//! Provides structures and traits for mathematical optimization problems.

use super::evolution::{EvolutionEngine, EvolutionError};
use nalgebra::{DMatrix, DVector, RealField};
use rand::RngCore;
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during optimization.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum OptimizationError {
    #[error("Failed to convert numerical value")]
    ConversionError,
}

impl From<EvolutionError> for OptimizationError {
    fn from(_: EvolutionError) -> Self {
        OptimizationError::ConversionError
    }
}

pub struct L1RegularizedLeastSquares {
    lambda: f64,
}

impl L1RegularizedLeastSquares {
    #[verified_engine::verified]
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }

    #[verified_engine::verified]
    pub fn cost(&mut self, a: &DMatrix<f64>, x: &DVector<f64>, y: &DVector<f64>) -> f64 {
        let residual = y - (a * x);
        let l2_term = 0.5 * residual.norm_squared();
        let l1_term = x.iter().map(|v| v.abs()).sum::<f64>();

        l2_term + self.lambda * l1_term
    }
}

/// Strategy for updating parameters.
/// Reimplemented via the Unified EvolutionEngine.
pub trait Optimizer<T: RealField + Copy, Key = u64> {
    fn update_matrix(
        &mut self,
        key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError>;
    fn update_vector(
        &mut self,
        key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError>;
}

pub struct SGD<T> {
    pub learning_rate: T,
}

impl<T: RealField + Copy> SGD<T> {
    #[verified_engine::verified]
    pub fn new(learning_rate: T) -> Self {
        Self { learning_rate }
    }
}

impl<T: RealField + Copy> EvolutionEngine<DMatrix<T>, DMatrix<T>> for SGD<T> {
    fn step<R: RngCore + ?Sized>(
        &mut self,
        state: &mut DMatrix<T>,
        aux: &mut DMatrix<T>,
        _rng: &mut R,
        dt: f64,
    ) -> Result<(), EvolutionError> {
        let lr = T::from_f64(dt).unwrap_or(self.learning_rate);
        *state -= &*aux * lr;
        Ok(())
    }
}

impl<T: RealField + Copy> EvolutionEngine<DVector<T>, DVector<T>> for SGD<T> {
    fn step<R: RngCore + ?Sized>(
        &mut self,
        state: &mut DVector<T>,
        aux: &mut DVector<T>,
        _rng: &mut R,
        dt: f64,
    ) -> Result<(), EvolutionError> {
        let lr = T::from_f64(dt).unwrap_or(self.learning_rate);
        *state -= &*aux * lr;
        Ok(())
    }
}

impl<T: RealField + Copy, Key> Optimizer<T, Key> for SGD<T> {
    #[verified_engine::verified]
    fn update_vector(
        &mut self,
        _key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError> {
        let mut aux = grad.clone();
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        self.step(param, &mut aux, &mut rng, 0.01)
            .map_err(Into::into)
    }

    #[verified_engine::verified]
    fn update_matrix(
        &mut self,
        _key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError> {
        let mut aux = grad.clone();
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        self.step(param, &mut aux, &mut rng, 0.01)
            .map_err(Into::into)
    }
}

pub struct AdamState<T> {
    pub m: DMatrix<T>,
    pub v: DMatrix<T>,
    pub t: i32,
}

pub struct Adam<T, Key>
where
    Key: Eq + std::hash::Hash,
{
    pub learning_rate: T,
    pub beta1: T,
    pub beta2: T,
    pub epsilon: T,
    pub states: HashMap<Key, AdamState<T>>,
}

impl<T: RealField + Copy, Key> Adam<T, Key>
where
    Key: Eq + std::hash::Hash + Clone,
{
    #[verified_engine::verified]
    pub fn new(lr: T) -> Result<Self, OptimizationError> {
        Ok(Self {
            learning_rate: lr,
            beta1: T::from_f64(0.9).ok_or(OptimizationError::ConversionError)?,
            beta2: T::from_f64(0.999).ok_or(OptimizationError::ConversionError)?,
            epsilon: T::from_f64(1e-8).ok_or(OptimizationError::ConversionError)?,
            states: HashMap::new(),
        })
    }

    #[verified_engine::verified]
    pub fn get_state(&mut self, key: Key, shape: (usize, usize)) -> &mut AdamState<T> {
        self.states.entry(key).or_insert_with(|| AdamState {
            m: DMatrix::zeros(shape.0, shape.1),
            v: DMatrix::zeros(shape.0, shape.1),
            t: 0,
        })
    }
}

impl<T: RealField + Copy, Key: Eq + std::hash::Hash>
    EvolutionEngine<DMatrix<T>, (DMatrix<T>, &mut AdamState<T>)> for Adam<T, Key>
{
    fn step<R: RngCore + ?Sized>(
        &mut self,
        state: &mut DMatrix<T>,
        aux: &mut (DMatrix<T>, &mut AdamState<T>),
        _rng: &mut R,
        dt: f64,
    ) -> Result<(), EvolutionError> {
        let grad = &aux.0;
        let adam_state = &mut aux.1;
        let lr = T::from_f64(dt).unwrap_or(self.learning_rate);

        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let one = T::one();

        adam_state.t += 1;
        let t_val = T::from_i32(adam_state.t).ok_or(EvolutionError::ConversionError)?;

        adam_state.m = &adam_state.m * beta1 + grad * (one - beta1);
        let grad_sq = grad.map(|g| g * g);
        adam_state.v = &adam_state.v * beta2 + grad_sq * (one - beta2);

        let m_hat = &adam_state.m / (one - beta1.powf(t_val));
        let v_hat = &adam_state.v / (one - beta2.powf(t_val));

        let update = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));
        *state -= update * lr;
        Ok(())
    }
}

impl<T: RealField + Copy, Key: Eq + std::hash::Hash>
    EvolutionEngine<DVector<T>, (DVector<T>, &mut AdamState<T>)> for Adam<T, Key>
{
    fn step<R: RngCore + ?Sized>(
        &mut self,
        state: &mut DVector<T>,
        aux: &mut (DVector<T>, &mut AdamState<T>),
        _rng: &mut R,
        dt: f64,
    ) -> Result<(), EvolutionError> {
        let grad = &aux.0;
        let adam_state = &mut aux.1;
        let lr = T::from_f64(dt).unwrap_or(self.learning_rate);

        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let one = T::one();

        let rows = state.len();
        let cols = 1;

        adam_state.t += 1;
        let t_val = T::from_i32(adam_state.t).ok_or(EvolutionError::ConversionError)?;

        let grad_mat = DMatrix::from_column_slice(rows, cols, grad.as_slice());

        adam_state.m = &adam_state.m * beta1 + &grad_mat * (one - beta1);
        let grad_sq = grad_mat.map(|g| g * g);
        adam_state.v = &adam_state.v * beta2 + grad_sq * (one - beta2);

        let m_hat = &adam_state.m / (one - beta1.powf(t_val));
        let v_hat = &adam_state.v / (one - beta2.powf(t_val));

        let update_mat = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));
        let update_vec = DVector::from_column_slice(update_mat.as_slice());
        *state -= update_vec * lr;
        Ok(())
    }
}

impl<T: RealField + Copy, Key> Optimizer<T, Key> for Adam<T, Key>
where
    Key: Eq + std::hash::Hash + Clone,
{
    #[verified_engine::verified]
    fn update_matrix(
        &mut self,
        key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError> {
        // extract state first to avoid borrow conflicts
        let shape = (param.nrows(), param.ncols());
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);

        // This is safe because we use the internal state, keeping the interface unchanged
        // while fulfilling the EvolutionEngine pattern.
        // Get the state first:
        let mut state_val = self.states.remove(&key).unwrap_or_else(|| AdamState {
            m: DMatrix::zeros(shape.0, shape.1),
            v: DMatrix::zeros(shape.0, shape.1),
            t: 0,
        });

        let mut aux = (grad.clone(), &mut state_val);
        EvolutionEngine::step(self, param, &mut aux, &mut rng, 0.01)?;

        self.states.insert(key, state_val);
        Ok(())
    }

    #[verified_engine::verified]
    fn update_vector(
        &mut self,
        key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError> {
        let shape = (param.len(), 1);
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);

        let mut state_val = self.states.remove(&key).unwrap_or_else(|| AdamState {
            m: DMatrix::zeros(shape.0, shape.1),
            v: DMatrix::zeros(shape.0, shape.1),
            t: 0,
        });

        let mut aux = (grad.clone(), &mut state_val);
        EvolutionEngine::step(self, param, &mut aux, &mut rng, 0.01)?;

        self.states.insert(key, state_val);
        Ok(())
    }
}
