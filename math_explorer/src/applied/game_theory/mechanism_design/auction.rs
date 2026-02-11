use super::traits::{AuctionMechanism, ValuationDistribution};
use crate::pure_math::analysis::roots::{AnalysisError, Bisection, RootFinder};

/// An Optimal (Myerson) Auction for a single item.
///
/// This mechanism maximizes expected revenue by setting a reserve price $r^*$ such that
/// the virtual valuation $J(r^*) = 0$. The item is awarded to the bidder with the
/// highest virtual valuation, provided it is non-negative.
///
/// # Note on Implementation
/// This implementation assumes the valuation distribution is "regular" (i.e., the virtual valuation
/// function $J(v)$ is strictly increasing). In this case, the optimal auction simplifies to a
/// Second-Price Auction with a reserve price $r^*$.
#[derive(Debug, Clone, Copy)]
pub struct OptimalAuction {
    /// The reserve price below which bids are ignored.
    pub reserve_price: f64,
}

impl OptimalAuction {
    /// Creates a new Optimal Auction by calculating the revenue-maximizing reserve price
    /// for the given distribution.
    ///
    /// # Parameters
    /// - `dist`: The probability distribution of bidder valuations.
    /// - `lower_bound`: The lower bound for the search (e.g., min possible valuation).
    /// - `upper_bound`: The upper bound for the search (e.g., max possible valuation).
    pub fn new<D: ValuationDistribution>(dist: &D, lower_bound: f64, upper_bound: f64) -> Self {
        let reserve_price = Self::calculate_optimal_reserve(dist, lower_bound, upper_bound);
        Self { reserve_price }
    }

    /// Creates an Optimal Auction with a manually specified reserve price.
    pub fn with_reserve(reserve_price: f64) -> Self {
        Self { reserve_price }
    }

    /// Internal logic to find the root of J(r) = 0.
    fn calculate_optimal_reserve<D: ValuationDistribution>(
        dist: &D,
        lower_bound: f64,
        upper_bound: f64,
    ) -> f64 {
        let solver = Bisection::default();
        match solver.find_root(|v| dist.virtual_valuation(v), lower_bound, upper_bound) {
            Ok(root) => root,
            Err(AnalysisError::ConvergenceError(best_guess)) => best_guess,
            Err(AnalysisError::InvalidParameters(_)) => {
                // Fallback for unbracketed roots to match legacy behavior
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

impl AuctionMechanism for OptimalAuction {
    fn determine_allocation(&self, bids: &[f64]) -> Vec<bool> {
        let mut allocation = vec![false; bids.len()];
        let mut max_bid = -1.0; // Valuations are non-negative
        let mut winner_idx = None;

        for (i, &bid) in bids.iter().enumerate() {
            if bid >= self.reserve_price && bid > max_bid {
                max_bid = bid;
                winner_idx = Some(i);
            }
        }

        if let Some(idx) = winner_idx {
            allocation[idx] = true;
        }
        allocation
    }

    fn calculate_payment(&self, bids: &[f64]) -> Vec<f64> {
        let mut payments = vec![0.0; bids.len()];
        let allocation = self.determine_allocation(bids);

        if let Some(winner_idx) = allocation.iter().position(|&x| x) {
            let mut highest_other_bid = 0.0;
            for (i, &bid) in bids.iter().enumerate() {
                if i != winner_idx && bid > highest_other_bid {
                    highest_other_bid = bid;
                }
            }
            // Payment is max(reserve, highest_other_bid)
            payments[winner_idx] = self.reserve_price.max(highest_other_bid);
        }

        payments
    }
}

/// A standard Second-Price (Vickrey) Auction.
///
/// The item is awarded to the highest bidder, who pays the second-highest bid.
/// This mechanism is strategy-proof (dominant strategy to bid one's true value).
#[derive(Debug, Clone, Copy, Default)]
pub struct SecondPriceAuction;

impl AuctionMechanism for SecondPriceAuction {
    fn determine_allocation(&self, bids: &[f64]) -> Vec<bool> {
        let mut allocation = vec![false; bids.len()];
        let mut max_bid = -1.0;
        let mut winner_idx = None;

        for (i, &bid) in bids.iter().enumerate() {
            if bid > max_bid {
                max_bid = bid;
                winner_idx = Some(i);
            }
        }

        if let Some(idx) = winner_idx {
            allocation[idx] = true;
        }
        allocation
    }

    fn calculate_payment(&self, bids: &[f64]) -> Vec<f64> {
        let mut payments = vec![0.0; bids.len()];
        let allocation = self.determine_allocation(bids);

        if let Some(winner_idx) = allocation.iter().position(|&x| x) {
            let mut second_highest = 0.0;
            for (i, &bid) in bids.iter().enumerate() {
                if i != winner_idx && bid > second_highest {
                    second_highest = bid;
                }
            }
            payments[winner_idx] = second_highest;
        }

        payments
    }
}
