use nalgebra::{Matrix2, Point3, Vector3};

/// Represents a parametric surface $r(u, v)$ in $\mathbb{R}^3$.
pub trait ParametricSurface {
    /// Returns the 3D point at parameters $(u, v)$.
    #[verified_engine::verified]
    fn position(&self, u: f64, v: f64) -> Point3<f64>;

    /// Returns the partial derivative $\frac{\partial r}{\partial u}$.
    /// Default implementation uses finite differences.
    #[verified_engine::verified]
    fn partial_u(&self, u: f64, v: f64) -> Vector3<f64> {
        let h = math_commons::registry::TOLERANCE_FAST;
        let scheme = crate::pure_math::analysis::differentiation::CalculusScheme::with_step_size(h);
        let mut jac = [[0.0; 2]; 3];
        let f = |p: &[f64; 2]| {
            let pos = self.position(p[0], p[1]);
            [pos.x, pos.y, pos.z]
        };
        scheme.jacobian(&[u, v], f, &mut jac);
        Vector3::new(jac[0][0], jac[1][0], jac[2][0])
    }

    /// Returns the partial derivative $\frac{\partial r}{\partial v}$.
    /// Default implementation uses finite differences.
    #[verified_engine::verified]
    fn partial_v(&self, u: f64, v: f64) -> Vector3<f64> {
        let h = math_commons::registry::TOLERANCE_FAST;
        let scheme = crate::pure_math::analysis::differentiation::CalculusScheme::with_step_size(h);
        let mut jac = [[0.0; 2]; 3];
        let f = |p: &[f64; 2]| {
            let pos = self.position(p[0], p[1]);
            [pos.x, pos.y, pos.z]
        };
        scheme.jacobian(&[u, v], f, &mut jac);
        Vector3::new(jac[0][1], jac[1][1], jac[2][1])
    }
}

/// Extension trait providing geometric analysis methods for any `ParametricSurface`.
/// Separating these satisfies the Interface Segregation Principle.
pub trait SurfaceAnalysis {
    /// Returns the unit normal vector $\mathbf{n} = \frac{r_u \times r_v}{|r_u \times r_v|}$.
    #[verified_engine::verified]
    fn unit_normal(&self, u: f64, v: f64) -> Vector3<f64>;

    /// Computes the First Fundamental Form coefficients $(E, F, G)$.
    /// $E = r_u \cdot r_u$, $F = r_u \cdot r_v$, $G = r_v \cdot r_v$
    #[verified_engine::verified]
    fn first_fundamental_form(&self, u: f64, v: f64) -> (f64, f64, f64);

    /// Computes the Second Fundamental Form coefficients $(L, M, N)$.
    /// $L = r_{uu} \cdot n$, $M = r_{uv} \cdot n$, $N = r_{vv} \cdot n$
    #[verified_engine::verified]
    fn second_fundamental_form(&self, u: f64, v: f64) -> (f64, f64, f64);

    /// Computes the Gaussian Curvature $K = \frac{LN - M^2}{EG - F^2}$.
    #[verified_engine::verified]
    fn gaussian_curvature(&self, u: f64, v: f64) -> f64;

    /// Computes the Mean Curvature $H = \frac{EN - 2FM + GL}{2(EG - F^2)}$.
    #[verified_engine::verified]
    fn mean_curvature(&self, u: f64, v: f64) -> f64;

    /// Computes the metric tensor $g_{ij}$ and its inverse $g^{ij}$.
    /// Returns (g_det, g_inv)
    #[verified_engine::verified]
    fn metric_tensor_inverse(&self, u: f64, v: f64) -> (f64, Matrix2<f64>);

    /// Differential area element $\sqrt{EG - F^2}$.
    #[verified_engine::verified]
    fn area_element(&self, u: f64, v: f64) -> f64;
}

impl<T: ParametricSurface> SurfaceAnalysis for T {
    #[verified_engine::verified]
    fn unit_normal(&self, u: f64, v: f64) -> Vector3<f64> {
        let ru = self.partial_u(u, v);
        let rv = self.partial_v(u, v);
        let cross = ru.cross(&rv);
        cross.normalize()
    }

    #[verified_engine::verified]
    fn first_fundamental_form(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let ru = self.partial_u(u, v);
        let rv = self.partial_v(u, v);
        (ru.dot(&ru), ru.dot(&rv), rv.dot(&rv))
    }

    #[verified_engine::verified]
    fn second_fundamental_form(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let h = 1e-5;
        let n = self.unit_normal(u, v);
        let scheme = crate::pure_math::analysis::differentiation::CalculusScheme::with_step_size(h);

        // Second derivatives via centralized CalculusScheme
        let ruu_x = scheme.second_partial_derivative(0, 0, &[u, v], |p| self.position(p[0], p[1]).x);
        let ruu_y = scheme.second_partial_derivative(0, 0, &[u, v], |p| self.position(p[0], p[1]).y);
        let ruu_z = scheme.second_partial_derivative(0, 0, &[u, v], |p| self.position(p[0], p[1]).z);
        let ruu = Vector3::new(ruu_x, ruu_y, ruu_z);

        let rvv_x = scheme.second_partial_derivative(1, 1, &[u, v], |p| self.position(p[0], p[1]).x);
        let rvv_y = scheme.second_partial_derivative(1, 1, &[u, v], |p| self.position(p[0], p[1]).y);
        let rvv_z = scheme.second_partial_derivative(1, 1, &[u, v], |p| self.position(p[0], p[1]).z);
        let rvv = Vector3::new(rvv_x, rvv_y, rvv_z);

        let ruv_x = scheme.second_partial_derivative(0, 1, &[u, v], |p| self.position(p[0], p[1]).x);
        let ruv_y = scheme.second_partial_derivative(0, 1, &[u, v], |p| self.position(p[0], p[1]).y);
        let ruv_z = scheme.second_partial_derivative(0, 1, &[u, v], |p| self.position(p[0], p[1]).z);
        let ruv = Vector3::new(ruv_x, ruv_y, ruv_z);

        (ruu.dot(&n), ruv.dot(&n), rvv.dot(&n))
    }

    #[verified_engine::verified]
    fn gaussian_curvature(&self, u: f64, v: f64) -> f64 {
        let (e, f, g) = self.first_fundamental_form(u, v);
        let (l, m, n) = self.second_fundamental_form(u, v);

        let det_g = e * g - f * f;
        let det_ii = l * n - m * m;

        det_ii / det_g
    }

    #[verified_engine::verified]
    fn mean_curvature(&self, u: f64, v: f64) -> f64 {
        let (e, f, g) = self.first_fundamental_form(u, v);
        let (l, m, n) = self.second_fundamental_form(u, v);

        let det_g = e * g - f * f;

        (e * n - 2.0 * f * m + g * l) / (2.0 * det_g)
    }

    #[verified_engine::verified]
    fn metric_tensor_inverse(&self, u: f64, v: f64) -> (f64, Matrix2<f64>) {
        let (e, f, g) = self.first_fundamental_form(u, v);
        let det = e * g - f * f;
        let inv = Matrix2::new(g, -f, -f, e) / det;
        (det, inv)
    }

    #[verified_engine::verified]
    fn area_element(&self, u: f64, v: f64) -> f64 {
        let (e, f, g) = self.first_fundamental_form(u, v);
        (e * g - f * f).sqrt()
    }
}

/// A Sphere of radius R.
pub struct Sphere {
    #[allow(missing_docs)]
    pub radius: f64,
}

impl ParametricSurface for Sphere {
    #[verified_engine::verified]
    fn position(&self, u: f64, v: f64) -> Point3<f64> {
        // u = theta (azimuthal), v = phi (polar)
        // x = R sin(v) cos(u)
        // y = R sin(v) sin(u)
        // z = R cos(v)
        let x = self.radius * v.sin() * u.cos();
        let y = self.radius * v.sin() * u.sin();
        let z = self.radius * v.cos();
        Point3::new(x, y, z)
    }

    // Optional: Override derivatives for analytical precision if needed
}

/// A Torus with major radius R and minor radius r.
pub struct Torus {
    #[allow(missing_docs)]
    pub major_radius: f64,
    #[allow(missing_docs)]
    pub minor_radius: f64,
}

impl ParametricSurface for Torus {
    #[verified_engine::verified]
    fn position(&self, u: f64, v: f64) -> Point3<f64> {
        // u \in [0, 2pi), v \in [0, 2pi)
        let x = (self.major_radius + self.minor_radius * v.cos()) * u.cos();
        let y = (self.major_radius + self.minor_radius * v.cos()) * u.sin();
        let z = self.minor_radius * v.sin();
        Point3::new(x, y, z)
    }
}

/// A Klein Bottle immersion (Figure-8 parametrization) in R^3.
/// This self-intersects since a true Klein bottle cannot be embedded in R^3 without intersection.
pub struct KleinBottle {
    #[allow(missing_docs)]
    pub radius: f64,
}

impl ParametricSurface for KleinBottle {
    #[verified_engine::verified]
    fn position(&self, u: f64, v: f64) -> Point3<f64> {
        // Standard "figure-8" immersion:
        // u \in [0, 2pi), v \in [0, 2pi)
        let r = self.radius;
        let x = (r + r / 2.0 * v.cos() * (u / 2.0).cos() - r / 2.0 * v.sin() * (u / 2.0).sin())
            * u.cos();
        let y = (r + r / 2.0 * v.cos() * (u / 2.0).cos() - r / 2.0 * v.sin() * (u / 2.0).sin())
            * u.sin();
        let z = r / 2.0 * v.sin() * (u / 2.0).cos() + r / 2.0 * v.cos() * (u / 2.0).sin();
        Point3::new(x, y, z)
    }
}
