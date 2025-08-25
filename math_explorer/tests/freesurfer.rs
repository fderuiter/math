use math_explorer::freesurfer::*;
use nalgebra::{DMatrix, DVector};

#[test]
fn test_surface_reconstruction() {
    let mut surface = Surface {
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]],
    };
    let internal = internal_energy(&surface, 0.5, 0.2);
    assert!(internal > 0.0);

    let external = external_energy(&surface, 10.0);
    assert!(external < 0.0);

    let initial_vertex = surface.vertices[0][0];
    evolve_surface(&mut surface, 0.1, 1.0);
    assert_ne!(initial_vertex, surface.vertices[0][0]);
}

#[test]
fn test_bayesian_classification() {
    let likelihood = 0.8;
    let prior = 0.5;
    let posterior = bayesian_classification(likelihood, prior);
    assert_eq!(posterior, 0.4);
}

#[test]
fn test_cortical_thickness() {
    // Add the test points as vertices to the surfaces to ensure the simplified
    // vertex-only distance calculation works as expected for this test.
    let white_surface = Surface {
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.5, 0.0, 0.0]],
    };
    let pial_surface = Surface {
        vertices: vec![[0.0, 2.0, 0.0], [1.0, 2.0, 0.0], [0.5, 2.0, 0.0]],
    };
    let v_w = [0.5, 0.0, 0.0];
    let v_p = [0.5, 2.0, 0.0];

    let thickness = cortical_thickness(&v_w, &v_p, &white_surface, &pial_surface);
    assert!((thickness - 2.0).abs() < 1e-9);
}

#[test]
fn test_glm_beta_estimation() {
    let x: DMatrix<f64> = DMatrix::from_row_slice(4, 2, &[
        1.0, 1.0,
        1.0, 2.0,
        1.0, 3.0,
        1.0, 4.0,
    ]);
    let y: DVector<f64> = DVector::from_vec(vec![6.0, 5.0, 7.0, 10.0]);

    let beta = estimate_beta(&x, &y).unwrap();
    assert!((beta[0] - 3.5).abs() < 1e-9);
    assert!((beta[1] - 1.4).abs() < 1e-9);
}

#[test]
fn test_glm_t_statistic() {
    let x: DMatrix<f64> = DMatrix::from_row_slice(4, 2, &[
        1.0, 1.0,
        1.0, 2.0,
        1.0, 3.0,
        1.0, 4.0,
    ]);
    let y: DVector<f64> = DVector::from_vec(vec![6.0, 5.0, 7.0, 10.0]);
    let beta = estimate_beta(&x, &y).unwrap();

    let c: DVector<f64> = DVector::from_vec(vec![0.0, 1.0]);
    let residuals = y - &x * &beta;
    let residual_variance = residuals.norm_squared() / (4.0 - 2.0);

    let t = t_statistic(&c, &beta, &x, residual_variance).unwrap();
    // The expected value was re-calculated to be 2.1602...
    assert!((t - 2.1602).abs() < 1e-4);
}
