use super::constants::{C, G};
use super::error::HighEnergyError;

/// Struct representing a Schwarzschild Black Hole.
pub struct SchwarzschildBlackHole {
    pub mass: f64,
}

impl SchwarzschildBlackHole {
    /// Creates a new SchwarzschildBlackHole.
    pub fn new(mass: f64) -> Result<Self, HighEnergyError> {
        if mass <= 0.0 {
            return Err(HighEnergyError::InvalidMass(mass));
        }
        Ok(Self { mass })
    }

    /// Calculates the Schwarzschild Radius.
    /// Formula: R_s = 2GM / c^2
    pub fn schwarzschild_radius(&self) -> Result<f64, HighEnergyError> {
        Ok((2.0 * G * self.mass) / C.powi(2))
    }

    /// Calculates the gravitational time dilation factor dt/dtau at radius r.
    /// Note: The prompt formula is dtau/dt = sqrt(1 - Rs/r).
    /// The prompt says "Calculates the time dilation factor... Formula: dtau/dt = ...".
    /// Usually "time dilation factor" refers to dt/dtau (gamma), which is > 1.
    /// But the formula provided is for dtau/dt (which is < 1, slowing down).
    /// I will implement the formula exactly as provided: d_tau / d_t.
    pub fn gravitational_time_dilation(&self, r: f64) -> Result<f64, HighEnergyError> {
        let rs = self.schwarzschild_radius()?;
        if r <= rs {
            return Err(HighEnergyError::InsideEventHorizon(r, rs));
        }
        Ok((1.0 - rs / r).sqrt())
    }

    /// Calculates the Innermost Stable Circular Orbit (ISCO).
    /// Formula: R_ISCO = 6GM / c^2
    pub fn isco(&self) -> Result<f64, HighEnergyError> {
        Ok((6.0 * G * self.mass) / C.powi(2))
    }
}

#[cfg(test)]
mod tests {
    use super::super::constants::SOLAR_MASS;
    use super::*;
    use approx::assert_relative_eq;

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
}
