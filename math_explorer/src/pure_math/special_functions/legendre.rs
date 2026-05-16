/// Legendre Polynomial $P_\ell(x)$ computed via recurrence relation.
///
/// $(n+1)P_{n+1}(x) = (2n+1)x P_n(x) - n P_{n-1}(x)$
pub fn legendre_p(l: u64, x: f64) -> f64 {
    if l == 0 {
        return 1.0;
    }
    if l == 1 {
        return x;
    }

    let mut p_prev = 1.0; // P_0
    let mut p_curr = x; // P_1

    for n in 1..l {
        let n_f = n as f64;
        let p_next = ((2.0 * n_f + 1.0) * x * p_curr - n_f * p_prev) / (n_f + 1.0);
        p_prev = p_curr;
        p_curr = p_next;
    }
    p_curr
}

/// Associated Legendre Polynomial $P_\ell^m(x)$.
///
/// Uses standard recurrence relations.
/// Note: This implementation includes the Condon-Shortley phase $(-1)^m$.
pub fn legendre_p_associated(l: u64, m: i64, x: f64) -> f64 {
    if m < 0 {
        // Relationship for negative m can be added if needed,
        // typically P_l^{-m} = (-1)^m (l-m)!/(l+m)! P_l^m
        // For now, return 0.0 or panic? 0.0 is safer.
        return 0.0;
    }
    let m = m as u64;
    if m > l {
        return 0.0;
    }

    // 1. Compute P_m^m(x)
    // P_m^m(x) = (-1)^m (2m-1)!! (1-x^2)^(m/2)
    let mut p_mm = 1.0;
    if m > 0 {
        let somx2 = (1.0 - x * x).sqrt();
        let mut fact = 1.0;
        for _ in 1..=m {
            p_mm *= -fact * somx2;
            fact += 2.0;
        }
    }
    if l == m {
        return p_mm;
    }

    // 2. Compute P_{m+1}^m(x)
    // P_{m+1}^m(x) = x(2m+1) P_m^m(x)
    let p_m1m = x * (2.0 * m as f64 + 1.0) * p_mm;
    if l == m + 1 {
        return p_m1m;
    }

    // 3. Compute P_l^m(x) for l > m+1
    // (l-m) P_l^m = x(2l-1) P_{l-1}^m - (l+m-1) P_{l-2}^m
    let mut p_prev = p_mm; // P_m^m
    let mut p_curr = p_m1m; // P_{m+1}^m

    for ll in (m + 2)..=l {
        let ll_f = ll as f64;
        let m_f = m as f64;
        let p_next = (x * (2.0 * ll_f - 1.0) * p_curr - (ll_f + m_f - 1.0) * p_prev) / (ll_f - m_f);
        p_prev = p_curr;
        p_curr = p_next;
    }
    p_curr
}

/// Checks orthogonality of Legendre polynomials on [-1, 1].
/// $\int_{-1}^1 P_\ell(x) P_k(x) dx = \frac{2}{2\ell+1} \delta_{\ell k}$
pub fn check_orthogonality_legendre(l: u64, k: u64) -> f64 {
    let steps = 1000;
    let h = 2.0 / steps as f64; // Range is 2 (-1 to 1)
    let mut sum = 0.0;

    let integrand = |x: f64| legendre_p(l, x) * legendre_p(k, x);

    for i in 0..steps {
        let x = -1.0 + i as f64 * h;
        let x_next = -1.0 + (i + 1) as f64 * h;
        sum += 0.5 * (integrand(x) + integrand(x_next)) * h;
    }
    sum
}
