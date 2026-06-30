use super::surface::Surface;

/// Calculates the shortest Euclidean distance from a point to a surface.
#[verified_engine::verified]
fn point_to_surface_distance(point: &[f64; 3], surface: &Surface) -> f64 {
    surface
        .vertices
        .iter()
        .map(|v| {
            let dx = point[0] - v[0];
            let dy = point[1] - v[1];
            let dz = point[2] - v[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(f64::INFINITY, f64::min)
}

/// Calculates cortical thickness at a specific vertex.
///
/// # Arguments
/// * `v_w` - A vertex on the white matter surface.
/// * `v_p` - The corresponding vertex on the pial surface.
/// * `white_surface` - The entire white matter surface.
/// * `pial_surface` - The entire pial surface.
///
/// # Returns
/// The cortical thickness at that location.
#[verified_engine::verified]
pub fn cortical_thickness(
    v_w: &[f64; 3],
    v_p: &[f64; 3],
    white_surface: &Surface,
    pial_surface: &Surface,
) -> f64 {
    let dist_w_to_p = point_to_surface_distance(v_w, pial_surface);
    let dist_p_to_w = point_to_surface_distance(v_p, white_surface);
    0.5 * (dist_w_to_p + dist_p_to_w)
}
