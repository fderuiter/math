//! Input Data Calibration: CT to Physical Density.

/// Converts Hounsfield Units (HU) to physical density (g/cm³).
///
/// The conversion uses a bi-linear model typical for scanner calibration curves:
/// - **Air to Water range (HU < 0)**: Linearly interpolates between Air (-1000 HU, ~0.0 g/cm³) and Water (0 HU, 1.0 g/cm³).
/// - **Water to Bone range (HU >= 0)**: Linearly interpolates between Water (0 HU, 1.0 g/cm³) and dense bone.
///
/// # Arguments
///
/// * `hu` - The CT number in Hounsfield Units.
///
/// # Returns
///
/// * `Result<f64, String>` - Physical density in g/cm³. Returns error if density would be negative (though simplified logic clamps/handles this).
///
/// # Formula
///
/// - If $HU < 0$: $\rho = 1.0 + (HU / 1000.0)$
/// - If $HU \geq 0$: $\rho = 1.0 + (HU / 500.0)$
pub fn hu_to_density(hu: f64) -> Result<f64, String> {
    let density = if hu < 0.0 {
        1.0 + (hu / 1000.0)
    } else {
        1.0 + (hu / 500.0)
    };

    if density < 0.0 {
        // While physically impossible, extremely low noise artifacts could theoretically cause this in raw data.
        // We clamp to 0.0 for safety but could error out if strict validation is required.
        // The prompt asks for "Constraint: Density cannot be negative".
        // Since -1000 is 0.0, anything below -1000 would be negative.
        // In medical imaging, -1024 is often the minimum (storage), so we treat it as 0 density air.
        Ok(0.0)
    } else {
        Ok(density)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hu_to_density() {
        // Water
        assert!((hu_to_density(0.0).unwrap() - 1.0).abs() < 1e-6);
        // Air
        assert!((hu_to_density(-1000.0).unwrap() - 0.0).abs() < 1e-6);
        // Bone
        // HU = 500 -> rho = 1 + 500/500 = 2.0
        assert!((hu_to_density(500.0).unwrap() - 2.0).abs() < 1e-6);

        // Deep vacuum / noise (should clip to 0)
        assert!((hu_to_density(-1500.0).unwrap() - 0.0).abs() < 1e-6);
    }
}
