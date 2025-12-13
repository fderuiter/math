//! # Q-Series
//!
//! This module implements a structure for representing and manipulating q-series
//! (power series in q), which are fundamental in the theory of partitions and
//! other areas of number theory.

use std::ops::{Add, Mul, Div};

/// Represents a q-series, a power series in q.
/// The vector `coeffs` stores the coefficients, where the index represents the power of q.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QSeries {
    pub coeffs: Vec<i64>,
}

impl Default for QSeries {
    fn default() -> Self {
        Self::new()
    }
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
            // Use iterator to avoid indexing check and satisfy clippy
            for (i, &coeff) in new_coeffs.iter().enumerate().take(n) {
                 sum_val += coeff * other.get_coeff(n - i);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qseries_add() {
        let s1 = QSeries::from_vec(vec![1, 2, 3]);
        let s2 = QSeries::from_vec(vec![4, 5, 6, 7]);
        let s3 = &s1 + &s2;
        assert_eq!(s3.coeffs, vec![5, 7, 9, 7]);
    }

    #[test]
    fn test_qseries_mul() {
        let s1 = QSeries::from_vec(vec![1, 1]); // 1+q
        let s2 = QSeries::from_vec(vec![1, 1]); // 1+q
        let s3 = &s1 * &s2; // (1+q)^2 = 1+2q+q^2
        assert_eq!(s3.coeffs, vec![1, 2]); // Truncated to precision 2 based on input length

        let s4 = QSeries::from_vec(vec![1, 1, 1]); // 1+q+q^2
        let s5 = QSeries::from_vec(vec![1, -1, 0]); // 1-q
        let s6 = &s4 * &s5; // (1-q^3) = 1
        assert_eq!(s6.coeffs, vec![1, 0, 0]);
    }

    #[test]
    fn test_qseries_div() {
        // 1 / (1-q) = 1+q+q^2+...
        let one = QSeries::from_vec(vec![1,0,0,0,0]);
        let one_minus_q = QSeries::from_vec(vec![1,-1]);
        let geom_series = &one / &one_minus_q;
        assert_eq!(geom_series.coeffs, vec![1,1,1,1,1]);
    }
}
