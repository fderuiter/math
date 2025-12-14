use nalgebra::DVector;

/// Represents a compact, convex subset of Euclidean space.
pub trait ConvexSet {
    /// Checks if a point belongs to the set.
    fn contains(&self, point: &DVector<f64>) -> bool;

    /// Checks if the set is convex (usually theoretical, here we might implement a check for sampled points).
    fn is_convex(&self) -> bool;
}

/// A simplified representation of a Box constraint set [min, max]^n, which is always convex and compact.
pub struct BoxSet {
    pub min_bounds: DVector<f64>,
    pub max_bounds: DVector<f64>,
}

impl BoxSet {
    pub fn new(min_vals: Vec<f64>, max_vals: Vec<f64>) -> Self {
        assert_eq!(min_vals.len(), max_vals.len());
        Self {
            min_bounds: DVector::from_vec(min_vals),
            max_bounds: DVector::from_vec(max_vals),
        }
    }
}

impl ConvexSet for BoxSet {
    fn contains(&self, point: &DVector<f64>) -> bool {
        if point.len() != self.min_bounds.len() {
            return false;
        }
        for i in 0..point.len() {
            if point[i] < self.min_bounds[i] || point[i] > self.max_bounds[i] {
                return false;
            }
        }
        true
    }

    fn is_convex(&self) -> bool {
        true // A box is always convex
    }
}

/// A set-valued function (correspondence) \phi: S -> 2^S.
/// In practice, we define it as a function that returns a set of points (or a region description).
/// For Kakutani, we need to check if x* \in \phi(x*).
pub trait Correspondence {
    /// Returns true if `target` is in the set \phi(source).
    /// effectively: target \in \phi(source)
    fn is_in_image(&self, source: &DVector<f64>, target: &DVector<f64>) -> bool;
}

/// Verifies if a point is a fixed point for the given correspondence.
/// i.e., checks if x* \in \phi(x*).
pub struct FixedPointVerifier;

impl FixedPointVerifier {
    pub fn is_fixed_point<C: Correspondence>(
        correspondence: &C,
        point: &DVector<f64>
    ) -> bool {
        correspondence.is_in_image(point, point)
    }
}

/// Example: Best Response correspondence in a simplified 2-player game.
/// This is a utility to demonstrate how one might wrap a Nash equilibrium check.
pub struct BestResponseCorrespondence {
    /// Payoff matrix for the player (A). We assume symmetric game or just checking one player's consistency for simplicity here,
    /// but for Nash we usually check the joint strategy x = (x1, x2).
    /// Let's implement a correspondence for a function f(x) -> {y | y minimizes distance to some target(x)}.
    /// Or simply, let's use a function where \phi(x) = { y | ||y - f(x)|| < epsilon }.
    #[allow(clippy::type_complexity)]
    pub mapping: Box<dyn Fn(&DVector<f64>) -> DVector<f64>>,
    pub tolerance: f64,
}

impl Correspondence for BestResponseCorrespondence {
    fn is_in_image(&self, source: &DVector<f64>, target: &DVector<f64>) -> bool {
        let expected = (self.mapping)(source);
        (target - expected).norm() <= self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_set_contains() {
        let box_set = BoxSet::new(vec![0.0, 0.0], vec![1.0, 1.0]);
        assert!(box_set.contains(&DVector::from_vec(vec![0.5, 0.5])));
        assert!(!box_set.contains(&DVector::from_vec(vec![1.5, 0.5])));
    }

    #[test]
    fn test_fixed_point() {
        // Define a simple mapping f(x) = x. Fixed point should be anywhere.
        // Let's use f(x) = 0.5 * x. Fixed point is 0.
        let correspondence = BestResponseCorrespondence {
            mapping: Box::new(|x| 0.5 * x),
            tolerance: 1e-6,
        };

        let point_zero = DVector::from_vec(vec![0.0]);
        let point_one = DVector::from_vec(vec![1.0]);

        assert!(FixedPointVerifier::is_fixed_point(&correspondence, &point_zero));
        assert!(!FixedPointVerifier::is_fixed_point(&correspondence, &point_one));
    }
}
