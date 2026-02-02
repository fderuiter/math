//! Medical Physics Geometry.
//!
//! Geometric calculations for Image Guided Radiation Therapy (IGRT).

/// Calculates Tracking Error for cine EPID or other tracking systems.
///
/// This represents the deviation between the target's position and the radiation field's center.
///
/// $$ E_{track} = C_{target} - C_{field} $$
///
/// # Arguments
///
/// * `target_center` ($C_{target}$) - Position of the target center (e.g., tumor/marker).
/// * `field_centroid` ($C_{field}$) - Position of the field centroid (e.g., MLC aperture center).
///
/// # Returns
///
/// * `f64` - The tracking error (Signed distance). Positive means target is "ahead" of field in the coordinate system.
pub fn tracking_error(target_center: f64, field_centroid: f64) -> f64 {
    target_center - field_centroid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking_error() {
        assert_eq!(tracking_error(10.0, 10.0), 0.0);
        assert_eq!(tracking_error(10.0, 5.0), 5.0);
        assert_eq!(tracking_error(5.0, 10.0), -5.0);
    }
}
