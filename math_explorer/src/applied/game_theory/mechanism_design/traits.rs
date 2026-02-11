use rand::distributions::Distribution as RandDistribution;
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

/// Defines the rules for an auction mechanism.
///
/// An auction mechanism consists of an **allocation rule** (who gets the item)
/// and a **payment rule** (how much they pay).
pub trait AuctionMechanism {
    /// Determines how much each bidder pays based on their bids.
    fn calculate_payment(&self, bids: &[f64]) -> Vec<f64>;

    /// Determines who gets the item(s). Returns a boolean vector (true = won).
    fn determine_allocation(&self, bids: &[f64]) -> Vec<bool>;

    /// Estimates the expected revenue of the auction via simulation.
    ///
    /// This default implementation runs a Monte Carlo simulation by sampling valuations
    /// from the provided distribution, simulating the auction, and averaging the total payments.
    fn expected_revenue<D: ValuationDistribution>(
        &self,
        dist: &D,
        n_bidders: usize,
        n_simulations: usize,
    ) -> f64 {
        let mut rng = rand::thread_rng();
        let mut total_revenue = 0.0;

        for _ in 0..n_simulations {
            let bids: Vec<f64> = (0..n_bidders).map(|_| dist.sample(&mut rng)).collect();
            let payments = self.calculate_payment(&bids);
            total_revenue += payments.iter().sum::<f64>();
        }

        total_revenue / (n_simulations as f64)
    }
}
