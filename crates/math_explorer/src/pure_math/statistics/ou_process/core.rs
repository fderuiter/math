//! Core types for Ornstein-Uhlenbeck process.

use super::error::OuError;

/// The mean reversion rate θ (theta).
///
/// Controls the speed at which the process returns to the long-term mean.
/// - High θ: Fast mean reversion (momentum is short-lived)
/// - Low θ: Slow mean reversion (momentum is "sticky")
///
/// Must be strictly positive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeanReversionRate(f64);

impl MeanReversionRate {
    /// Creates a new mean reversion rate.
    ///
    /// # Arguments
    ///
    /// * `value` - The rate value (must be positive)
    ///
    /// # Returns
    ///
    /// * `Result<MeanReversionRate, OuError>` - The validated rate or an error
    ///
    /// # Errors
    ///
    /// Returns `OuError::InvalidMeanReversionRate` if the value is zero, negative, or non-finite.
    ///
    /// # Example
    ///
    /// ```
    /// use math_explorer::pure_math::statistics::ou_process::MeanReversionRate;
    ///
    /// let theta = MeanReversionRate::new(0.5).unwrap();
    /// assert_eq!(theta.value(), 0.5);
    /// ```
    pub fn new(value: f64) -> Result<Self, OuError> {
        if value <= 0.0 || !value.is_finite() {
            return Err(OuError::InvalidMeanReversionRate { value });
        }
        Ok(Self(value))
    }

    /// Returns the raw rate value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The volatility parameter σ (sigma).
///
/// Controls the magnitude of random fluctuations in the process.
/// - High σ: High volatility (large random swings)
/// - Low σ: Low volatility (smooth, predictable behavior)
///
/// Must be non-negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Volatility(f64);

impl Volatility {
    /// Creates a new volatility parameter.
    ///
    /// # Arguments
    ///
    /// * `value` - The volatility value (must be non-negative)
    ///
    /// # Returns
    ///
    /// * `Result<Volatility, OuError>` - The validated parameter or an error
    ///
    /// # Errors
    ///
    /// Returns `OuError::InvalidVolatility` if the value is negative or non-finite.
    ///
    /// # Example
    ///
    /// ```
    /// use math_explorer::pure_math::statistics::ou_process::Volatility;
    ///
    /// let sigma = Volatility::new(0.2).unwrap();
    /// assert_eq!(sigma.value(), 0.2);
    /// ```
    pub fn new(value: f64) -> Result<Self, OuError> {
        if value < 0.0 || !value.is_finite() {
            return Err(OuError::InvalidVolatility { value });
        }
        Ok(Self(value))
    }

    /// Returns the raw volatility value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The long-term mean μ (mu).
///
/// The equilibrium level to which the process reverts over time.
/// In sports analytics, this represents the player's "true skill" or baseline performance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LongTermMean(f64);

impl LongTermMean {
    /// Creates a new long-term mean.
    ///
    /// # Arguments
    ///
    /// * `value` - The mean value (any finite value)
    ///
    /// # Returns
    ///
    /// * `Result<LongTermMean, OuError>` - The validated mean or an error
    ///
    /// # Errors
    ///
    /// Returns `OuError::InvalidSimulationParams` if the value is non-finite.
    ///
    /// # Example
    ///
    /// ```
    /// use math_explorer::pure_math::statistics::ou_process::LongTermMean;
    ///
    /// let mu = LongTermMean::new(0.45).unwrap();
    /// assert_eq!(mu.value(), 0.45);
    /// ```
    pub fn new(value: f64) -> Result<Self, OuError> {
        if !value.is_finite() {
            return Err(OuError::InvalidSimulationParams {
                reason: "Long-term mean must be finite".to_string(),
            });
        }
        Ok(Self(value))
    }

    /// Returns the raw mean value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Parameters for an Ornstein-Uhlenbeck process.
///
/// This struct encapsulates all the parameters needed to define an OU process:
/// - μ (mu): Long-term mean
/// - θ (theta): Mean reversion rate
/// - σ (sigma): Volatility
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OuParams {
    /// The long-term mean μ.
    pub mu: LongTermMean,
    /// The mean reversion rate θ.
    pub theta: MeanReversionRate,
    /// The volatility σ.
    pub sigma: Volatility,
}

impl OuParams {
    /// Creates new OU parameters.
    ///
    /// # Arguments
    ///
    /// * `mu` - The long-term mean
    /// * `theta` - The mean reversion rate
    /// * `sigma` - The volatility
    ///
    /// # Example
    ///
    /// ```
    /// use math_explorer::pure_math::statistics::ou_process::{
    ///     OuParams, LongTermMean, MeanReversionRate, Volatility
    /// };
    ///
    /// let params = OuParams::new(
    ///     LongTermMean::new(0.5).unwrap(),
    ///     MeanReversionRate::new(1.0).unwrap(),
    ///     Volatility::new(0.3).unwrap()
    /// );
    /// ```
    pub fn new(mu: LongTermMean, theta: MeanReversionRate, sigma: Volatility) -> Self {
        Self { mu, theta, sigma }
    }

    /// Creates new OU parameters from raw f64 values with validation.
    ///
    /// # Arguments
    ///
    /// * `mu` - The long-term mean (any finite value)
    /// * `theta` - The mean reversion rate (must be positive)
    /// * `sigma` - The volatility (must be non-negative)
    ///
    /// # Returns
    ///
    /// * `Result<OuParams, OuError>` - The validated parameters or an error
    ///
    /// # Errors
    ///
    /// Returns an `OuError` if any parameter is invalid (e.g., negative volatility).
    pub fn from_values(mu: f64, theta: f64, sigma: f64) -> Result<Self, OuError> {
        Ok(Self {
            mu: LongTermMean::new(mu)?,
            theta: MeanReversionRate::new(theta)?,
            sigma: Volatility::new(sigma)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_reversion_rate_valid() {
        let theta = MeanReversionRate::new(0.5).unwrap();
        assert_eq!(theta.value(), 0.5);
    }

    #[test]
    fn test_mean_reversion_rate_invalid() {
        assert!(MeanReversionRate::new(-0.1).is_err());
        assert!(MeanReversionRate::new(0.0).is_err());
        assert!(MeanReversionRate::new(f64::NAN).is_err());
        assert!(MeanReversionRate::new(f64::INFINITY).is_err());
    }

    #[test]
    fn test_volatility_valid() {
        let sigma = Volatility::new(0.2).unwrap();
        assert_eq!(sigma.value(), 0.2);

        let sigma_zero = Volatility::new(0.0).unwrap();
        assert_eq!(sigma_zero.value(), 0.0);
    }

    #[test]
    fn test_volatility_invalid() {
        assert!(Volatility::new(-0.1).is_err());
        assert!(Volatility::new(f64::NAN).is_err());
    }

    #[test]
    fn test_long_term_mean_valid() {
        let mu = LongTermMean::new(0.5).unwrap();
        assert_eq!(mu.value(), 0.5);

        let mu_negative = LongTermMean::new(-1.5).unwrap();
        assert_eq!(mu_negative.value(), -1.5);
    }

    #[test]
    fn test_long_term_mean_invalid() {
        assert!(LongTermMean::new(f64::NAN).is_err());
        assert!(LongTermMean::new(f64::INFINITY).is_err());
    }

    #[test]
    fn test_ou_params() {
        let params = OuParams::from_values(0.5, 1.0, 0.3).unwrap();
        assert_eq!(params.mu.value(), 0.5);
        assert_eq!(params.theta.value(), 1.0);
        assert_eq!(params.sigma.value(), 0.3);
    }

    #[test]
    fn test_ou_params_invalid() {
        // Invalid theta (negative)
        assert!(OuParams::from_values(0.5, -1.0, 0.3).is_err());

        // Invalid sigma (negative)
        assert!(OuParams::from_values(0.5, 1.0, -0.3).is_err());

        // Invalid mu (NaN)
        assert!(OuParams::from_values(f64::NAN, 1.0, 0.3).is_err());
    }
}
