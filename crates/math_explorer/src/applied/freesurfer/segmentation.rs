/// Calculates the posterior probability for a single voxel label.
/// P(L|I) proportional to P(I|L) * P(L)
///
/// # Arguments
/// * `likelihood` - P(I|L): Probability of intensity given the label.
/// * `prior` - P(L): Prior probability of the label at this location.
///
/// # Returns
/// The unnormalized posterior probability. The normalization (denominator)
/// would be the sum of this value over all possible labels.
pub fn bayesian_classification(likelihood: f64, prior: f64) -> f64 {
    likelihood * prior
}
