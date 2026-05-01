use super::linear_algebra::Vector;

/// Softmax function.
/// Converts a vector of raw scores (logits) into a probability distribution.
///
/// P(y=j|\mathbf{x}) = \frac{e^{z_j}}{\sum_{k} e^{z_k}}
///
/// This ensures all outputs sum to 1, representing a valid categorical distribution.
pub fn softmax(z: &Vector) -> Vector {
    // For numerical stability, subtract the max value from z before exp.
    let max_z = z.max();
    let exps = z.map(|v| (v - max_z).exp());
    let sum_exps = exps.sum();
    exps / sum_exps
}

/// Likelihood and Maximum Likelihood Estimation (MLE) explanation.
///
/// Most training objectives are derived from the principle of MLE:
/// finding the parameters \theta that maximize the likelihood of the observed data.
///
/// L(\theta) = \prod_{i} P(y^{(i)} | x^{(i)}; \theta)
///
/// In practice, we maximize the Log-Likelihood:
/// \ell(\theta) = \sum_{i} \log P(y^{(i)} | x^{(i)}; \theta)
///
/// Minimizing the Cross-Entropy Loss is mathematically equivalent to maximizing
/// the likelihood for classification tasks.
pub fn mle_explanation() -> &'static str {
    "MLE searches for parameters that maximize the probability of the observed data."
}
