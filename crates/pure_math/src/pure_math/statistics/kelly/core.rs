//! Core types for Kelly Criterion.

use crate::error::KellyError;

/// Win probability p (must be between 0 and 1).
///
/// Represents the probability of winning a bet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeProbability(f64);

impl EdgeProbability {
    /// Creates a new edge probability.
    ///
    /// # Arguments
    ///
    /// * `value` - The probability value (must be between 0 and 1)
    ///
    /// # Returns
    ///
    /// * `Result<EdgeProbability, KellyError>` - The validated probability or an error
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::EdgeProbability;
    ///
    /// let p = EdgeProbability::new(0.55).unwrap();
    /// assert_eq!(p.value(), 0.55);
    /// ```
    pub fn new(value: f64) -> Result<Self, KellyError> {
        if !(0.0..=1.0).contains(&value) || !value.is_finite() {
            return Err(KellyError::InvalidProbability { value });
        }
        Ok(Self(value))
    }

    /// Returns the raw probability value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns the complementary probability (q = 1 - p).
    pub fn complement(&self) -> f64 {
        1.0 - self.0
    }
}

/// Decimal odds b (must be > 1.0).
///
/// Represents the odds offered. For example:
/// - Odds of 2.0 means you win $1 for every $1 bet (even money)
/// - Odds of 3.0 means you win $2 for every $1 bet (2:1)
///
/// Note: This uses decimal odds format. To convert from:
/// - American odds: Use `Odds::from_american()`
/// - Fractional odds: Use `Odds::from_fractional()`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Odds(f64);

impl Odds {
    /// Creates new decimal odds.
    ///
    /// # Arguments
    ///
    /// * `value` - The decimal odds (must be > 1.0)
    ///
    /// # Returns
    ///
    /// * `Result<Odds, KellyError>` - The validated odds or an error
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::Odds;
    ///
    /// let odds = Odds::new(2.5).unwrap();
    /// assert_eq!(odds.value(), 2.5);
    /// ```
    pub fn new(value: f64) -> Result<Self, KellyError> {
        if value <= 1.0 || !value.is_finite() {
            return Err(KellyError::InvalidOdds { value });
        }
        Ok(Self(value))
    }

    /// Returns the raw odds value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// The net profit multiplier (b - 1).
    ///
    /// This is the amount won per unit bet, excluding the returned stake.
    /// For decimal odds of 2.5, the net profit multiplier is 1.5.
    pub fn net_profit_multiplier(&self) -> f64 {
        self.0 - 1.0
    }

    /// Creates odds from American format.
    ///
    /// American odds use positive numbers for underdogs and negative for favorites:
    /// - Positive (e.g., +200): win $200 on $100 bet → decimal = (200/100) + 1 = 3.0
    /// - Negative (e.g., -150): bet $150 to win $100 → decimal = (100/150) + 1 ≈ 1.67
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::Odds;
    ///
    /// let underdog = Odds::from_american(200.0).unwrap(); // +200
    /// assert!((underdog.value() - 3.0).abs() < 0.01);
    ///
    /// let favorite = Odds::from_american(-150.0).unwrap(); // -150
    /// assert!((favorite.value() - 1.667).abs() < 0.01);
    /// ```
    pub fn from_american(american: f64) -> Result<Self, KellyError> {
        if !american.is_finite() || american == 0.0 || (-100.0..100.0).contains(&american) {
            return Err(KellyError::InvalidOdds { value: american });
        }

        let decimal = if american > 0.0 {
            (american / 100.0) + 1.0
        } else {
            (100.0 / american.abs()) + 1.0
        };

        Self::new(decimal)
    }

    /// Creates odds from fractional format.
    ///
    /// Fractional odds represent the profit relative to stake (e.g., 5/2 or "5 to 2").
    /// To convert to decimal: (numerator / denominator) + 1
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::Odds;
    ///
    /// let odds = Odds::from_fractional(5.0, 2.0).unwrap(); // 5/2
    /// assert!((odds.value() - 3.5).abs() < 0.01);
    /// ```
    pub fn from_fractional(numerator: f64, denominator: f64) -> Result<Self, KellyError> {
        if denominator <= 0.0
            || numerator < 0.0
            || !numerator.is_finite()
            || !denominator.is_finite()
        {
            return Err(KellyError::InvalidOdds {
                value: numerator / denominator,
            });
        }

        Self::new((numerator / denominator) + 1.0)
    }

    /// Converts to implied probability (the bookmaker's edge).
    ///
    /// Implied probability = 1 / decimal_odds
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::Odds;
    ///
    /// let odds = Odds::new(2.0).unwrap();
    /// assert!((odds.implied_probability() - 0.5).abs() < 0.01);
    /// ```
    pub fn implied_probability(&self) -> f64 {
        1.0 / self.0
    }
}

/// Bankroll fraction to bet (must be between 0 and 1).
///
/// Represents what fraction of the total bankroll to wager.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BankrollFraction(f64);

impl BankrollFraction {
    /// Creates a new bankroll fraction.
    ///
    /// # Arguments
    ///
    /// * `value` - The fraction value (must be between 0 and 1)
    ///
    /// # Returns
    ///
    /// * `Result<BankrollFraction, KellyError>` - The validated fraction or an error
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::BankrollFraction;
    ///
    /// let fraction = BankrollFraction::new(0.1).unwrap();
    /// assert_eq!(fraction.value(), 0.1);
    /// ```
    pub fn new(value: f64) -> Result<Self, KellyError> {
        if !(0.0..=1.0).contains(&value) || !value.is_finite() {
            return Err(KellyError::InvalidFraction { value });
        }
        Ok(Self(value))
    }

    /// Returns the raw fraction value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Computes the bet amount given a total bankroll.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::pure_math::statistics::kelly::BankrollFraction;
    ///
    /// let fraction = BankrollFraction::new(0.1).unwrap();
    /// let bet = fraction.bet_amount(1000.0).unwrap();
    /// assert_eq!(bet, 100.0);
    /// ```
    pub fn bet_amount(&self, bankroll: f64) -> Result<f64, KellyError> {
        if bankroll <= 0.0 || !bankroll.is_finite() {
            return Err(KellyError::InvalidBankroll { value: bankroll });
        }
        Ok(self.0 * bankroll)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_probability_valid() {
        let p = EdgeProbability::new(0.55).unwrap();
        assert_eq!(p.value(), 0.55);
        assert!((p.complement() - 0.45).abs() < 1e-10);
    }

    #[test]
    fn test_edge_probability_invalid() {
        assert!(EdgeProbability::new(-0.1).is_err());
        assert!(EdgeProbability::new(1.1).is_err());
        assert!(EdgeProbability::new(f64::NAN).is_err());
    }

    #[test]
    fn test_edge_probability_boundaries() {
        assert!(EdgeProbability::new(0.0).is_ok());
        assert!(EdgeProbability::new(1.0).is_ok());
    }

    #[test]
    fn test_odds_valid() {
        let odds = Odds::new(2.5).unwrap();
        assert_eq!(odds.value(), 2.5);
        assert_eq!(odds.net_profit_multiplier(), 1.5);
    }

    #[test]
    fn test_odds_invalid() {
        assert!(Odds::new(1.0).is_err()); // Must be > 1.0
        assert!(Odds::new(0.5).is_err());
        assert!(Odds::new(f64::NAN).is_err());
    }

    #[test]
    fn test_odds_from_american() {
        // Underdog (+200)
        let odds = Odds::from_american(200.0).unwrap();
        assert!((odds.value() - 3.0).abs() < 0.01);

        // Favorite (-150)
        let odds = Odds::from_american(-150.0).unwrap();
        assert!((odds.value() - 1.667).abs() < 0.01);
    }

    #[test]
    fn test_odds_from_fractional() {
        let odds = Odds::from_fractional(5.0, 2.0).unwrap(); // 5/2
        assert!((odds.value() - 3.5).abs() < 0.01);

        let odds = Odds::from_fractional(1.0, 1.0).unwrap(); // Even money
        assert!((odds.value() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_odds_implied_probability() {
        let odds = Odds::new(2.0).unwrap();
        assert!((odds.implied_probability() - 0.5).abs() < 1e-6);

        let odds = Odds::new(4.0).unwrap();
        assert!((odds.implied_probability() - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_bankroll_fraction_valid() {
        let fraction = BankrollFraction::new(0.1).unwrap();
        assert_eq!(fraction.value(), 0.1);
    }

    #[test]
    fn test_bankroll_fraction_invalid() {
        assert!(BankrollFraction::new(-0.1).is_err());
        assert!(BankrollFraction::new(1.1).is_err());
        assert!(BankrollFraction::new(f64::NAN).is_err());
    }

    #[test]
    fn test_bankroll_fraction_bet_amount() {
        let fraction = BankrollFraction::new(0.1).unwrap();
        let bet = fraction.bet_amount(1000.0).unwrap();
        assert_eq!(bet, 100.0);
    }

    #[test]
    fn test_bankroll_fraction_boundaries() {
        assert!(BankrollFraction::new(0.0).is_ok());
        assert!(BankrollFraction::new(1.0).is_ok());
    }
}
