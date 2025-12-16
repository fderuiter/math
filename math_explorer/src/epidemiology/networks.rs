/// Calculates R0 for a heterogeneous network.
///
/// $R_0 = \frac{\beta}{\gamma} \frac{\langle k^2 \rangle - \langle k \rangle}{\langle k \rangle}$
pub fn heterogeneous_r0(beta: f64, gamma: f64, mean_degree: f64, degree_variance: f64) -> f64 {
    if mean_degree == 0.0 || gamma == 0.0 {
        return 0.0;
    }

    // Var(k) = E[k^2] - (E[k])^2
    // E[k^2] = Var(k) + (E[k])^2

    let mean_k_sq = degree_variance + mean_degree.powi(2);
    let factor = (mean_k_sq - mean_degree) / mean_degree;

    (beta / gamma) * factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heterogeneous_r0() {
        let beta = 0.5;
        let gamma = 0.1;
        // Homogeneous network: Variance = 0. Factor = (k^2 - k)/k = (k^2 - k)/k = k - 1?
        // Wait, if Var=0, then <k^2> = <k>^2.
        // Factor = (<k>^2 - <k>)/<k> = <k> - 1.
        // Standard formula usually assumes contact rate is proportional to k.

        // Using provided formula:
        let mean_k = 4.0;
        let var_k = 0.0;
        let r0 = heterogeneous_r0(beta, gamma, mean_k, var_k);

        // R0 = (beta/gamma) * (16 - 4)/4 = 5 * 3 = 15.
        assert!((r0 - 15.0).abs() < 1e-6);
    }
}
