use super::coordinates::OrthogonalCoordinateSystem;
use nalgebra::Vector3;

const H: f64 = 1e-5;

/// Computes the gradient of a scalar field $\nabla \Phi$.
/// Returns the components of the gradient in the local basis vectors of the coordinate system.
pub fn gradient<S, F>(coords: &S, field: F, point: &Vector3<f64>) -> Vector3<f64>
where
    S: OrthogonalCoordinateSystem,
    F: Fn(&Vector3<f64>) -> f64,
{
    let factors = coords.scale_factors(point);
    let mut grad = Vector3::zeros();

    for i in 0..3 {
        let mut p_plus = *point;
        p_plus[i] += H;
        let mut p_minus = *point;
        p_minus[i] -= H;

        let df = (field(&p_plus) - field(&p_minus)) / (2.0 * H);
        grad[i] = df / factors[i];
    }

    grad
}

/// Computes the divergence of a vector field $\nabla \cdot \mathbf{A}$.
/// The field `A` should return components in the curvilinear basis.
pub fn divergence<S, F>(coords: &S, field: F, point: &Vector3<f64>) -> f64
where
    S: OrthogonalCoordinateSystem,
    F: Fn(&Vector3<f64>) -> Vector3<f64>,
{
    let factors = coords.scale_factors(point);
    let h1h2h3 = factors[0] * factors[1] * factors[2];

    let mut sum = 0.0;

    // Term 1: d/du1 (h2 h3 A1)
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            let a = field(p);
            h[1] * h[2] * a[0]
        };
        let mut p_plus = *point;
        p_plus[0] += H;
        let mut p_minus = *point;
        p_minus[0] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    // Term 2: d/du2 (h3 h1 A2)
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            let a = field(p);
            h[2] * h[0] * a[1]
        };
        let mut p_plus = *point;
        p_plus[1] += H;
        let mut p_minus = *point;
        p_minus[1] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    // Term 3: d/du3 (h1 h2 A3)
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            let a = field(p);
            h[0] * h[1] * a[2]
        };
        let mut p_plus = *point;
        p_plus[2] += H;
        let mut p_minus = *point;
        p_minus[2] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    if h1h2h3.abs() < 1e-12 {
        0.0 // Avoid division by zero at singularities
    } else {
        sum / h1h2h3
    }
}

/// Computes the curl of a vector field $\nabla \times \mathbf{A}$.
/// Returns the components in the curvilinear basis.
pub fn curl<S, F>(coords: &S, field: F, point: &Vector3<f64>) -> Vector3<f64>
where
    S: OrthogonalCoordinateSystem,
    F: Fn(&Vector3<f64>) -> Vector3<f64>,
{
    let h = coords.scale_factors(point);
    // h1h2h3 cancels out with leading h factors in the determinant expansion
    let mut result = Vector3::zeros();

    // Helper for d/dui (factor * component)
    // We need d/du2 (h3 A3) - d/du3 (h2 A2)

    // Component 1 (factor h1)
    let term1_1 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[2] * field(p)[2];
        let mut p_plus = *point;
        p_plus[1] += H;
        let mut p_minus = *point;
        p_minus[1] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    let term1_2 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[1] * field(p)[1];
        let mut p_plus = *point;
        p_plus[2] += H;
        let mut p_minus = *point;
        p_minus[2] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    result[0] = (term1_1 - term1_2) / (h[1] * h[2]); // Formula: (1/h2h3) * (...) which is (h1 / h1h2h3) * (...)

    // Component 2 (factor h2)
    let term2_1 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[0] * field(p)[0];
        let mut p_plus = *point;
        p_plus[2] += H;
        let mut p_minus = *point;
        p_minus[2] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    let term2_2 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[2] * field(p)[2];
        let mut p_plus = *point;
        p_plus[0] += H;
        let mut p_minus = *point;
        p_minus[0] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    result[1] = (term2_1 - term2_2) / (h[0] * h[2]);

    // Component 3 (factor h3)
    let term3_1 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[1] * field(p)[1];
        let mut p_plus = *point;
        p_plus[0] += H;
        let mut p_minus = *point;
        p_minus[0] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    let term3_2 = {
        let func = |p: &Vector3<f64>| coords.scale_factors(p)[0] * field(p)[0];
        let mut p_plus = *point;
        p_plus[1] += H;
        let mut p_minus = *point;
        p_minus[1] -= H;
        (func(&p_plus) - func(&p_minus)) / (2.0 * H)
    };
    result[2] = (term3_1 - term3_2) / (h[0] * h[1]);

    result
}

/// Computes the Laplacian of a scalar field $\nabla^2 \Phi$.
pub fn laplacian<S, F>(coords: &S, field: F, point: &Vector3<f64>) -> f64
where
    S: OrthogonalCoordinateSystem,
    F: Fn(&Vector3<f64>) -> f64,
{
    let h_factors = coords.scale_factors(point);
    let h1h2h3 = h_factors[0] * h_factors[1] * h_factors[2];

    let mut sum = 0.0;

    // Term 1: d/du1 ( (h2 h3 / h1) dPhi/du1 )
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            // dPhi/du1
            let mut p_plus = *p;
            p_plus[0] += H;
            let mut p_minus = *p;
            p_minus[0] -= H;
            let dphi = (field(&p_plus) - field(&p_minus)) / (2.0 * H);

            (h[1] * h[2] / h[0]) * dphi
        };
        let mut p_plus = *point;
        p_plus[0] += H;
        let mut p_minus = *point;
        p_minus[0] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    // Term 2: d/du2 ( (h3 h1 / h2) dPhi/du2 )
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            let mut p_plus = *p;
            p_plus[1] += H;
            let mut p_minus = *p;
            p_minus[1] -= H;
            let dphi = (field(&p_plus) - field(&p_minus)) / (2.0 * H);

            (h[2] * h[0] / h[1]) * dphi
        };
        let mut p_plus = *point;
        p_plus[1] += H;
        let mut p_minus = *point;
        p_minus[1] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    // Term 3: d/du3 ( (h1 h2 / h3) dPhi/du3 )
    {
        let term_func = |p: &Vector3<f64>| {
            let h = coords.scale_factors(p);
            let mut p_plus = *p;
            p_plus[2] += H;
            let mut p_minus = *p;
            p_minus[2] -= H;
            let dphi = (field(&p_plus) - field(&p_minus)) / (2.0 * H);

            (h[0] * h[1] / h[2]) * dphi
        };
        let mut p_plus = *point;
        p_plus[2] += H;
        let mut p_minus = *point;
        p_minus[2] -= H;
        sum += (term_func(&p_plus) - term_func(&p_minus)) / (2.0 * H);
    }

    if h1h2h3.abs() < 1e-12 {
        0.0
    } else {
        sum / h1h2h3
    }
}
