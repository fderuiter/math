//! # Partition Functions
//!
//! This module implements functions related to integer partitions,
//! focusing on the seven restricted partition functions introduced by Pushpa and Vasuki.
//! The implementation is based on the paper "Arithmetic properties of partition
//! functions introduced by Pushpa and Vasuki" by Nath and Saikia.

use std::ops::{Add, Mul, Div};

/// Represents a q-series, a power series in q.
/// The vector `coeffs` stores the coefficients, where the index represents the power of q.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QSeries {
    pub coeffs: Vec<i64>,
}

impl QSeries {
    /// Creates a new empty QSeries.
    pub fn new() -> Self {
        QSeries { coeffs: vec![] }
    }

    /// Creates a QSeries with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        QSeries {
            coeffs: Vec::with_capacity(capacity),
        }
    }

    /// Creates a QSeries from a vector of coefficients.
    pub fn from_vec(coeffs: Vec<i64>) -> Self {
        QSeries { coeffs }
    }

    /// Gets the coefficient of q^n.
    pub fn get_coeff(&self, n: usize) -> i64 {
        self.coeffs.get(n).cloned().unwrap_or(0)
    }

    /// Truncates the series to a given precision.
    pub fn truncate(&mut self, precision: usize) {
        self.coeffs.truncate(precision);
    }

    /// Computes the power of a QSeries using exponentiation by squaring.
    pub fn pow(&self, exp: u32) -> QSeries {
        let precision = self.coeffs.len();
        if exp == 0 {
            let mut coeffs = vec![0; precision];
            if precision > 0 {
                coeffs[0] = 1;
            }
            return QSeries::from_vec(coeffs);
        }

        let mut base = self.clone();
        let mut e = exp;

        let mut result_coeffs = vec![0; precision];
        if precision > 0 {
            result_coeffs[0] = 1;
        }
        let mut result = QSeries::from_vec(result_coeffs);

        while e > 0 {
            if e % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            e /= 2;
        }
        result
    }
}

impl Add for &QSeries {
    type Output = QSeries;

    fn add(self, other: Self) -> QSeries {
        let len1 = self.coeffs.len();
        let len2 = other.coeffs.len();
        let max_len = std::cmp::max(len1, len2);
        let mut new_coeffs = Vec::with_capacity(max_len);

        for i in 0..max_len {
            new_coeffs.push(self.get_coeff(i) + other.get_coeff(i));
        }

        QSeries { coeffs: new_coeffs }
    }
}

impl Mul for &QSeries {
    type Output = QSeries;

    fn mul(self, other: Self) -> QSeries {
        let len1 = self.coeffs.len();
        let len2 = other.coeffs.len();
        if len1 == 0 || len2 == 0 {
            return QSeries::new();
        }
        let precision = std::cmp::max(len1, len2);
        let mut new_coeffs = vec![0; precision];

        for i in 0..len1 {
            for j in 0..len2 {
                if i + j < precision {
                    new_coeffs[i + j] += self.coeffs[i] * other.coeffs[j];
                }
            }
        }

        QSeries { coeffs: new_coeffs }
    }
}

impl Div for &QSeries {
    type Output = QSeries;

    fn div(self, other: Self) -> QSeries {
        let len1 = self.coeffs.len();
        let len2 = other.coeffs.len();
        if len2 == 0 {
            panic!("Division by zero QSeries");
        }
        let b0 = other.get_coeff(0);
        if b0 == 0 {
            panic!("Division by a QSeries with zero constant term");
        }

        let precision = len1;
        let mut new_coeffs = vec![0; precision];

        for n in 0..precision {
            let mut sum_val = 0;
            for i in 0..n {
                sum_val += new_coeffs[i] * other.get_coeff(n - i);
            }
            let numerator = self.get_coeff(n) - sum_val;
            if numerator % b0 != 0 {
                // This can happen in intermediate calculations.
                // The final result should have integer coefficients.
            }
            new_coeffs[n] = numerator / b0;
        }

        QSeries { coeffs: new_coeffs }
    }
}


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
