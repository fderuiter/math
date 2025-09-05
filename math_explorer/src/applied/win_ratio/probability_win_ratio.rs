//! # Probability Win Ratio
//!
//! Functions to calculate the probability win ratio, a parameter that extends the concept
//! of the sample win ratio. This module uses numerical integration to calculate the
//! win and loss probabilities over a specified time interval.

use quadrature::clenshaw_curtis::integrate;

/// Calculates the win probability, W(c).
///
/// ## Formula
///
/// W(c) = integral from 0 to c of S1(t)*f0(t)dt - S1(c)*S0(c) * integral from 0 to c of G1(x|c)*f0(x|c)dx
///
/// where:
/// - S1(t): Marginal survival function for the treatment group.
/// - f0(t): Probability density function for the fatal event in the control group.
/// - G1(x|c): Conditional survival function for the non-fatal event in the treatment group.
/// - f0(x|c): Conditional probability density function for the non-fatal event in the control group.
/// - S0(c), S1(c): Survival probabilities at time c.
///
/// ## Parameters
///
/// * `s1`: Closure for the marginal survival function of the treatment group, S1(t).
/// * `pdf_t0`: Closure for the PDF of the fatal event in the control group, f0(t).
/// * `g1_given_c`: Closure for the conditional survival function of the non-fatal event in the treatment group, G1(x|c).
/// * `pdf_x0_given_c`: Closure for the conditional PDF of the non-fatal event in the control group, f0(x|c).
/// * `s0_at_c`: The value of S0(c).
/// * `s1_at_c`: The value of S1(c).
/// * `c`: The time interval [0, c].
/// * `error_tolerance`: The desired error tolerance for numerical integration.
pub fn calculate_win_probability<FS1, FPDFT0, FG1, FPDFX0>(
    s1: FS1,
    pdf_t0: FPDFT0,
    g1_given_c: FG1,
    pdf_x0_given_c: FPDFX0,
    s0_at_c: f64,
    s1_at_c: f64,
    c: f64,
    error_tolerance: f64,
) -> f64
where
    FS1: Fn(f64) -> f64,
    FPDFT0: Fn(f64) -> f64,
    FG1: Fn(f64) -> f64,
    FPDFX0: Fn(f64) -> f64,
{
    let integrand1 = |t: f64| s1(t) * pdf_t0(t);
    let integral1 = integrate(integrand1, 0.0, c, error_tolerance).integral;

    let integrand2 = |x: f64| g1_given_c(x) * pdf_x0_given_c(x);
    let integral2 = integrate(integrand2, 0.0, c, error_tolerance).integral;

    integral1 - s0_at_c * s1_at_c * integral2
}

/// Calculates the loss probability, L(c).
///
/// ## Formula
///
/// L(c) = integral from 0 to c of S0(t)*f1(t)dt - S1(c)*S0(c) * integral from 0 to c of G0(x|c)*f1(x|c)dx
///
/// ## Parameters
/// (See `calculate_win_probability` for details, with roles of group 0 and 1 swapped)
pub fn calculate_loss_probability<FS0, FPDFT1, FG0, FPDFX1>(
    s0: FS0,
    pdf_t1: FPDFT1,
    g0_given_c: FG0,
    pdf_x1_given_c: FPDFX1,
    s0_at_c: f64,
    s1_at_c: f64,
    c: f64,
    error_tolerance: f64,
) -> f64
where
    FS0: Fn(f64) -> f64,
    FPDFT1: Fn(f64) -> f64,
    FG0: Fn(f64) -> f64,
    FPDFX1: Fn(f64) -> f64,
{
    let integrand1 = |t: f64| s0(t) * pdf_t1(t);
    let integral1 = integrate(integrand1, 0.0, c, error_tolerance).integral;

    let integrand2 = |x: f64| g0_given_c(x) * pdf_x1_given_c(x);
    let integral2 = integrate(integrand2, 0.0, c, error_tolerance).integral;

    integral1 - s0_at_c * s1_at_c * integral2
}

/// Calculates the probability win ratio, PR(c).
///
/// ## Formula
///
/// PR(c) = W(c) / L(c)
///
/// ## Returns
///
/// The probability win ratio as a `f64`. Returns `f64::INFINITY` if the loss probability is zero.
pub fn calculate_probability_win_ratio(win_probability: f64, loss_probability: f64) -> f64 {
    if loss_probability == 0.0 {
        return f64::INFINITY;
    }
    win_probability / loss_probability
}
