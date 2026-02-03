use super::metric::Metric;
use super::types::TensorError;
use nalgebra::{DMatrix, DVector};

/// Computes the Christoffel symbols of the second kind $\Gamma^k_{ij}$ at a given point.
/// Returns a vector of matrices, where the k-th matrix contains elements $[i, j]$.
/// Result[k][i, j] = $\Gamma^k_{ij}$.
pub fn christoffel_symbols(
    metric: &impl Metric,
    point: &DVector<f64>,
) -> Result<Vec<DMatrix<f64>>, TensorError> {
    let dim = point.len();
    let h = 1e-5;

    let g_inv = metric.inverse_metric_at(point)?;

    // Precompute metric derivatives: partial_g[k][i, j] = d(g_ij)/dx^k
    let mut partial_g = vec![DMatrix::zeros(dim, dim); dim];

    for k in 0..dim {
        let mut point_plus = point.clone();
        point_plus[k] += h;
        let mut point_minus = point.clone();
        point_minus[k] -= h;

        let g_plus = metric.metric_at(&point_plus)?;
        let g_minus = metric.metric_at(&point_minus)?;

        partial_g[k] = (g_plus - g_minus) / (2.0 * h);
    }

    let mut gammas = vec![DMatrix::zeros(dim, dim); dim];

    for m in 0..dim {
        // Upper index
        for i in 0..dim {
            // Lower index 1
            for j in 0..dim {
                // Lower index 2
                let mut sum = 0.0;
                for k in 0..dim {
                    // Summation index
                    // \Gamma^m_{ij} = 0.5 * g^{mk} * (dg_jk/du^i + dg_ki/du^j - dg_ij/du^k)
                    let term = partial_g[i][(j, k)] + partial_g[j][(k, i)] - partial_g[k][(i, j)];
                    sum += g_inv[(m, k)] * term;
                }
                gammas[m][(i, j)] = 0.5 * sum;
            }
        }
    }

    Ok(gammas)
}
