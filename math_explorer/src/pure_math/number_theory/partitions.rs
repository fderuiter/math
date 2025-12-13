//! # Partition Functions
//!
//! This module implements functions related to integer partitions,
//! focusing on the seven restricted partition functions introduced by Pushpa and Vasuki.
//! The implementation is based on the paper "Arithmetic properties of partition
//! functions introduced by Pushpa and Vasuki" by Nath and Saikia.

// Re-export QSeries so that users of this module (like tests) can still access it via
// math_explorer::pure_math::number_theory::partitions::QSeries
// This also brings QSeries into scope for this module.
pub use crate::pure_math::number_theory::q_series::QSeries;

/// Computes the q-series for f_k = (q^k; q^k)_inf up to a given precision.
/// f_k = product_{i>=1} (1 - q^(k*i))
pub fn f_k(k: usize, precision: usize) -> QSeries {
    if k == 0 {
        return QSeries::from_vec(vec![0; precision]);
    }
    if precision == 0 {
        return QSeries::new();
    }
    if k >= precision {
        let mut coeffs = vec![0; precision];
        coeffs[0] = 1;
        return QSeries { coeffs };
    }
    let mut coeffs = vec![0; precision];
    coeffs[0] = 1;

    for i in 1.. {
        let power = i * k;
        if power >= precision {
            break;
        }
        // This is multiplication by (1 - q^power)
        for j in (power..precision).rev() {
            coeffs[j] -= coeffs[j - power];
        }
    }
    QSeries { coeffs }
}

/// Generating function for P*(n)
pub fn gen_p_star(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f5 = f_k(5, precision);
    let f1_pow4 = f1.pow(4);
    let f5_pow4 = f5.pow(4);
    &f1_pow4 * &f5_pow4
}

/// Generating function for M(n)
pub fn gen_m(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f5 = f_k(5, precision);
    let f10 = f_k(10, precision);

    let f2_pow5 = f2.pow(5);
    let f5_pow5 = f5.pow(5);

    let numerator = &f2_pow5 * &f5_pow5;
    let denominator = &f1 * &f10;

    &numerator / &denominator
}

/// Generating function for T*(n)
pub fn gen_t_star(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f5 = f_k(5, precision);
    let f10 = f_k(10, precision);

    let f1_pow5 = f1.pow(5);
    let f10_pow5 = f10.pow(5);

    let numerator = &f1_pow5 * &f10_pow5;
    let denominator = &f2 * &f5;

    &numerator / &denominator
}

/// Generating function for A(n)
pub fn gen_a(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f7 = f_k(7, precision);

    let f2_pow6 = f2.pow(6);
    let f7_pow6 = f7.pow(6);
    let f1_pow2 = f1.pow(2);

    let numerator = &f2_pow6 * &f7_pow6;

    &numerator / &f1_pow2
}

/// Generating function for B(n)
pub fn gen_b(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f7 = f_k(7, precision);
    let f14 = f_k(14, precision);

    let f1_pow6 = f1.pow(6);
    let f14_pow4 = f14.pow(4);
    let f2_pow2 = f2.pow(2);
    let f7_pow2 = f7.pow(2);

    let numerator = &f1_pow6 * &f14_pow4;
    let denominator = &f2_pow2 * &f7_pow2;

    &numerator / &denominator
}

/// Generating function for K(n)
pub fn gen_k(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f7 = f_k(7, precision);
    let f14 = f_k(14, precision);

    let f1_pow2 = f1.pow(2);
    let f2_pow2 = f2.pow(2);
    let f7_pow2 = f7.pow(2);
    let f14_pow2 = f14.pow(2);

    &(&(&f1_pow2 * &f2_pow2) * &f7_pow2) * &f14_pow2
}

/// Generating function for L(n)
pub fn gen_l(precision: usize) -> QSeries {
    let f1 = f_k(1, precision);
    let f2 = f_k(2, precision);
    let f7 = f_k(7, precision);
    let f14 = f_k(14, precision);

    let f1_pow5 = f1.pow(5);
    let f7_pow5 = f7.pow(5);

    let numerator = &f1_pow5 * &f7_pow5;
    let denominator = &f2 * &f14;

    &numerator / &denominator
}
