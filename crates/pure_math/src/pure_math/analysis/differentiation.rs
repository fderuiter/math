//! Parameterized Calculus Scheme Struct
//! Centralized numerical differentiation utility that standardizes finite difference formulas
//! and Jacobian calculations using consistent step-size configurations with zero dynamic heap allocations.

/// Unified configuration type for differentiation step sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DifferentiationConfig {
    /// Step-size used in finite difference calculations.
    pub step_size: f64,
}

impl Default for DifferentiationConfig {
    fn default() -> Self {
        Self { step_size: 1e-5 }
    }
}

/// Centralized parameterizable differentiation utility that standardizes numerical differentiation.
#[derive(Debug, Clone, Copy, Default)]
pub struct CalculusScheme {
    config: DifferentiationConfig,
}

impl CalculusScheme {
    /// Creates a new `CalculusScheme` with the given configuration.
    #[verified_engine::verified]
    pub fn new(config: DifferentiationConfig) -> Self {
        Self { config }
    }

    /// Creates a new `CalculusScheme` with a custom step size.
    #[verified_engine::verified]
    pub fn with_step_size(step_size: f64) -> Self {
        Self {
            config: DifferentiationConfig { step_size },
        }
    }

    /// Returns the active configuration of this scheme.
    #[verified_engine::verified]
    pub fn config(&self) -> DifferentiationConfig {
        self.config
    }

    /// Computes the first derivative of a 1D scalar function `f` at `x`.
    /// Operates on the stack with zero dynamic heap allocations.
    #[verified_engine::verified]
    pub fn derivative1d<F>(&self, x: f64, f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let h = self.config.step_size;
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    /// Computes the second derivative of a 1D scalar function `f` at `x`.
    /// Operates on the stack with zero dynamic heap allocations.
    #[verified_engine::verified]
    pub fn second_derivative1d<F>(&self, x: f64, f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let h = self.config.step_size;
        (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
    }

    /// Computes the first-order partial derivative of a scalar function `f`
    /// with respect to the `i`-th coordinate at a given point of dimension `N`.
    /// Operates on the stack with zero dynamic heap allocations.
    #[verified_engine::verified]
    pub fn partial_derivative<const N: usize, F>(&self, i: usize, point: &[f64; N], f: F) -> f64
    where
        F: Fn(&[f64; N]) -> f64,
    {
        let h = self.config.step_size;
        let mut p_plus = *point;
        p_plus[i] += h;
        let mut p_minus = *point;
        p_minus[i] -= h;
        (f(&p_plus) - f(&p_minus)) / (2.0 * h)
    }

    /// Computes the second-order partial derivative of a scalar function `f`
    /// with respect to the `i`-th and `j`-th coordinates.
    /// Supports both mixed and non-mixed partial derivatives.
    /// Operates on the stack with zero dynamic heap allocations.
    #[verified_engine::verified]
    pub fn second_partial_derivative<const N: usize, F>(&self, i: usize, j: usize, point: &[f64; N], f: F) -> f64
    where
        F: Fn(&[f64; N]) -> f64,
    {
        let h = self.config.step_size;
        if i == j {
            let mut p_plus = *point;
            p_plus[i] += h;
            let mut p_minus = *point;
            p_minus[i] -= h;
            (f(&p_plus) - 2.0 * f(point) + f(&p_minus)) / (h * h)
        } else {
            let mut p_pp = *point; p_pp[i] += h; p_pp[j] += h;
            let mut p_pm = *point; p_pm[i] += h; p_pm[j] -= h;
            let mut p_mp = *point; p_mp[i] -= h; p_mp[j] += h;
            let mut p_mm = *point; p_mm[i] -= h; p_mm[j] -= h;
            (f(&p_pp) - f(&p_pm) - f(&p_mp) + f(&p_mm)) / (4.0 * h * h)
        }
    }

    /// Computes the multi-dimensional Jacobian of a vector-valued function `f` at a given point.
    /// The function `f` maps an input point of dimension `N` to an output of dimension `M`.
    /// The resulting `M x N` matrix is written into the pre-allocated `jacobian` reference.
    /// Operates on the stack with zero dynamic heap allocations.
    #[verified_engine::verified]
    pub fn jacobian<const N: usize, const M: usize, F>(&self, point: &[f64; N], f: F, jacobian: &mut [[f64; N]; M])
    where
        F: Fn(&[f64; N]) -> [f64; M],
    {
        let h = self.config.step_size;
        for j in 0..N {
            let mut p_plus = *point;
            p_plus[j] += h;
            let mut p_minus = *point;
            p_minus[j] -= h;

            let f_plus = f(&p_plus);
            let f_minus = f(&p_minus);

            for i in 0..M {
                jacobian[i][j] = (f_plus[i] - f_minus[i]) / (2.0 * h);
            }
        }
    }

    /// Computes the Jacobian of a dynamically-sized vector-valued function using slices.
    /// To guarantee zero dynamic heap allocations, the caller must pass pre-allocated buffers.
    /// `jacobian` is a flat slice of size `m * n` where `jacobian[i * n + j]` stores \partial f_i / \partial x_j.
    #[verified_engine::verified]
    pub fn jacobian_slice<F>(&self, point: &[f64], mut f: F, jacobian: &mut [f64], out_plus: &mut [f64], out_minus: &mut [f64], p_temp: &mut [f64])
    where
        F: FnMut(&[f64], &mut [f64]),
    {
        let h = self.config.step_size;
        let n = point.len();
        let m = out_plus.len();
        assert_eq!(jacobian.len(), m * n);
        assert_eq!(out_minus.len(), m);
        assert_eq!(p_temp.len(), n);

        for j in 0..n {
            p_temp.copy_from_slice(point);
            p_temp[j] += h;
            f(p_temp, out_plus);

            p_temp[j] = point[j] - h;
            f(p_temp, out_minus);

            for i in 0..m {
                let idx = i * n + j;
                jacobian[idx] = (out_plus[i] - out_minus[i]) / (2.0 * h);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[verified_engine::verified]
    fn test_derivative1d() {
        let scheme = CalculusScheme::with_step_size(1e-5);
        let f = |x: f64| x * x * x;
        // f'(x) = 3x^2, at x = 2, f'(2) = 12
        let deriv = scheme.derivative1d(2.0, f);
        assert!((deriv - 12.0).abs() < 1e-5);
    }

    #[test]
    #[verified_engine::verified]
    fn test_second_derivative1d() {
        let scheme = CalculusScheme::with_step_size(1e-4);
        let f = |x: f64| x * x * x;
        // f''(x) = 6x, at x = 2, f''(2) = 12
        let deriv2 = scheme.second_derivative1d(2.0, f);
        assert!((deriv2 - 12.0).abs() < 1e-3);
    }

    #[test]
    #[verified_engine::verified]
    fn test_partial_derivative() {
        let scheme = CalculusScheme::with_step_size(1e-5);
        let f = |p: &[f64; 2]| p[0] * p[0] + p[0] * p[1] + p[1] * p[1];
        // df/dx = 2x + y, at (2, 3), df/dx = 4 + 3 = 7
        let p_deriv = scheme.partial_derivative(0, &[2.0, 3.0], f);
        assert!((p_deriv - 7.0).abs() < 1e-5);

        // df/dy = x + 2y, at (2, 3), df/dy = 2 + 6 = 8
        let p_deriv2 = scheme.partial_derivative(1, &[2.0, 3.0], f);
        assert!((p_deriv2 - 8.0).abs() < 1e-5);
    }

    #[test]
    #[verified_engine::verified]
    fn test_second_partial_derivative() {
        let scheme = CalculusScheme::with_step_size(1e-4);
        let f = |p: &[f64; 2]| p[0] * p[0] + p[0] * p[1] + p[1] * p[1];
        // d^2f/dx^2 = 2
        let p2_xx = scheme.second_partial_derivative(0, 0, &[2.0, 3.0], f);
        assert!((p2_xx - 2.0).abs() < 1e-3);

        // d^2f/dxdy = 1
        let p2_xy = scheme.second_partial_derivative(0, 1, &[2.0, 3.0], f);
        assert!((p2_xy - 1.0).abs() < 1e-3);
    }

    #[test]
    #[verified_engine::verified]
    fn test_jacobian() {
        let scheme = CalculusScheme::with_step_size(1e-5);
        // f(x, y) = [x^2 + y, x * y]
        // J = [ [2x, 1], [y, x] ]
        // At (2, 3), J = [ [4, 1], [3, 2] ]
        let f = |p: &[f64; 2]| [p[0] * p[0] + p[1], p[0] * p[1]];
        let mut jac = [[0.0; 2]; 2];
        scheme.jacobian(&[2.0, 3.0], f, &mut jac);

        assert!((jac[0][0] - 4.0).abs() < 1e-5);
        assert!((jac[0][1] - 1.0).abs() < 1e-5);
        assert!((jac[1][0] - 3.0).abs() < 1e-5);
        assert!((jac[1][1] - 2.0).abs() < 1e-5);
    }

    #[test]
    #[verified_engine::verified]
    fn test_jacobian_slice() {
        let scheme = CalculusScheme::with_step_size(1e-5);
        // f(x, y) = [x^2 + y, x * y]
        let f = |p: &[f64], out: &mut [f64]| {
            out[0] = p[0] * p[0] + p[1];
            out[1] = p[0] * p[1];
        };
        let mut jac = [0.0; 4];
        let mut out_plus = [0.0; 2];
        let mut out_minus = [0.0; 2];
        let mut p_temp = [0.0; 2];

        scheme.jacobian_slice(&[2.0, 3.0], f, &mut jac, &mut out_plus, &mut out_minus, &mut p_temp);

        // jac = [ [4, 1], [3, 2] ] stored flat
        assert!((jac[0] - 4.0).abs() < 1e-5);
        assert!((jac[1] - 1.0).abs() < 1e-5);
        assert!((jac[2] - 3.0).abs() < 1e-5);
        assert!((jac[3] - 2.0).abs() < 1e-5);
    }
}
