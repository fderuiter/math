//! # Medical Physics: Treatment Planning
//!
//! This module implements core algorithms for Radiation Therapy Treatment Planning (RTTP).
//! It covers four main domains:
//!
//! 1.  **Calibration**: Converting CT imaging data (Hounsfield Units) into physical density for dose calculation.
//! 2.  **Dose Calculation**: Modeling how radiation interacts with matter using convolution/superposition principles.
//! 3.  **Inverse Planning**: Optimizing beam intensities (IMRT/VMAT) to maximize tumor control while sparing healthy tissue.
//! 4.  **Evaluation**: Quantifying plan quality using Dose-Volume Histograms (DVH) and biological modeling (TCP).
//!
//! ## Context
//!
//! In radiation therapy, the goal is to deliver a lethal dose of radiation to a tumor (PTV - Planning Target Volume)
//! while minimizing the dose to surrounding Organs At Risk (OARs). This is an inverse problem where we solve for
//! the optimal beam fluence map that results in the desired 3D dose distribution.
//!
//! ## Units
//!
//! - **Dose**: Gray (Gy), where 1 Gy = 1 J/kg.
//! - **Distance**: Centimeters (cm).
//! - **Density**: g/cm³.
//! - **Hounsfield Units (HU)**: Dimensionless scale where Water = 0, Air = -1000.

/// Input Data Calibration: CT to Physical Density.
pub mod calibration {
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
}

/// Dose Calculation Algorithms.
pub mod dose_calculation {
    /// Calculates the Total Energy Released per Mass (TERMA) for a ray segment.
    ///
    /// TERMA represents the primary energy fluence released into the medium at a point,
    /// before accounting for secondary electron transport (scatter).
    ///
    /// # Arguments
    ///
    /// * `incident_fluence` ($\Psi_0$) - The initial radiant energy fluence.
    /// * `mu` ($\mu$) - The linear attenuation coefficient of the medium (cm⁻¹).
    /// * `depth` ($d$) - The radiological depth along the ray (cm).
    ///
    /// # Returns
    ///
    /// * `f64` - The TERMA value.
    ///
    /// # Formula
    ///
    /// $T = \mu \Psi_0 e^{-\mu d}$
    pub fn calculate_terma(incident_fluence: f64, mu: f64, depth: f64) -> f64 {
        if incident_fluence < 0.0 || mu < 0.0 || depth < 0.0 {
            // Physical quantities should be non-negative, but we return 0.0 or handle gracefully.
            return 0.0;
        }
        mu * incident_fluence * (-mu * depth).exp()
    }

    /// Calculates a simplified analytical Point Spread Function (Kernel).
    ///
    /// This kernel represents the distribution of dose deposited by secondary particles
    /// scattered from a primary interaction point. It describes how TERMA is redistributed into Dose.
    ///
    /// # Arguments
    ///
    /// * `radius` ($r$) - Radial distance from the interaction point (cm).
    /// * `amplitude` ($A$) - Scaling factor proportional to the total energy fraction.
    /// * `beta` ($\beta$) - Decay constant representing the mean free path of secondary particles.
    ///
    /// # Returns
    ///
    /// * `Result<f64, String>` - The kernel value at radius $r$.
    ///
    /// # Formula
    ///
    /// $K(r) = \frac{A}{r^2} e^{-\beta r}$
    ///
    /// *Note*: This is a singular kernel at r=0. In practice, finite voxel size integration is used.
    /// Here we return an error or handle the singularity if r is too close to 0.
    pub fn point_kernel(radius: f64, amplitude: f64, beta: f64) -> Result<f64, String> {
        if radius.abs() < 1e-6 {
            return Err("Radius cannot be zero (singularity at r=0)".to_string());
        }
        if radius < 0.0 {
            return Err("Radius must be non-negative".to_string());
        }

        let val = (amplitude / (radius * radius)) * (-beta * radius).exp();
        Ok(val)
    }
}

/// Inverse Planning Optimization.
pub mod optimization {
    use nalgebra::DVector;

    /// Calculates the quadratic objective function cost.
    ///
    /// The cost function penalizes deviations from the prescription dose in the tumor
    /// and any dose exceeding the limit in organs at risk (OAR).
    ///
    /// # Arguments
    ///
    /// * `current_dose` - Vector of doses for all voxels (Tumor voxels followed by OAR voxels, or managed by indices).
    /// * `tumor_indices` - Indices in `current_dose` corresponding to the tumor.
    /// * `organ_indices` - Indices in `current_dose` corresponding to the OAR.
    /// * `prescription` ($D_{Rx}$) - Desired dose for the tumor.
    /// * `limit` ($D_{lim}$) - Max allowable dose for the organ.
    /// * `alpha` - Weighting factor for tumor under/over-dosage.
    /// * `beta` - Weighting factor for organ over-dosage.
    ///
    /// # Returns
    ///
    /// * `f64` - The total cost.
    pub fn calculate_cost(
        current_dose: &DVector<f64>,
        tumor_indices: &[usize],
        organ_indices: &[usize],
        prescription: f64,
        limit: f64,
        alpha: f64,
        beta: f64,
    ) -> f64 {
        let mut cost = 0.0;

        // Tumor Term: alpha * sum((D_i - D_Rx)^2)
        for &idx in tumor_indices {
            if idx < current_dose.len() {
                let diff = current_dose[idx] - prescription;
                cost += alpha * diff * diff;
            }
        }

        // Organ Term: beta * sum(H(D_i - D_lim) * (D_i - D_lim)^2)
        for &idx in organ_indices {
            if idx < current_dose.len() {
                let dose = current_dose[idx];
                if dose > limit {
                    let diff = dose - limit;
                    cost += beta * diff * diff;
                }
            }
        }

        cost
    }

    /// Performs a single Gradient Descent step to update beamlet weights.
    ///
    /// # Arguments
    ///
    /// * `current_weights` ($w_{old}$) - Current intensity of beamlets.
    /// * `gradient` ($\nabla Cost$) - Gradient of the cost function with respect to weights.
    /// * `eta` ($\eta$) - Learning rate (step size).
    ///
    /// # Returns
    ///
    /// * `DVector<f64>` - Updated weights ($w_{new}$), clipped to be non-negative.
    pub fn update_weights(
        current_weights: &DVector<f64>,
        gradient: &DVector<f64>,
        eta: f64,
    ) -> DVector<f64> {
        let raw_update = current_weights - (gradient * eta);
        // Constraint: Enforce w >= 0 (clip negative weights)
        raw_update.map(|w| if w < 0.0 { 0.0 } else { w })
    }
}

/// Plan Evaluation: DVH and Radiobiology.
pub mod evaluation {
    use std::f64;

    /// Calculates the Cumulative Dose-Volume Histogram (DVH).
    ///
    /// The DVH summarizes the 3D dose distribution into a 2D graph showing how much volume
    /// receives at least a certain dose.
    ///
    /// # Arguments
    ///
    /// * `doses` - A slice of dose values for all voxels in the structure of interest.
    ///
    /// # Returns
    ///
    /// * `Vec<(f64, f64)>` - A sorted vector of (Dose, Normalized Volume) pairs.
    ///   - Dose: The dose bin.
    ///   - Normalized Volume: Fraction of total volume (0.0 to 1.0) receiving >= Dose.
    pub fn calculate_dvh(doses: &[f64]) -> Vec<(f64, f64)> {
        if doses.is_empty() {
            return Vec::new();
        }

        let mut sorted_doses = doses.to_vec();
        // Sort descending to easily compute cumulative count
        sorted_doses.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        let total_voxels = sorted_doses.len() as f64;
        let mut dvh = Vec::new();

        // We can create bins or just return the curve at every point.
        // For a precise curve, we return the step function at every dose value.
        // V(D) is the fraction of voxels with dose >= D.

        for (i, &dose) in sorted_doses.iter().enumerate() {
            // i + 1 is the number of voxels with dose >= sorted_doses[i]
            // because we sorted descending.
            let vol = (i as f64 + 1.0) / total_voxels;
            dvh.push((dose, vol));
        }

        // Reverse back to ascending dose for standard plotting conventions,
        // but the prompt just asks for "pairs representing the Cumulative DVH".
        // Usually DVH is plotted with Dose on X (ascending).
        // If we list (Dose, Volume), and Doses are descending, the vector is ordered by decreasing X.
        // Let's sort by Dose ascending for the return value.
        dvh.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        dvh
    }

    /// Calculates Tumor Control Probability (TCP) using the Poisson Model.
    ///
    /// The Poisson model assumes that the number of surviving clonogens follows a Poisson distribution.
    /// TCP is the probability that zero clonogens survive.
    ///
    /// # Arguments
    ///
    /// * `n0` ($N_0$) - Initial number of clonogenic cells.
    /// * `alpha` ($\alpha$) - Linear radiosensitivity parameter (Gy⁻¹).
    /// * `beta` ($\beta$) - Quadratic radiosensitivity parameter (Gy⁻²).
    /// * `dose_per_fraction` ($d$) - Dose delivered per fraction (Gy).
    /// * `fractions` ($n$) - Number of fractions.
    ///
    /// # Returns
    ///
    /// * `f64` - The probability of tumor control (0.0 to 1.0).
    ///
    /// # Formula
    ///
    /// $TCP = \exp(-N_0 \exp(-\alpha n d - \beta n d^2))$
    pub fn tcp_model(
        n0: f64,
        alpha: f64,
        beta: f64,
        dose_per_fraction: f64,
        fractions: f64,
    ) -> f64 {
        // The exponent inside is: - alpha * n * d - beta * n * d^2
        let exponent =
            -alpha * fractions * dose_per_fraction - beta * fractions * dose_per_fraction.powi(2);
        let surviving_clonogens = n0 * exponent.exp();

        (-surviving_clonogens).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DVector;

    #[test]
    fn test_hu_to_density() {
        // Water
        assert!((calibration::hu_to_density(0.0).unwrap() - 1.0).abs() < 1e-6);
        // Air
        assert!((calibration::hu_to_density(-1000.0).unwrap() - 0.0).abs() < 1e-6);
        // Bone
        // HU = 500 -> rho = 1 + 500/500 = 2.0
        assert!((calibration::hu_to_density(500.0).unwrap() - 2.0).abs() < 1e-6);

        // Deep vacuum / noise (should clip to 0)
        assert!((calibration::hu_to_density(-1500.0).unwrap() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_terma_calculation() {
        // Simple case: no attenuation (mu=0) -> T = 0
        assert_eq!(dose_calculation::calculate_terma(100.0, 0.0, 10.0), 0.0);

        // d=0 -> T = mu * Psi0
        let t0 = dose_calculation::calculate_terma(100.0, 0.1, 0.0);
        assert!((t0 - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_point_kernel() {
        // Error on r=0
        assert!(dose_calculation::point_kernel(0.0, 1.0, 1.0).is_err());

        // Check calculation
        let r = 2.0;
        let a = 4.0;
        let b = 0.5;
        // K = (4 / 4) * exp(-0.5 * 2) = 1 * e^-1 = 0.367879
        let k = dose_calculation::point_kernel(r, a, b).unwrap();
        assert!((k - (-1.0_f64).exp()).abs() < 1e-5);
    }

    #[test]
    fn test_cost_function_logic() {
        // Tumor: 2 voxels, indices 0, 1. Prescription 50.
        // Organ: 2 voxels, indices 2, 3. Limit 20.
        // Doses: [50, 40, 10, 30]
        // Tumor cost: (50-50)^2 + (40-50)^2 = 0 + 100 = 100
        // Organ cost: (10 < 20 -> 0) + (30 > 20 -> (30-20)^2 = 100)
        // Total if alpha=1, beta=1 -> 200

        let doses = DVector::from_vec(vec![50.0, 40.0, 10.0, 30.0]);
        let tumor_idx = vec![0, 1];
        let organ_idx = vec![2, 3];

        let cost = optimization::calculate_cost(
            &doses, &tumor_idx, &organ_idx,
            50.0, 20.0, 1.0, 1.0
        );

        assert!((cost - 200.0).abs() < 1e-6);

        // Verify organ below limit contributes 0
        // Change organ doses to [10, 15] (both < 20)
        let doses_safe = DVector::from_vec(vec![50.0, 50.0, 10.0, 15.0]);
        let cost_safe = optimization::calculate_cost(
            &doses_safe, &tumor_idx, &organ_idx,
            50.0, 20.0, 1.0, 1.0
        );
        assert_eq!(cost_safe, 0.0);
    }

    #[test]
    fn test_gradient_update() {
        let w_old = DVector::from_vec(vec![10.0, 5.0]);
        let grad = DVector::from_vec(vec![2.0, -1.0]);
        let eta = 1.0;

        // w_new = [10, 5] - 1.0 * [2, -1] = [8, 6]
        let w_new = optimization::update_weights(&w_old, &grad, eta);
        assert!((w_new[0] - 8.0).abs() < 1e-6);
        assert!((w_new[1] - 6.0).abs() < 1e-6);

        // Test clipping
        let w_old_clip = DVector::from_vec(vec![1.0]);
        let grad_clip = DVector::from_vec(vec![2.0]); // Update -> 1 - 2 = -1
        let w_new_clip = optimization::update_weights(&w_old_clip, &grad_clip, 1.0);
        assert_eq!(w_new_clip[0], 0.0);
    }

    #[test]
    fn test_tcp_behavior() {
        let n0 = 1e6;
        let alpha = 0.3;
        let beta = 0.03;
        let fractions = 30.0;

        let d_low = 1.0;
        let d_high = 3.0;

        let tcp_low = evaluation::tcp_model(n0, alpha, beta, d_low, fractions);
        let tcp_high = evaluation::tcp_model(n0, alpha, beta, d_high, fractions);

        // Higher dose should result in higher TCP (less survival)
        assert!(tcp_high > tcp_low);
    }

    #[test]
    fn test_dvh_calculation() {
        let doses = vec![10.0, 20.0, 30.0, 40.0];
        let dvh = evaluation::calculate_dvh(&doses);

        // Doses are distinct.
        // Sorted desc: 40, 30, 20, 10
        // 40: vol = 1/4 = 0.25
        // 30: vol = 2/4 = 0.50
        // 20: vol = 3/4 = 0.75
        // 10: vol = 4/4 = 1.00

        // Output is sorted by dose asc: (10, 1.0), (20, 0.75), (30, 0.5), (40, 0.25)
        assert_eq!(dvh.len(), 4);
        assert_eq!(dvh[0], (10.0, 1.0));
        assert_eq!(dvh[3], (40.0, 0.25));
    }
}
