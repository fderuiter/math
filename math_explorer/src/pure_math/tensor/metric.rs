use super::types::{ContravariantVector, CovariantVector, TensorError};
use nalgebra::{DMatrix, DVector};

/// A trait representing a metric tensor field $g_{ij}(x)$.
pub trait Metric {
    /// Computes the covariant metric tensor $g_{ij}$ at a given point.
    fn metric_at(&self, point: &DVector<f64>) -> Result<DMatrix<f64>, TensorError>;

    /// Computes the inverse (contravariant) metric tensor $g^{ij}$ at a given point.
    fn inverse_metric_at(&self, point: &DVector<f64>) -> Result<DMatrix<f64>, TensorError> {
        let g_cov = self.metric_at(point)?;
        g_cov.try_inverse().ok_or(TensorError::SingularMetric)
    }

    /// Lowers the index of a contravariant vector: $A_i = g_{ij} A^j$.
    fn lower_index(
        &self,
        vec: &ContravariantVector,
        point: &DVector<f64>,
    ) -> Result<CovariantVector, TensorError> {
        let g = self.metric_at(point)?;
        if g.nrows() != vec.dim() || g.ncols() != vec.dim() {
            return Err(TensorError::DimensionMismatch {
                expected: g.nrows(),
                got: vec.dim(),
            });
        }
        let lowered = &g * &vec.0;
        Ok(CovariantVector::new(lowered))
    }

    /// Raises the index of a covariant vector: $A^i = g^{ij} A_j$.
    fn raise_index(
        &self,
        vec: &CovariantVector,
        point: &DVector<f64>,
    ) -> Result<ContravariantVector, TensorError> {
        let g_inv = self.inverse_metric_at(point)?;
        if g_inv.nrows() != vec.dim() || g_inv.ncols() != vec.dim() {
            return Err(TensorError::DimensionMismatch {
                expected: g_inv.nrows(),
                got: vec.dim(),
            });
        }
        let raised = &g_inv * &vec.0;
        Ok(ContravariantVector::new(raised))
    }
}

/// A generic implementation of a Riemannian metric defined by a closure.
pub struct RiemannianMetric<F>
where
    F: Fn(&DVector<f64>) -> DMatrix<f64>,
{
    metric_fn: F,
}

impl<F> RiemannianMetric<F>
where
    F: Fn(&DVector<f64>) -> DMatrix<f64>,
{
    pub fn new(metric_fn: F) -> Self {
        Self { metric_fn }
    }
}

impl<F> Metric for RiemannianMetric<F>
where
    F: Fn(&DVector<f64>) -> DMatrix<f64>,
{
    fn metric_at(&self, point: &DVector<f64>) -> Result<DMatrix<f64>, TensorError> {
        Ok((self.metric_fn)(point))
    }
}
