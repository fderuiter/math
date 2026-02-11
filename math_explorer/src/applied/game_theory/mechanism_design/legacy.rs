use crate::pure_math::analysis::roots::{AnalysisError, RootFinder};
use rand::Rng;
use super::traits::ValuationDistribution;
use super::auction::OptimalAuction;

/// Mechanism Design utilities.
#[deprecated(since = "0.2.0", note = "Use `OptimalAuction` or `AuctionMechanism` instead.")]
pub struct MechanismDesign;

#[allow(deprecated)]
impl MechanismDesign {
    /// Calculates the **Optimal Reserve Price** for a single-item auction.
    ///
    /// According to Myerson (1981), for a "regular" distribution (where $J(v)$ is strictly increasing),
    /// the revenue-maximizing reserve price $r^*$ is the value such that the virtual valuation is zero:
    ///
    /// $$ J(r^*) = r^* - \frac{1 - F(r^*)}{f(r^*)} = 0 $$
    ///
    /// This function finds the root of $J(r) = 0$ using the bisection method within the given bounds.
    ///
    /// # Parameters
    /// - `dist`: The probability distribution of bidder valuations.
    /// - `lower_bound`: The lower bound for the search (e.g., min possible valuation).
    /// - `upper_bound`: The upper bound for the search (e.g., max possible valuation).
    pub fn optimal_reserve_price<D: ValuationDistribution>(
        dist: &D,
        lower_bound: f64,
        upper_bound: f64,
    ) -> f64 {
        // Delegate to the new OptimalAuction implementation
        let auction = OptimalAuction::new(dist, lower_bound, upper_bound);
        auction.reserve_price
    }

    /// Calculates the **Optimal Reserve Price** for a single-item auction using a custom solver.
    pub fn optimal_reserve_price_with_solver<D: ValuationDistribution, S: RootFinder>(
        dist: &D,
        lower_bound: f64,
        upper_bound: f64,
        solver: &S,
    ) -> Result<f64, AnalysisError> {
        solver.find_root(|v| dist.virtual_valuation(v), lower_bound, upper_bound)
    }

    /// Estimates the expected revenue of an optimal auction with `n_bidders`
    /// via Monte Carlo simulation.
    ///
    /// The revenue of the optimal auction is given by:
    /// $$ \text{Revenue} = \mathbb{E} \left[ \max(0, J(v_1), \dots, J(v_n)) \right] $$
    ///
    /// This simulation draws random valuations for $n$ bidders, calculates their virtual valuations,
    /// and averages the maximum non-negative virtual valuation over `n_simulations`.
    pub fn simulate_optimal_revenue<D: ValuationDistribution>(
        dist: &D,
        n_bidders: usize,
        n_simulations: usize,
    ) -> f64 {
        let mut rng = rand::thread_rng();
        Self::simulate_optimal_revenue_with_rng(dist, n_bidders, n_simulations, &mut rng)
    }

    /// Same as `simulate_optimal_revenue` but allows injecting a custom RNG
    /// for deterministic testing.
    pub fn simulate_optimal_revenue_with_rng<D: ValuationDistribution, R: Rng + ?Sized>(
        dist: &D,
        n_bidders: usize,
        n_simulations: usize,
        rng: &mut R,
    ) -> f64 {
        let mut total_revenue = 0.0;

        for _ in 0..n_simulations {
            let mut max_virtual_val = 0.0;
            for _ in 0..n_bidders {
                let v = dist.sample(rng);
                let j = dist.virtual_valuation(v);
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
    #[allow(deprecated)]
    fn test_virtual_valuation_uniform() {
        // For Uniform(0, 1): f(v) = 1, F(v) = v.
        // J(v) = v - (1 - v)/1 = v - 1 + v = 2v - 1.
        let dist = Uniform::new(0.0, 1.0).unwrap();

        let v = 0.5;
        let j = dist.virtual_valuation(v);
        assert!((j - (2.0 * v - 1.0)).abs() < 1e-9); // J(0.5) = 0.

        let v = 0.8;
        let j = dist.virtual_valuation(v);
        assert!((j - (2.0 * v - 1.0)).abs() < 1e-9); // J(0.8) = 0.6.
    }

    #[test]
    #[allow(deprecated)]
    fn test_optimal_reserve_uniform() {
        // For Uniform(0, 1), J(r) = 2r - 1 = 0 => r = 0.5.
        let dist = Uniform::new(0.0, 1.0).unwrap();
        let r_star = MechanismDesign::optimal_reserve_price(&dist, 0.0, 1.0);
        assert!((r_star - 0.5).abs() < 1e-4);
    }

    #[test]
    #[allow(deprecated)]
    fn test_revenue_simulation() {
        let dist = Uniform::new(0.0, 1.0).unwrap();
        let revenue = MechanismDesign::simulate_optimal_revenue(&dist, 1, 10_000);
        assert!((revenue - 0.25).abs() < 0.02); // MC error margin
    }

    #[test]
    #[allow(deprecated)]
    fn test_optimal_reserve_unbracketed_fallback() {
        // Uniform(0, 100). J(v) = 2v - 100. Root at 50.
        let dist = Uniform::new(0.0, 100.0).unwrap();

        // Search range [60, 100]. J(60)=20, J(100)=100. Both > 0.
        // Should return lower bound (60) as J is increasing and positive.
        let r = MechanismDesign::optimal_reserve_price(&dist, 60.0, 100.0);
        assert!((r - 60.0).abs() < 1e-4);

        // Search range [0, 40]. J(0)=-100, J(40)=-20. Both < 0.
        // Should return upper bound (40) as J is increasing and negative.
        let r = MechanismDesign::optimal_reserve_price(&dist, 0.0, 40.0);
        assert!((r - 40.0).abs() < 1e-4);
    }
}
