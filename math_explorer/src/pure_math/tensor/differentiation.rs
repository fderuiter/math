use super::christoffel::christoffel_symbols;
use super::metric::Metric;
use super::types::{ContravariantVector, CovariantVector, Rank2Tensor, TensorError};
use nalgebra::{DMatrix, DVector};

/// Computes the covariant derivative of a contravariant vector field $A^i_{;j}$.
/// Returns a tensor where element (i, j) corresponds to the derivative.
pub fn covariant_derivative_contravariant<F>(
    field: F,
    metric: &impl Metric,
    point: &DVector<f64>,
) -> Result<Rank2Tensor, TensorError>
where
    F: Fn(&DVector<f64>) -> ContravariantVector,
{
    let dim = point.len();
    let h = 1e-5;
    let gammas = christoffel_symbols(metric, point)?;
    let vec_at_point = field(point);

    // Compute partial derivatives \partial A^i / \partial u^j
    let mut partial_derivatives = DMatrix::zeros(dim, dim);
    for j in 0..dim {
        let mut point_plus = point.clone();
        point_plus[j] += h;
        let mut point_minus = point.clone();
        point_minus[j] -= h;

        let vec_plus = field(&point_plus);
        let vec_minus = field(&point_minus);

        let derivative_vec = (vec_plus.0 - vec_minus.0) / (2.0 * h);
        for i in 0..dim {
            partial_derivatives[(i, j)] = derivative_vec[i];
        }
    }

    // Add Christoffel term: + \Gamma^i_{kj} A^k
    let mut result = DMatrix::zeros(dim, dim);
    for i in 0..dim {
        for j in 0..dim {
            let mut gamma_term = 0.0;
            for k in 0..dim {
                // Formula: \Gamma^i_{kj}. m=i, lower1=k, lower2=j.
                // So access gammas[i][(k, j)].
                let gamma_val = gammas[i][(k, j)];
                gamma_term += gamma_val * vec_at_point.0[k];
            }

            result[(i, j)] = partial_derivatives[(i, j)] + gamma_term;
        }
    }

    Ok(Rank2Tensor(result))
}

/// Computes the covariant derivative of a covariant vector field $A_{i;j}$.
/// Returns a tensor where element (i, j) corresponds to the derivative.
pub fn covariant_derivative_covariant<F>(
    field: F,
    metric: &impl Metric,
    point: &DVector<f64>,
) -> Result<Rank2Tensor, TensorError>
where
    F: Fn(&DVector<f64>) -> CovariantVector,
{
    let dim = point.len();
    let h = 1e-5;
    let gammas = christoffel_symbols(metric, point)?;
    let vec_at_point = field(point);

    // Compute partial derivatives \partial A_i / \partial u^j
    let mut partial_derivatives = DMatrix::zeros(dim, dim);
    for j in 0..dim {
        let mut point_plus = point.clone();
        point_plus[j] += h;
        let mut point_minus = point.clone();
        point_minus[j] -= h;

        let vec_plus = field(&point_plus);
        let vec_minus = field(&point_minus);

        let derivative_vec = (vec_plus.0 - vec_minus.0) / (2.0 * h);
        for i in 0..dim {
            partial_derivatives[(i, j)] = derivative_vec[i];
        }
    }

    // Subtract Christoffel term: - \Gamma^k_{ij} A_k
    let mut result = DMatrix::zeros(dim, dim);
    for i in 0..dim {
        for j in 0..dim {
            let mut gamma_term = 0.0;
            for (k, gamma_k) in gammas.iter().enumerate().take(dim) {
                // Formula: \Gamma^k_{ij}. Upper=k, Lower1=i, Lower2=j.
                // Access gammas[k][(i, j)].
                let gamma_val = gamma_k[(i, j)];
                gamma_term += gamma_val * vec_at_point.0[k];
            }

            result[(i, j)] = partial_derivatives[(i, j)] - gamma_term;
        }
    }

    Ok(Rank2Tensor(result))
}
