use rand::distributions::Distribution as RandDistribution;
use rand::Rng;
use statrs::distribution::{Continuous, ContinuousCDF};
use statrs::statistics::Distribution;

/// Represents a distribution of bidder valuations.
///
/// In Mechanism Design, particularly for auction theory, we often need to analyze properties
/// of the distribution of valuations $v$ drawn from a cumulative distribution function $F(v)$
/// with probability density function $f(v)$.
pub trait ValuationDistribution:
    Continuous<f64, f64> + ContinuousCDF<f64, f64> + Distribution<f64> + RandDistribution<f64>
{
    /// Computes the **Virtual Valuation** $J(v)$ according to Myerson's Lemma.
    ///
    /// $$ J(v) = v - \frac{1 - F(v)}{f(v)} $$
    ///
    /// In the context of optimal auction design (revenue maximization), the auctioneer
    /// treats the virtual valuation $J(v)$ as the "real" value they extract from the bidder.
    /// An optimal auction awards the item to the bidder with the highest virtual valuation,
    /// provided it is non-negative.
    ///
    /// The condition $J(v) \geq 0$ defines the optimal reserve price.
    fn virtual_valuation(&self, v: f64) -> f64 {
        let pdf = self.pdf(v);
        let cdf = self.cdf(v);
        if pdf.abs() < 1e-9 {
            // Handle edge case where density is 0 (shouldn't happen in support)
            v
        } else {
            v - (1.0 - cdf) / pdf
        }
    }
}

// Blanket implementation for any continuous distribution from statrs
impl<D: Continuous<f64, f64> + ContinuousCDF<f64, f64> + Distribution<f64> + RandDistribution<f64>>
    ValuationDistribution for D
{
}

/// A generic auction mechanism that can be simulated.
pub trait AuctionMechanism {
    /// Simulates the expected revenue of the auction mechanism via Monte Carlo.
    ///
    /// # Arguments
    /// * `n_bidders` - Number of bidders participating.
    /// * `n_simulations` - Number of Monte Carlo simulations to run.
    /// * `rng` - Random number generator.
    fn simulate_revenue<R: Rng + ?Sized>(
        &self,
        n_bidders: usize,
        n_simulations: usize,
        rng: &mut R,
    ) -> f64;
}
