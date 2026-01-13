//! Surface Meshing and Noise Reduction.
//!
//! Fits a continuous **Bi-Quadratic Polynomial Surface** to the sparse radar point cloud to smooth out "sparkle" noise.
//!
//! # Model
//! $$ Z(x,y) = c_0 + c_1 x + c_2 y + c_3 x^2 + c_4 y^2 + c_5 xy $$
//!
//! Note: The prompt defines the terms as $c_3 x^2 + c_4 y^2 + c_5 xy$. The implementation follows this coefficient ordering.
//!
//! The coordinate system here assumes $z$ is height/chest amplitude, and $x, y$ are lateral positions.

use nalgebra::{DMatrix, DVector, Point3};

/// A fitted bi-quadratic surface.
#[derive(Debug, Clone)]
pub struct BiQuadraticSurface {
    /// Coefficients $[c_0, c_1, c_2, c_3, c_4, c_5]$.
    ///
    /// - $c_0$: Height offset
    /// - $c_1$: x-slope
    /// - $c_2$: y-slope
    /// - $c_3$: x-curvature
    /// - $c_4$: y-curvature
    /// - $c_5$: Twist (xy)
    pub coefficients: DVector<f64>,
}

impl BiQuadraticSurface {
    /// Fits a bi-quadratic surface to a set of 3D points using Least Squares.
    ///
    /// Solve for $c$ in $A c = b$.
    ///
    /// # Arguments
    /// * `points` - A slice of Cartesian points $(x, y, z)$.
    ///
    /// # Returns
    /// * `Some(BiQuadraticSurface)` if the fit is successful.
    /// * `None` if the system is under-determined (too few points).
    pub fn fit(points: &[Point3<f64>]) -> Option<Self> {
        let n = points.len();
        if n < 6 {
            return None; // Need at least 6 points for 6 coefficients
        }

        // Construct Design Matrix A (n x 6) and Vector b (n x 1)
        // Model: Z = c0 + c1*x + c2*y + c3*x^2 + c4*y^2 + c5*xy
        // Rows of A: [1, x, y, x^2, y^2, xy]

        let mut a_data = Vec::with_capacity(n * 6);
        let mut b_data = Vec::with_capacity(n);

        for p in points {
            a_data.push(1.0);
            a_data.push(p.x);
            a_data.push(p.y);
            a_data.push(p.x.powi(2));
            a_data.push(p.y.powi(2));
            a_data.push(p.x * p.y);

            b_data.push(p.z);
        }

        let a = DMatrix::from_row_slice(n, 6, &a_data);
        let b = DVector::from_column_slice(&b_data);

        // Solve using SVD for numerical stability
        let epsilon = 1e-9;
        match a.svd(true, true).solve(&b, epsilon) {
            Ok(coefficients) => Some(Self { coefficients }),
            Err(_) => None,
        }
    }

    /// Evaluates the surface height $z$ at a given $(x, y)$.
    pub fn evaluate(&self, x: f64, y: f64) -> f64 {
        let c = &self.coefficients;
        c[0]
            + c[1] * x
            + c[2] * y
            + c[3] * x.powi(2)
            + c[4] * y.powi(2)
            + c[5] * x * y
    }

    /// Generates a dense grid of points from the fitted surface.
    pub fn generate_mesh(&self, x_range: (f64, f64), y_range: (f64, f64), resolution: usize) -> Vec<Point3<f64>> {
        let mut mesh = Vec::with_capacity(resolution * resolution);
        let dx = (x_range.1 - x_range.0) / (resolution as f64 - 1.0);
        let dy = (y_range.1 - y_range.0) / (resolution as f64 - 1.0);

        for i in 0..resolution {
            for j in 0..resolution {
                let x = x_range.0 + i as f64 * dx;
                let y = y_range.0 + j as f64 * dy;
                let z = self.evaluate(x, y);
                mesh.push(Point3::new(x, y, z));
            }
        }
        mesh
    }

    /// Calculates the Weighted Average Height (WAH) signal.
    ///
    /// $$ V_{resp}(t) = \frac{1}{\text{Area}} \iint_{ROI} Z_{mesh}(x,y,t) dx dy $$
    ///
    /// In the discrete case, this is the arithmetic mean of Z-coordinates in the ROI.
    ///
    /// # Arguments
    /// * `mesh` - The reconstructed surface points.
    /// * `roi_filter` - A function that returns true if a point is within the Region of Interest.
    pub fn weighted_average_height<F>(mesh: &[Point3<f64>], roi_filter: F) -> Option<f64>
    where
        F: Fn(&Point3<f64>) -> bool,
    {
        let mut sum_z = 0.0;
        let mut count = 0;

        for p in mesh {
            if roi_filter(p) {
                sum_z += p.z;
                count += 1;
            }
        }

        if count == 0 {
            None
        } else {
            Some(sum_z / count as f64)
        }
    }
}
