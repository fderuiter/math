use crate::error::MarkovError;
pub type Result<T> = std::result::Result<T, MarkovError>;
use nalgebra::{DMatrix, DVector, RealField};
use num_traits::ToPrimitive;

/// Validates that a matrix is row-stochastic.
#[verified_engine::verified]
pub fn validate_stochastic_matrix<T: RealField + Copy + ToPrimitive>(
    matrix: &DMatrix<T>,
) -> Result<()> {
    let tolerance = T::from_f64(1e-10).unwrap();
    let one = T::one();
    let zero = T::zero();

    for i in 0..matrix.nrows() {
        let row_sum: T = matrix.row(i).iter().fold(zero, |acc, &x| acc + x);
        if (row_sum - one).abs() > tolerance {
            return Err(MarkovError::NotStochastic {
                reason: format!(
                    "Row {} sums to {} instead of 1.0",
                    i,
                    row_sum.to_f64().unwrap_or(f64::NAN)
                ),
            });
        }

        for j in 0..matrix.ncols() {
            let p = matrix[(i, j)];
            if !p.is_finite() || p < zero || p > one {
                return Err(MarkovError::InvalidProbability {
                    value: p.to_f64().unwrap_or(f64::NAN),
                });
            }
        }
    }

    Ok(())
}

/// Validates that a vector is a valid probability distribution.
#[verified_engine::verified]
pub fn validate_probability_vector<T: RealField + Copy + ToPrimitive>(
    vec: &DVector<T>,
) -> Result<()> {
    let tolerance = T::from_f64(1e-10).unwrap();
    let one = T::one();
    let zero = T::zero();

    let sum: T = vec.iter().fold(zero, |acc, &x| acc + x);
    if (sum - one).abs() > tolerance {
        return Err(MarkovError::NotStochastic {
            reason: format!(
                "Vector sums to {} instead of 1.0",
                sum.to_f64().unwrap_or(f64::NAN)
            ),
        });
    }

    for &p in vec.iter() {
        if !p.is_finite() || p < zero || p > one {
            return Err(MarkovError::InvalidProbability {
                value: p.to_f64().unwrap_or(f64::NAN),
            });
        }
    }

    Ok(())
}

/// Validates that a matrix is a valid continuous-time Markov chain generator.
#[verified_engine::verified]
pub fn validate_generator_matrix<T: RealField + Copy + ToPrimitive>(
    generator: &DMatrix<T>,
) -> Result<()> {
    let tolerance = T::from_f64(1e-10).unwrap();
    let zero = T::zero();

    for i in 0..generator.nrows() {
        // Check row sum is 0
        let row_sum: T = generator.row(i).iter().fold(zero, |acc, &x| acc + x);
        if row_sum.abs() > tolerance {
            return Err(MarkovError::InvalidGenerator {
                reason: format!(
                    "Row {} sums to {} instead of 0.0",
                    i,
                    row_sum.to_f64().unwrap_or(f64::NAN)
                ),
            });
        }

        // Check diagonal is non-positive
        if generator[(i, i)] > tolerance {
            return Err(MarkovError::InvalidGenerator {
                reason: format!(
                    "Diagonal element G[{},{}] = {} must be non-positive",
                    i,
                    i,
                    generator[(i, i)].to_f64().unwrap_or(f64::NAN)
                ),
            });
        }

        // Check off-diagonals are non-negative
        for j in 0..generator.ncols() {
            if i != j {
                let rate = generator[(i, j)];
                if rate < -tolerance {
                    return Err(MarkovError::InvalidGenerator {
                        reason: format!(
                            "Off-diagonal element G[{},{}] = {} must be non-negative",
                            i,
                            j,
                            rate.to_f64().unwrap_or(f64::NAN)
                        ),
                    });
                }
            }
        }

        // Check all values are finite
        for j in 0..generator.ncols() {
            if !generator[(i, j)].is_finite() {
                return Err(MarkovError::InvalidGenerator {
                    reason: format!("G[{},{}] is not finite", i, j),
                });
            }
        }
    }

    Ok(())
}
