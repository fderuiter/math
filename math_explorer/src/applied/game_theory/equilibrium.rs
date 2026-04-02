use nalgebra::DVector;

/// Represents a compact, convex subset of Euclidean space.
///
/// In Game Theory, the strategy space of a player is often modeled as a convex set.
/// The convexity property is crucial for the application of fixed-point theorems
/// (like Kakutani's) which guarantee the existence of Nash Equilibria.
pub trait ConvexSet {
    /// Checks if a point belongs to the set.
    fn contains(&self, point: &DVector<f64>) -> bool;

    /// Checks if the set is convex.
    ///
    /// *Note:* This is often a theoretical property assumed by the modeler.
    /// An implementation might perform a randomized check (e.g., is the midpoint of two random points in the set?).
    fn is_convex(&self) -> bool;
}

/// A simplified representation of a Box constraint set $[min, max]^n$.
///
/// A hyper-rectangle is always convex and compact (closed and bounded), making it
/// a valid domain for standard fixed-point theorems.
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

/// A set-valued function (correspondence) $\phi: S \to 2^S$.
///
/// In Nash Equilibrium analysis, the "Best Response" function is often a set-valued correspondence
/// (i.e., there may be multiple equally good strategies).
///
/// Kakutani's Fixed Point Theorem states that if $S$ is a non-empty, compact, convex subset of Euclidean space,
/// and $\phi: S \to 2^S$ is upper hemicontinuous with non-empty, convex images, then there exists a fixed point
/// $x^* \in \phi(x^*)$. This fixed point is the Nash Equilibrium.
pub trait Correspondence {
    /// Checks if `target` is in the image set $\phi(\text{source})$.
    /// effectively: `target` $\in \phi(\text{source})$
    fn is_in_image(&self, source: &DVector<f64>, target: &DVector<f64>) -> bool;
}

/// Checks if $x^* \in \phi(x^*)$.
///
/// If this returns true, `point` represents a Nash Equilibrium (or a fixed point of the dynamical system).
pub fn is_fixed_point<C: Correspondence>(correspondence: &C, point: &DVector<f64>) -> bool {
    correspondence.is_in_image(point, point)
}

/// Verifies if a point is a fixed point for the given correspondence.
#[deprecated(
    since = "0.2.0",
    note = "FixedPointVerifier struct is deprecated. Use the module-level is_fixed_point function directly."
)]
pub struct FixedPointVerifier;

#[allow(deprecated)]
impl FixedPointVerifier {
    #[deprecated(
        since = "0.2.0",
        note = "Use the module-level is_fixed_point function directly."
    )]
    pub fn is_fixed_point<C: Correspondence>(correspondence: &C, point: &DVector<f64>) -> bool {
        is_fixed_point(correspondence, point)
    }
}

/// Example: Best Response correspondence in a simplified 2-player game.
///
/// This is a utility to demonstrate how one might wrap a Nash equilibrium check.
/// For a function $f(x)$, the correspondence is often defined as "all points close to $f(x)$".
pub struct BestResponseCorrespondence {
    /// A mapping function that returns the "ideal" best response.
    #[allow(clippy::type_complexity)]
    pub mapping: Box<dyn Fn(&DVector<f64>) -> DVector<f64>>,
    /// Tolerance for checking set membership (handling floating point inaccuracies).
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
        // Define a simple mapping f(x) = 0.5 * x. Fixed point is 0.
        let correspondence = BestResponseCorrespondence {
            mapping: Box::new(|x| 0.5 * x),
            tolerance: 1e-6,
        };

        let point_zero = DVector::from_vec(vec![0.0]);
        let point_one = DVector::from_vec(vec![1.0]);

        // x=0 => f(0)=0. 0 is in {y | |y-0| < eps}. True.
        assert!(is_fixed_point(
            &correspondence,
            &point_zero
        ));

        // x=1 => f(1)=0.5. 1 is NOT in {y | |y-0.5| < eps}. False.
        assert!(!is_fixed_point(
            &correspondence,
            &point_one
        ));
    }
}
