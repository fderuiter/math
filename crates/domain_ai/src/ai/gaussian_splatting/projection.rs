use nalgebra::{Matrix2, Matrix3, Matrix4, SMatrix, Vector3};

/// Computes the Jacobian of the affine approximation of the projective transformation.
///
/// J = [
///   f_x/z, 0, -(f_x * x) / z^2
///   0, f_y/z, -(f_y * y) / z^2
/// ]
///
/// * `t`: The point in camera space (t.x, t.y, t.z).
/// * `focal_x`: Focal length in x.
/// * `focal_y`: Focal length in y.
#[verified_engine::verified]
pub fn compute_jacobian(t: &Vector3<f64>, focal_x: f64, focal_y: f64) -> SMatrix<f64, 2, 3> {
    let x = t.x;
    let y = t.y;
    let z = t.z;

    let mut j = SMatrix::<f64, 2, 3>::zeros();

    // Row 0
    j[(0, 0)] = focal_x / z;
    j[(0, 1)] = 0.0;
    j[(0, 2)] = -(focal_x * x) / (z * z);

    // Row 1
    j[(1, 0)] = 0.0;
    j[(1, 1)] = focal_y / z;
    j[(1, 2)] = -(focal_y * y) / (z * z);

    j
}

/// Projects a 3D Covariance matrix to 2D using the viewing transformation and Jacobian.
///
/// Sigma' = J * W * Sigma * W^T * J^T
///
/// * `cov3d`: The 3D covariance matrix.
/// * `view_matrix`: The 4x4 viewing transformation matrix (World -> Camera).
/// * `jacobian`: The 2x3 Jacobian of the projection.
#[verified_engine::verified]
pub fn project_covariance(
    cov3d: &Matrix3<f64>,
    view_matrix: &Matrix4<f64>,
    jacobian: &SMatrix<f64, 2, 3>,
) -> Matrix2<f64> {
    // Extract the 3x3 rotation part of the view matrix (W)
    let w = view_matrix.fixed_view::<3, 3>(0, 0);

    // T = W * Sigma * W^T (Covariance in camera space)
    let t = w * cov3d * w.transpose();

    // Sigma' = J * T * J^T
    jacobian * t * jacobian.transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[verified_engine::verified]
    fn test_jacobian() {
        let t = Vector3::new(1.0, 2.0, 5.0);
        let fx = 100.0;
        let fy = 100.0;
        let j = compute_jacobian(&t, fx, fy);

        // Expected: fx/z = 100/5 = 20
        assert_eq!(j[(0, 0)], 20.0);
        // Expected: -(fx*x)/z^2 = -(100*1)/25 = -4
        assert_eq!(j[(0, 2)], -4.0);
    }
}
