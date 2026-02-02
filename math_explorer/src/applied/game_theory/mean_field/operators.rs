use nalgebra::DMatrix;

/// Computes the central difference first derivative at index (i, n).
///
/// $$ \frac{\partial f}{\partial x} \approx \frac{f(x+\Delta x) - f(x-\Delta x)}{2 \Delta x} $$
///
/// # Panics
/// Panics if `i` is 0 or `i >= nrows - 1`.
#[inline(always)]
pub fn central_grad(data: &DMatrix<f64>, i: usize, n: usize, dx: f64) -> f64 {
    (data[(i + 1, n)] - data[(i - 1, n)]) / (2.0 * dx)
}

/// Computes the central difference second derivative (Laplacian) at index (i, n).
///
/// $$ \frac{\partial^2 f}{\partial x^2} \approx \frac{f(x+\Delta x) - 2f(x) + f(x-\Delta x)}{\Delta x^2} $$
///
/// # Panics
/// Panics if `i` is 0 or `i >= nrows - 1`.
#[inline(always)]
pub fn central_laplacian(data: &DMatrix<f64>, i: usize, n: usize, dx: f64) -> f64 {
    (data[(i + 1, n)] - 2.0 * data[(i, n)] + data[(i - 1, n)]) / (dx * dx)
}

/// Computes the upwind divergence for the Fokker-Planck drift term.
///
/// $$ \nabla \cdot (m v) $$
///
/// Uses upwind scheme based on velocity sign to ensure stability.
#[inline(always)]
pub fn upwind_divergence(m: &DMatrix<f64>, v: f64, i: usize, n: usize, dx: f64) -> f64 {
    if v > 0.0 {
        (m[(i, n)] * v - m[(i - 1, n)] * v) / dx
    } else {
        (m[(i + 1, n)] * v - m[(i, n)] * v) / dx
    }
}
