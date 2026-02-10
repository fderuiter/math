use super::traits::{AuctionMechanism, ValuationDistribution};
use crate::pure_math::analysis::roots::{AnalysisError, Bisection, RootFinder};
use rand::Rng;

/// An auction mechanism designed to maximize expected revenue.
///
/// Implements Myerson's Optimal Auction for a single item.
/// It sets a reserve price $r^*$ such that the virtual valuation $J(r^*) = 0$.
pub struct OptimalAuction<D: ValuationDistribution> {
    distribution: D,
    reserve_price: f64,
}

impl<D: ValuationDistribution> OptimalAuction<D> {
    /// Creates a new Optimal Auction for a given valuation distribution.
    ///
    /// Automatically calculates the optimal reserve price within the given bounds.
    pub fn new(distribution: D, lower_bound: f64, upper_bound: f64) -> Self {
        let reserve_price = Self::calculate_optimal_reserve(&distribution, lower_bound, upper_bound);
        Self {
            distribution,
            reserve_price,
        }
    }

    /// Returns the calculated optimal reserve price.
    pub fn reserve_price(&self) -> f64 {
        self.reserve_price
    }

    /// Calculates the optimal reserve price.
    fn calculate_optimal_reserve(dist: &D, lower_bound: f64, upper_bound: f64) -> f64 {
        let solver = Bisection::default();
        match solver.find_root(|v| dist.virtual_valuation(v), lower_bound, upper_bound) {
            Ok(root) => root,
            Err(AnalysisError::ConvergenceError(best_guess)) => best_guess,
            Err(AnalysisError::InvalidParameters(_)) => {
                // Fallback logic
                let j_low = dist.virtual_valuation(lower_bound);
                if j_low > 0.0 {
                    return lower_bound;
                }
                let j_high = dist.virtual_valuation(upper_bound);
                if j_high < 0.0 {
                    return upper_bound;
                }
                (lower_bound + upper_bound) / 2.0
            }
        }
    }
}

impl<D: ValuationDistribution> AuctionMechanism for OptimalAuction<D> {
    fn simulate_revenue<R: Rng + ?Sized>(
        &self,
        n_bidders: usize,
        n_simulations: usize,
        rng: &mut R,
    ) -> f64 {
        let mut total_revenue = 0.0;

        for _ in 0..n_simulations {
            let mut max_virtual_val = 0.0;
            for _ in 0..n_bidders {
                let v = self.distribution.sample(rng);
                let j = self.distribution.virtual_valuation(v);
                if j > max_virtual_val {
                    max_virtual_val = j;
                }
            }
            total_revenue += max_virtual_val;
        }

        total_revenue / (n_simulations as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use statrs::distribution::Uniform;

    #[test]
    fn test_optimal_auction_reserve() {
        let dist = Uniform::new(0.0, 100.0).unwrap();
        let auction = OptimalAuction::new(dist, 0.0, 100.0);
        assert!((auction.reserve_price() - 50.0).abs() < 1e-4);
    }

    #[test]
    fn test_second_price_auction_revenue() {
        // Uniform(0, 1). E[2nd highest of 2] = 1/3.
        let dist = Uniform::new(0.0, 1.0).unwrap();
        let auction = SecondPriceAuction::new(dist, 0.0);
        let mut rng = rand::thread_rng();
        let revenue = auction.simulate_revenue(2, 10_000, &mut rng);
        assert!((revenue - 1.0 / 3.0).abs() < 0.02);
    }
}

/// A standard Second-Price (Vickrey) Auction with a reserve price.
///
/// The item is awarded to the highest bidder at the price of the second-highest bid,
/// or the reserve price if the second-highest bid is lower.
pub struct SecondPriceAuction<D: ValuationDistribution> {
    distribution: D,
    reserve_price: f64,
}

impl<D: ValuationDistribution> SecondPriceAuction<D> {
    /// Creates a new Second-Price Auction with a fixed reserve price.
    pub fn new(distribution: D, reserve_price: f64) -> Self {
        Self {
            distribution,
            reserve_price,
        }
    }
}

impl<D: ValuationDistribution> AuctionMechanism for SecondPriceAuction<D> {
    fn simulate_revenue<R: Rng + ?Sized>(
        &self,
        n_bidders: usize,
        n_simulations: usize,
        rng: &mut R,
    ) -> f64 {
        let mut total_revenue = 0.0;
        let mut bids = Vec::with_capacity(n_bidders);

        for _ in 0..n_simulations {
            bids.clear();
            for _ in 0..n_bidders {
                bids.push(self.distribution.sample(rng));
            }

            // Sort descending
            bids.sort_by(|a, b| b.partial_cmp(a).unwrap());

            let highest_bid = bids[0];
            if highest_bid < self.reserve_price {
                // No sale
                continue;
            }

            let second_highest = if n_bidders > 1 { bids[1] } else { 0.0 };

            // Revenue is max(second_highest, reserve_price)
            let price = if second_highest > self.reserve_price {
                second_highest
            } else {
                self.reserve_price
            };

            total_revenue += price;
        }

        total_revenue / (n_simulations as f64)
    }
}
