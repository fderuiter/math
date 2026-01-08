//! Type definitions for clinical trials.

use std::fmt;

/// Errors that can occur during clinical trial analysis.
#[derive(Debug, Clone)]
pub enum ClinicalTrialError {
    /// Sample size is too small for the requested test.
    InsufficientSampleSize { required: usize, actual: usize },
    /// Invalid input data (e.g., negative counts).
    InvalidData(String),
    /// Statistical calculation failed (e.g., convergence issue).
    StatisticalError(String),
    /// Zero total count in a contingency table.
    ZeroTotalCount,
}

impl fmt::Display for ClinicalTrialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InsufficientSampleSize { required, actual } => {
                write!(f, "Insufficient sample size: required {}, got {}", required, actual)
            }
            Self::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Self::StatisticalError(msg) => write!(f, "Statistical error: {}", msg),
            Self::ZeroTotalCount => write!(f, "Total count in contingency table is zero"),
        }
    }
}

impl std::error::Error for ClinicalTrialError {}

/// A 2x2 Contingency Table for binary outcomes.
///
/// | | Event | No Event |
/// |---|---|---|
/// | Treatment | a | b |
/// | Control | c | d |
#[derive(Debug, Clone, Copy)]
pub struct ContingencyTable {
    /// Treatment Group: Event count (a).
    pub treatment_event: u32,
    /// Treatment Group: No Event count (b).
    pub treatment_no_event: u32,
    /// Control Group: Event count (c).
    pub control_event: u32,
    /// Control Group: No Event count (d).
    pub control_no_event: u32,
}

impl ContingencyTable {
    /// Creates a new contingency table.
    pub fn new(
        treatment_event: u32,
        treatment_no_event: u32,
        control_event: u32,
        control_no_event: u32,
    ) -> Result<Self, ClinicalTrialError> {
        let total = treatment_event as u64
            + treatment_no_event as u64
            + control_event as u64
            + control_no_event as u64;

        if total == 0 {
            return Err(ClinicalTrialError::ZeroTotalCount);
        }

        Ok(Self {
            treatment_event,
            treatment_no_event,
            control_event,
            control_no_event,
        })
    }

    /// Returns total count.
    pub fn total(&self) -> f64 {
        (self.treatment_event
            + self.treatment_no_event
            + self.control_event
            + self.control_no_event) as f64
    }
}

/// A wrapper for continuous data from a study group.
///
/// Calculates and caches summary statistics to avoid re-computation.
#[derive(Debug, Clone)]
pub struct GroupData {
    data: Vec<f64>,
}

impl GroupData {
    /// Creates a new `GroupData` instance.
    pub fn new(data: Vec<f64>) -> Result<Self, ClinicalTrialError> {
        if data.len() < 2 {
            return Err(ClinicalTrialError::InsufficientSampleSize {
                required: 2,
                actual: data.len(),
            });
        }
        Ok(Self { data })
    }

    /// Returns the raw data slice.
    pub fn as_slice(&self) -> &[f64] {
        &self.data
    }

    /// Returns the number of samples.
    pub fn n(&self) -> usize {
        self.data.len()
    }

    /// Calculates the mean.
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.data.iter().sum();
        sum / self.n() as f64
    }

    /// Calculates the variance.
    pub fn variance(&self) -> f64 {
        let m = self.mean();
        let sum_sq_diff: f64 = self.data.iter().map(|x| (x - m).powi(2)).sum();
        sum_sq_diff / (self.n() - 1) as f64
    }
}
