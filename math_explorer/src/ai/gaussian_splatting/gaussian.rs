use nalgebra::{Matrix3, UnitQuaternion, Vector3};

/// Represents a single 3D Gaussian for Splatting.
///
/// Each Gaussian acts as a "soft" particle in space, defined by its position,
/// shape (covariance), opacity, and color (spherical harmonics).
#[derive(Debug, Clone)]
pub struct Gaussian3D {
    /// The center of the Gaussian in 3D space (Mean, \mu).
    pub position: Vector3<f64>,

    /// The scaling factors for the covariance ellipsoid.
    /// This forms the diagonal of the scaling matrix S.
    pub scaling: Vector3<f64>,

    /// The orientation of the Gaussian.
    /// Used to construct the rotation matrix R.
    pub rotation: UnitQuaternion<f64>,

    /// A scalar between 0 and 1 determining how transparent the Gaussian is.
    pub opacity: f64,

    /// Spherical Harmonic coefficients for view-dependent color.
    /// Each element is a Vector3 representing RGB coefficients.
    pub sh_coeffs: Vec<Vector3<f64>>,
}

impl Gaussian3D {
    /// Creates a new 3D Gaussian.
    pub fn new(
        position: Vector3<f64>,
        scaling: Vector3<f64>,
        rotation: UnitQuaternion<f64>,
        opacity: f64,
        sh_coeffs: Vec<Vector3<f64>>,
    ) -> Self {
        Self {
            position,
            scaling,
            rotation,
            opacity,
            sh_coeffs,
        }
    }

    /// Computes the 3x3 covariance matrix \Sigma.
    ///
    /// The covariance defines the shape, size, and orientation of the ellipsoid.
    /// \Sigma = R S S^T R^T
    ///
    /// Where:
    /// - R is the rotation matrix derived from the quaternion.
    /// - S is the scaling matrix (diagonal matrix from scaling vector).
    pub fn compute_covariance(&self) -> Matrix3<f64> {
        // Create Scaling Matrix S (diagonal)
        let s_matrix = Matrix3::from_diagonal(&self.scaling);

        // Get Rotation Matrix R
        let r_matrix = self.rotation.to_rotation_matrix();

        // Calculate M = R * S
        // Note: nalgebra's Rotation matrix acts on vectors. Matrix multiplication order matters.
        // We want to transform the axis-aligned scaling ellipsoid by rotation.
        // \Sigma = R * S * S^T * R^T
        // Let M = R * S
        // Then \Sigma = M * M^T

        let m = r_matrix * s_matrix;

        m * m.transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_covariance_identity() {
        // Identity rotation, unit scaling
        let position = Vector3::new(0.0, 0.0, 0.0);
        let scaling = Vector3::new(1.0, 1.0, 1.0);
        let rotation = UnitQuaternion::identity();
        let opacity = 1.0;
        let sh_coeffs = vec![];

        let gaussian = Gaussian3D::new(position, scaling, rotation, opacity, sh_coeffs);
        let cov = gaussian.compute_covariance();

        assert_relative_eq!(cov, Matrix3::identity());
    }

    #[test]
    fn test_covariance_scaling_only() {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let scaling = Vector3::new(2.0, 0.5, 1.0);
        let rotation = UnitQuaternion::identity();
        let opacity = 1.0;
        let sh_coeffs = vec![];

        let gaussian = Gaussian3D::new(position, scaling, rotation, opacity, sh_coeffs);
        let cov = gaussian.compute_covariance();

        // With identity rotation, Sigma = S * S^T = S^2 (since S is diagonal)
        let expected = Matrix3::new(
            4.0, 0.0, 0.0,
            0.0, 0.25, 0.0,
            0.0, 0.0, 1.0
        );

        assert_relative_eq!(cov, expected);
    }

    #[test]
    fn test_covariance_rotation() {
         let position = Vector3::new(0.0, 0.0, 0.0);
        let scaling = Vector3::new(2.0, 1.0, 1.0);

        // Rotate 90 degrees around Z axis.
        // The x-axis (scale 2.0) should become the y-axis.
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), std::f64::consts::FRAC_PI_2);

        let opacity = 1.0;
        let sh_coeffs = vec![];

        let gaussian = Gaussian3D::new(position, scaling, rotation, opacity, sh_coeffs);
        let cov = gaussian.compute_covariance();

        // Original S^2 = Diag(4, 1, 1).
        // Rotated 90 deg Z: X becomes Y, Y becomes -X.
        // So the "spread" of 4 should now be along Y.

        let expected = Matrix3::new(
            1.0, 0.0, 0.0,
            0.0, 4.0, 0.0,
            0.0, 0.0, 1.0
        );

        assert_relative_eq!(cov, expected, epsilon = 1e-10);
    }
}
