use super::structs::Gaussian2D;
use nalgebra::Vector3;

/// Computes color for a 2x2 block of pixels.
/// Returns 4 colors in row-major order: (x,y), (x+1,y), (x,y+1), (x+1,y+1).
#[inline]
pub fn blend_gaussians_block_2x2(
    sorted_gaussians: &[Gaussian2D],
    top_left: &nalgebra::Point2<f64>,
    stride_x: f64,
    stride_y: f64,
) -> [Vector3<f64>; 4] {
    let mut c = [Vector3::zeros(); 4];
    let mut t = [1.0; 4];

    // Offsets for the 4 pixels
    let off_x = [0.0, stride_x, 0.0, stride_x];
    let off_y = [0.0, 0.0, stride_y, stride_y];

    for g in sorted_gaussians {
        // Load Gaussian data once to reuse across 4 pixels
        let mx = g.mean.x;
        let my = g.mean.y;
        let a = g.conic[(0, 0)];
        let b = g.conic[(0, 1)];
        let cc = g.conic[(1, 1)];
        let op = g.opacity;

        // Load color components once
        let cr = g.color.x;
        let cg = g.color.y;
        let cb = g.color.z;

        let mut all_opaque = true;

        for k in 0..4 {
            if t[k] >= 0.0001 {
                all_opaque = false;

                let px = top_left.x + off_x[k];
                let py = top_left.y + off_y[k];

                let dx = px - mx;
                let dy = py - my;

                let power = a * dx * dx + 2.0 * b * dx * dy + cc * dy * dy;

                if power <= 0.0 {
                    let alpha = op * power.exp();
                    let contribution = alpha * t[k];

                    c[k].x += cr * contribution;
                    c[k].y += cg * contribution;
                    c[k].z += cb * contribution;

                    t[k] *= 1.0 - alpha;
                }
            }
        }

        if all_opaque {
            break;
        }
    }

    c
}

/// Computes the accumulated color for a pixel using alpha blending.
///
/// C = sum(c_i * alpha_i * T_i)
/// where T_i = prod(1 - alpha_j) for j < i
#[inline]
pub fn blend_gaussians(
    sorted_gaussians: &[Gaussian2D],
    pixel_coord: &nalgebra::Point2<f64>,
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
#[inline]
pub fn evaluate_gaussian_opacity(gaussian: &Gaussian2D, point: &nalgebra::Point2<f64>) -> f64 {
    let dx = point.x - gaussian.mean.x;
    let dy = point.y - gaussian.mean.y;

    // Power = d^T * Conic * d
    // Conic is symmetric: [[a, b], [b, c]] (where conic = -0.5 * Sigma^-1)
    // Result = a*x^2 + 2*b*x*y + c*y^2
    let a = gaussian.conic[(0, 0)];
    let b = gaussian.conic[(0, 1)];
    let c = gaussian.conic[(1, 1)];

    let power = a * dx * dx + 2.0 * b * dx * dy + c * dy * dy;

    if power > 0.0 {
        return 0.0;
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
            conic: Matrix2::from_diagonal_element(-0.5),
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
            conic: Matrix2::from_diagonal_element(-0.5),
            opacity: 0.5,
            color: Vector3::new(1.0, 0.0, 0.0), // Red
            depth: 1.0,
        };
        let g2 = Gaussian2D {
            mean: Point2::new(0.0, 0.0),
            conic: Matrix2::from_diagonal_element(-0.5),
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
