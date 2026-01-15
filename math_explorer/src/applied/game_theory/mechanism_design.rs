use rand::Rng;
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

/// Mechanism Design utilities.
pub struct MechanismDesign;

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
    ///
    /// # Example
    /// ```
    /// use math_explorer::applied::game_theory::mechanism_design::MechanismDesign;
    /// use statrs::distribution::Uniform;
    ///
    /// // For Uniform(0, 100), J(v) = 2v - 100.
    /// // J(r) = 0 => 2r = 100 => r = 50.
    /// let dist = Uniform::new(0.0, 100.0).unwrap();
    /// let r_star = MechanismDesign::optimal_reserve_price(&dist, 0.0, 100.0);
    /// assert!((r_star - 50.0).abs() < 1e-4);
    /// ```
    pub fn optimal_reserve_price<D: ValuationDistribution>(
        dist: &D,
        lower_bound: f64,
        upper_bound: f64,
    ) -> f64 {
        let mut low = lower_bound;
        let mut high = upper_bound;
        let mut mid = low;

        for _ in 0..50 {
            mid = (low + high) / 2.0;
            let j_val = dist.virtual_valuation(mid);

            // Assuming regularity (J is increasing), if J(mid) > 0, we need smaller v (wait, J is increasing in v usually).
            // J(v) = v - (1-F)/f. As v increases, 1-F decreases, f varies.
            // For regular distributions, J is increasing.
            // If J(mid) > 0, then the root is to the left? No.
            // If J is increasing, and we want J(r) = 0.
            // If J(mid) > 0, then we are to the RIGHT of the root. So we need to lower the search range.
            // wait: J(r*) = 0.
            // If mid > r*, then J(mid) > J(r*) = 0.
            // So if J(mid) > 0, mid is too high.
            if j_val > 0.0 {
                high = mid;
            } else {
                low = mid;
            }
        }
        mid
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
    fn test_optimal_reserve_uniform() {
        // For Uniform(0, 1), J(r) = 2r - 1 = 0 => r = 0.5.
        let dist = Uniform::new(0.0, 1.0).unwrap();
        let r_star = MechanismDesign::optimal_reserve_price(&dist, 0.0, 1.0);
        assert!((r_star - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_revenue_simulation() {
        // 1 bidder, Uniform(0, 1). Optimal auction sets reserve r=0.5.
        // Revenue = E[max(0, J(v))]. J(v) = 2v-1.
        // If v < 0.5, J < 0, contribution 0.
        // If v >= 0.5, contribution 2v-1.
        // Integral_{0.5}^1 (2v - 1) * 1 dv = [v^2 - v]_{0.5}^1
        // = (1 - 1) - (0.25 - 0.5) = 0 - (-0.25) = 0.25.

        let dist = Uniform::new(0.0, 1.0).unwrap();
        let revenue = MechanismDesign::simulate_optimal_revenue(&dist, 1, 10_000);
        assert!((revenue - 0.25).abs() < 0.02); // MC error margin
    }
}
