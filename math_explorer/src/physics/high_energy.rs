//! High Energy Physics module.
//!
//! This module implements key concepts in high-energy astrophysics including:
//! - Special Relativity and Four-Vectors
//! - Radiative Processes (Synchrotron, Inverse Compton)
//! - Relativistic Fluid Dynamics
//! - General Relativity (Schwarzschild metric)
//! - Statistics (Li & Ma significance)

// Use nalgebra for vector operations if needed, though strictly typed fields are used for FourVector.
#[allow(unused_imports)]
use nalgebra::Vector3;

// --- Physical Constants ---

/// Speed of light in vacuum (m/s).
pub const C: f64 = 299_792_458.0;

/// Gravitational constant (m^3 kg^-1 s^-2).
pub const G: f64 = 6.674_30e-11;

/// Solar Mass (kg).
pub const SOLAR_MASS: f64 = 1.989e30;

/// Thomson Cross Section (m^2).
/// Value derived from approx 6.6524e-25 cm^2.
pub const SIGMA_T: f64 = 6.6524e-29;

/// Small epsilon for floating point comparisons.
const EPSILON: f64 = 1e-10;

// --- 1. Special Relativity & Four-Vectors ---

pub mod observer {
    use super::*;

    /// A struct representing a Four-Vector (Time + 3-Space).
    /// The metric signature is -+++.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct FourVector {
        /// Time component (e.g., t or ct, depending on context).
        /// For `invariant_interval`, this is interpreted as time `t` (seconds).
        /// For `is_valid_momentum`, this is interpreted as energy component `E/c`.
        pub t: f64,
        /// Spatial X component.
        pub x: f64,
        /// Spatial Y component.
        pub y: f64,
        /// Spatial Z component.
        pub z: f64,
    }

    impl FourVector {
        /// Creates a new FourVector.
        pub fn new(t: f64, x: f64, y: f64, z: f64) -> Self {
            Self { t, x, y, z }
        }

        /// Calculates the spacetime interval s^2.
        /// Formula: s^2 = -(ct)^2 + x^2 + y^2 + z^2.
        /// This assumes `self.t` is time in seconds.
        pub fn invariant_interval(&self) -> Result<f64, String> {
            let ct = C * self.t;
            Ok(-(ct * ct) + self.x * self.x + self.y * self.y + self.z * self.z)
        }

        /// Validates 4-Momentum Invariance.
        /// Checks if p^mu p_mu = -(mc)^2.
        ///
        /// Assumes the vector represents 4-momentum p^mu = (E/c, px, py, pz).
        /// Note: The input `mass` is the rest mass `m`.
        pub fn is_valid_momentum(&self, mass: f64) -> Result<bool, String> {
            if mass < 0.0 {
                return Err("Mass cannot be negative".to_string());
            }
            // Contraction p^mu p_mu = - (p^0)^2 + (p^1)^2 + ...
            // Here self.t is p^0 (E/c).
            let p_sq = -(self.t * self.t) + self.x * self.x + self.y * self.y + self.z * self.z;
            let target = -(mass * C).powi(2);

            // Using relative error for robustness with large numbers, or absolute for small.
            let diff = (p_sq - target).abs();
            let tolerance = 1e-5; // Tolerance for floating point comparison

            if target.abs() > EPSILON {
                Ok(diff / target.abs() < tolerance)
            } else {
                // Mass is effectively 0 (photon). p_sq should be 0.
                Ok(diff < tolerance)
            }
        }
    }

    /// Calculates the Lorentz Factor gamma.
    /// Formula: gamma = 1 / sqrt(1 - beta^2) where beta = v/c.
    pub fn calculate_lorentz_factor(v: f64) -> Result<f64, String> {
        let beta = v / C;
        if beta.abs() >= 1.0 {
            return Err("Velocity must be strictly less than c for massive particles".to_string());
        }
        let gamma = 1.0 / (1.0 - beta * beta).sqrt();
        Ok(gamma)
    }
}

// --- 2. Radiative Processes ---

pub mod radiation {
    use super::*;

    /// Calculates the total synchrotron power radiated by a single electron.
    /// Formula: P = (4/3) * sigma_T * c * beta^2 * gamma^2 * U_B
    ///
    /// # Arguments
    /// * `u_b` - Magnetic energy density.
    /// * `beta` - Electron velocity / c.
    /// * `gamma` - Lorentz factor.
    pub fn synchrotron_power(u_b: f64, beta: f64, gamma: f64) -> Result<f64, String> {
        if u_b < 0.0 {
            return Err("Energy density cannot be negative".to_string());
        }
        if gamma < 1.0 {
            return Err("Lorentz factor must be >= 1".to_string());
        }

        // P = 4/3 sigma_T c beta^2 gamma^2 U_B
        let power = (4.0 / 3.0) * SIGMA_T * C * beta.powi(2) * gamma.powi(2) * u_b;
        Ok(power)
    }

    /// Calculates the observed spectral index alpha from an electron power-law distribution p.
    /// Formula: alpha = (p - 1) / 2
    pub fn inverse_compton_spectral_index(p: f64) -> Result<f64, String> {
        if p <= 1.0 {
            return Err("Power law index p must be > 1 for convergence".to_string());
        }
        Ok((p - 1.0) / 2.0)
    }
}

// --- 3. Relativistic Fluid Dynamics ---

pub mod fluid_dynamics {
    use super::*;

    /// Calculates the density compression ratio r for a strong relativistic shock.
    /// Formula: r = (gamma_hat + 1) / (gamma_hat - 1)
    ///
    /// # Arguments
    /// * `adiabatic_index` - The adiabatic index (gamma_hat).
    pub fn shock_compression_ratio(adiabatic_index: f64) -> Result<f64, String> {
        if adiabatic_index <= 1.0 {
            return Err("Adiabatic index must be > 1".to_string());
        }
        let r = (adiabatic_index + 1.0) / (adiabatic_index - 1.0);
        Ok(r)
    }

    /// Calculates the specific enthalpy h.
    /// Formula: h = 1 + (gamma_hat / (gamma_hat - 1)) * (P / (rho * c^2))
    pub fn specific_enthalpy(adiabatic_index: f64, pressure: f64, density: f64) -> Result<f64, String> {
        if adiabatic_index <= 1.0 {
            return Err("Adiabatic index must be > 1".to_string());
        }
        if density <= 0.0 {
            return Err("Density must be positive".to_string());
        }
        if pressure < 0.0 {
            return Err("Pressure cannot be negative".to_string());
        }

        let term = (adiabatic_index / (adiabatic_index - 1.0)) * (pressure / (density * C.powi(2)));
        Ok(1.0 + term)
    }
}

// --- 4. General Relativity (Schwarzschild) ---

/// Struct representing a Schwarzschild Black Hole.
pub struct SchwarzschildBlackHole {
    pub mass: f64,
}

impl SchwarzschildBlackHole {
    /// Creates a new SchwarzschildBlackHole.
    pub fn new(mass: f64) -> Result<Self, String> {
        if mass <= 0.0 {
            return Err("Mass must be positive".to_string());
        }
        Ok(Self { mass })
    }

    /// Calculates the Schwarzschild Radius.
    /// Formula: R_s = 2GM / c^2
    pub fn schwarzschild_radius(&self) -> Result<f64, String> {
        Ok((2.0 * G * self.mass) / C.powi(2))
    }

    /// Calculates the gravitational time dilation factor dt/dtau at radius r.
    /// Note: The prompt formula is dtau/dt = sqrt(1 - Rs/r).
    /// The prompt says "Calculates the time dilation factor... Formula: dtau/dt = ...".
    /// Usually "time dilation factor" refers to dt/dtau (gamma), which is > 1.
    /// But the formula provided is for dtau/dt (which is < 1, slowing down).
    /// I will implement the formula exactly as provided: d_tau / d_t.
    pub fn gravitational_time_dilation(&self, r: f64) -> Result<f64, String> {
        let rs = self.schwarzschild_radius()?;
        if r <= rs {
            return Err("Radius must be greater than Schwarzschild radius".to_string());
        }
        Ok((1.0 - rs / r).sqrt())
    }

    /// Calculates the Innermost Stable Circular Orbit (ISCO).
    /// Formula: R_ISCO = 6GM / c^2
    pub fn isco(&self) -> Result<f64, String> {
        Ok((6.0 * G * self.mass) / C.powi(2))
    }
}

// --- 5. Statistics (Li & Ma) ---

pub mod statistics {
    /// Calculates the Li & Ma Significance (sigma).
    ///
    /// # Arguments
    /// * `n_on` - Counts on source.
    /// * `n_off` - Background counts.
    /// * `alpha` - Ratio of exposure times (t_on / t_off).
    pub fn li_ma_significance(n_on: f64, n_off: f64, alpha: f64) -> Result<f64, String> {
        if n_on < 0.0 || n_off < 0.0 || alpha <= 0.0 {
            return Err("Counts must be non-negative and alpha positive".to_string());
        }

        let term1 = if n_on > 0.0 {
            let ratio = (1.0 + alpha) / alpha * (n_on / (n_on + n_off));
            n_on * ratio.ln()
        } else {
            0.0
        };

        let term2 = if n_off > 0.0 {
            let ratio = (1.0 + alpha) * (n_off / (n_on + n_off));
            n_off * ratio.ln()
        } else {
            0.0
        };

        let sum = term1 + term2;
        if sum < 0.0 {
            // Should not happen for valid inputs where n_on/n_off reflect an excess,
            // but numerically possible or if deficit.
            // Formula has sqrt.
             return Err("Negative argument for sqrt in Li & Ma".to_string());
        }

        Ok(2.0f64.sqrt() * sum.sqrt())
    }
}

// --- 6. Verification ---

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_special_relativity() {
        // Lorentz Factor: v = 0.6c => gamma = 1.25
        let v = 0.6 * C;
        let gamma = observer::calculate_lorentz_factor(v).unwrap();
        assert_relative_eq!(gamma, 1.25, epsilon = 1e-6);

        // Invariant Interval: ct=3, x=4, y=0, z=0 => s^2 = -9 + 16 = 7
        // t = 3/c.
        let fv = observer::FourVector::new(3.0 / C, 4.0, 0.0, 0.0);
        let s2 = fv.invariant_interval().unwrap();
        assert_relative_eq!(s2, 7.0, epsilon = 1e-6);

        // Momentum Invariance
        // m=1. p^mu = (gamma mc, gamma mv, 0, 0) / c?
        // p^0 = E/c = gamma m c.
        // p^1 = gamma m v.
        // p.p = -(gamma m c)^2 + (gamma m v)^2 = gamma^2 m^2 (v^2 - c^2) = gamma^2 m^2 (-c^2/gamma^2) = -m^2 c^2.
        // Let's use m=1, v=0.6c, gamma=1.25.
        // p^0 = 1.25 * 1 * C.
        // p^1 = 1.25 * 1 * 0.6 * C = 0.75 * C.
        let mass = 1.0;
        // Wait, FourVector.t is just the value.
        // If p^mu = (E/c, px, py, pz).
        // E/c = 1.25 * C.
        // px = 0.75 * C.
        let p_vec = observer::FourVector::new(1.25 * C, 0.75 * C, 0.0, 0.0);
        assert!(p_vec.is_valid_momentum(mass).unwrap());
    }

    #[test]
    fn test_radiation() {
        // Synchrotron
        // beta -> 1, gamma = 2. U_B = 1.
        // P = 4/3 sigma_T c * 1 * 4 * 1
        let u_b = 1.0;
        let gamma = 2.0;
        let beta = (1.0 - 1.0/4.0f64).sqrt(); // consistent beta
        let p = radiation::synchrotron_power(u_b, beta, gamma).unwrap();
        let expected = (4.0/3.0) * SIGMA_T * C * beta * beta * 4.0;
        assert_relative_eq!(p, expected, epsilon = 1e-6);

        // Inverse Compton
        // p = 3 => alpha = (3-1)/2 = 1.
        let alpha = radiation::inverse_compton_spectral_index(3.0).unwrap();
        assert_relative_eq!(alpha, 1.0);
    }

    #[test]
    fn test_fluid_dynamics() {
        // Compression ratio, gamma_hat = 4/3.
        // r = (4/3 + 1) / (4/3 - 1) = (7/3) / (1/3) = 7.
        let r = fluid_dynamics::shock_compression_ratio(4.0/3.0).unwrap();
        assert_relative_eq!(r, 7.0);
    }

    #[test]
    fn test_schwarzschild() {
        let mass = SOLAR_MASS;
        let bh = SchwarzschildBlackHole::new(mass).unwrap();

        let rs = bh.schwarzschild_radius().unwrap();
        let expected_rs = 2.0 * G * mass / (C * C);
        assert_relative_eq!(rs, expected_rs);

        let isco = bh.isco().unwrap();
        assert_relative_eq!(isco, 3.0 * rs);

        // Time dilation at r = 2 Rs.
        // factor = sqrt(1 - 1/2) = sqrt(0.5) approx 0.707
        let factor = bh.gravitational_time_dilation(2.0 * rs).unwrap();
        assert_relative_eq!(factor, 0.5f64.sqrt());
    }

    #[test]
    fn test_statistics_li_ma() {
        // Example: Non=10, Noff=10, alpha=1.
        // term1 = 10 * ln(2 * 10/20) = 10 * ln(1) = 0.
        // term2 = 10 * ln(2 * 10/20) = 0.
        // S = 0.
        let s = statistics::li_ma_significance(10.0, 10.0, 1.0).unwrap();
        assert_relative_eq!(s, 0.0);

        // Example: Non=20, Noff=10, alpha=1.
        // term1: 20 * ln(2 * 20/30) = 20 * ln(4/3) = 20 * 0.28768 = 5.75
        // term2: 10 * ln(2 * 10/30) = 10 * ln(2/3) = 10 * -0.4054 = -4.05
        // sum = 1.70. S = sqrt(2 * 1.70) = sqrt(3.4) ~ 1.84.
        let s2 = statistics::li_ma_significance(20.0, 10.0, 1.0).unwrap();
        assert!(s2 > 1.8 && s2 < 1.9);
    }
}
