use super::structs::Gaussian2D;
use nalgebra::Vector3;
use nalgebra::Matrix2;

/// Computes the accumulated color for a pixel using alpha blending.
///
/// C = sum(c_i * alpha_i * T_i)
/// where T_i = prod(1 - alpha_j) for j < i
pub fn blend_gaussians(
    sorted_gaussians: &[Gaussian2D],
    pixel_coord: &nalgebra::Point2<f64>
) -> Vector3<f64> {
    let mut c = Vector3::zeros();
    let mut transmittance = 1.0;

    for g in sorted_gaussians {
        let alpha = evaluate_gaussian_opacity(g, pixel_coord);
        // Alpha blending contribution
        // contribution = alpha * transmittance
        // color += c_i * contribution
        // transmittance *= (1 - alpha)

        let contribution = alpha * transmittance;
        c += g.color * contribution;
        transmittance *= 1.0 - alpha;

        // Optimization: Stop if transmittance is negligible
        if transmittance < 0.0001 {
            break;
        }
    }

    c
}

/// Evaluates the opacity of a 2D Gaussian at a specific point.
///
/// alpha = alpha_raw * exp(-0.5 * (x - mu)^T * Sigma^-1 * (x - mu))
pub fn evaluate_gaussian_opacity(
    gaussian: &Gaussian2D,
    point: &nalgebra::Point2<f64>
) -> f64 {
    let diff = point - gaussian.mean;

    // Inverse covariance
    let det = gaussian.covariance.determinant();
    if det.abs() < 1e-6 {
        return 0.0;
    }
    let inv_cov = gaussian.covariance.try_inverse().unwrap_or_else(Matrix2::identity);

    // Power = -0.5 * d^T * Sigma^-1 * d
    let power = -0.5 * (diff.transpose() * inv_cov * diff)[(0, 0)];

    if power > 0.0 {
        return 0.0; // Should not happen for positive definite covariance unless logic error
    }

    gaussian.opacity * power.exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix2, Point2};

    #[test]
    fn test_evaluate_gaussian() {
        let g = Gaussian2D {
            mean: Point2::new(0.0, 0.0),
            covariance: Matrix2::identity(),
            opacity: 1.0,
            color: Vector3::new(1.0, 0.0, 0.0),
            depth: 1.0,
        };

        // At mean, exp(0) = 1, so returns opacity * 1 = 1.0
        let opacity = evaluate_gaussian_opacity(&g, &Point2::new(0.0, 0.0));
        assert!((opacity - 1.0).abs() < 1e-6);

        // At 1 sigma away (x=1), exp(-0.5 * 1 * 1 * 1) = exp(-0.5) = 0.6065
        let opacity_sigma = evaluate_gaussian_opacity(&g, &Point2::new(1.0, 0.0));
        assert!((opacity_sigma - (-0.5f64).exp()).abs() < 1e-6);
    }

    #[test]
    fn test_blend_gaussians() {
        let g1 = Gaussian2D {
            mean: Point2::new(0.0, 0.0),
            covariance: Matrix2::identity(),
            opacity: 0.5,
            color: Vector3::new(1.0, 0.0, 0.0), // Red
            depth: 1.0,
        };
        let g2 = Gaussian2D {
            mean: Point2::new(0.0, 0.0),
            covariance: Matrix2::identity(),
            opacity: 0.5,
            color: Vector3::new(0.0, 1.0, 0.0), // Green
            depth: 2.0,
        };

        // At mean (0,0), alpha = opacity * 1 = 0.5 for both.
        // C = g1.c * 0.5 * 1.0 + g2.c * 0.5 * (1 - 0.5)
        //   = (1,0,0) * 0.5 + (0,1,0) * 0.25
        //   = (0.5, 0.25, 0.0)

        let gaussians = vec![g1, g2];
        let color = blend_gaussians(&gaussians, &Point2::new(0.0, 0.0));

        assert!((color.x - 0.5).abs() < 1e-6);
        assert!((color.y - 0.25).abs() < 1e-6);
        assert!((color.z - 0.0).abs() < 1e-6);
    }
}
