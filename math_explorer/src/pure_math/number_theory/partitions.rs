//! Partition Function Q-Series
//!
//! Calculates the partition function $P(n)$ using the pentagonal number theorem.
//!
//! $$ \prod_{k=1}^{\infty} (1 - x^k) = \sum_{k=-\infty}^{\infty} (-1)^k x^{k(3k-1)/2} $$

use super::q_series::QSeries;

/// Generates the Q-Series for the partition function denominator.
///
/// $\phi(x) = \prod_{k=1}^{\infty} (1 - x^k)$
///
/// This uses the Pentagonal Number Theorem for $O(\sqrt{N})$ efficiency.
pub fn partition_generating_function(precision: usize) -> QSeries<i64> {
    // We need to fill coefficients up to x^precision.
    // The pentagonal numbers are k(3k-1)/2.
    // k = 0 -> 0
    // k = 1 -> 1, k = -1 -> 2
    // k = 2 -> 5, k = -2 -> 7
    // ...

    let mut coeffs = vec![0; precision];
    coeffs[0] = 1; // Constant term is always 1

    let mut k = 1;
    loop {
        let k_pos = k;
        let k_neg = -k;

        let p_pos = (k_pos * (3 * k_pos - 1)) / 2;
        let p_neg = (k_neg * (3 * k_neg - 1)) / 2;

        let sign = if k % 2 == 0 { 1 } else { -1 };

        let idx_pos = usize::try_from(p_pos).ok();
        let idx_neg = usize::try_from(p_neg).ok();

        let mut added = false;

        if let Some(idx) = idx_pos.filter(|&i| i < precision) {
            coeffs[idx] = sign;
            added = true;
        }

        if let Some(idx) = idx_neg.filter(|&i| i < precision) {
            coeffs[idx] = sign;
            added = true;
        }

        if !added {
            break;
        }

        k += 1;
    }

    QSeries::from_vec(coeffs)
}

/// Calculates $p(n)$, the number of partitions of $n$.
///
/// $$ \sum_{n=0}^{\infty} p(n)x^n = \frac{1}{\phi(x)} $$
pub fn partition_function(n: usize) -> i64 {
    // We need the inverse of the generating function.
    // P(x) * Phi(x) = 1
    // We can solve this recursively.
    // p(n) = sum_{k != 0, (-1)^(k-1) * p(n - pent(k))}

    if n == 0 {
        return 1;
    }

    // Dynamic programming / recursion with memoization is implicit if we compute iteratively.
    // We can just use the recurrence directly.

    let mut p = vec![0i64; n + 1];
    p[0] = 1;

    for i in 1..=n {
        let mut sum = 0;
        let mut k = 1;

        loop {
            let pent_1 = (k * (3 * k - 1)) / 2;
            let pent_2 = (k * (3 * k + 1)) / 2; // This corresponds to -k

            let sign = if k % 2 == 0 { -1 } else { 1 };

            let mut term_added = false;

            if pent_1 <= (i as i64) {
                sum += sign * p[i - pent_1 as usize];
                term_added = true;
            }

            if pent_2 <= (i as i64) {
                sum += sign * p[i - pent_2 as usize];
                term_added = true;
            }

            if !term_added {
                break;
            }

            k += 1;
        }
        p[i] = sum;
    }

    p[n]
}

/// Generic version of `f_k` used in Q-Series multiplication optimization.
///
/// Returns the k-th pentagonal number (generalized).
pub fn f_k(k: i64) -> i64 {
    k * (3 * k - 1) / 2
}
