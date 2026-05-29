use thiserror::Error;
use math_core::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

/// Number Theory module errors
#[derive(Debug, Error)]
pub enum NumberTheoryError {
    #[error("Failed to parse integer from string: {0}")]
    ParseError(String),

    #[error("Modulo operation failed")]
    ModuloError,

    #[error("Conversion to usize failed")]
    ConversionError,

    #[error("Division by zero QSeries")]
    DivisionByZeroQSeries,

    #[error("Division by a QSeries with zero constant term")]
    DivisionByZeroConstantTerm,
}


impl Diagnostic for NumberTheoryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "NumberTheoryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Markov chain operations.


/// Errors that can occur during Markov chain operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkovError {
    /// Invalid probability value (must be in [0, 1] and finite).
    InvalidProbability {
        /// The invalid probability value.
        value: f64,
    },

    /// Matrix dimension mismatch.
    DimensionMismatch {
        /// Expected dimension.
        expected: usize,
        /// Actual dimension.
        actual: usize,
    },

    /// Matrix is not stochastic (rows don't sum to 1).
    NotStochastic {
        /// Description of the issue.
        reason: String,
    },

    /// Matrix is not a valid generator (rows don't sum to 0 or invalid rates).
    InvalidGenerator {
        /// Description of the issue.
        reason: String,
    },

    /// Numerical computation error.
    NumericalError {
        /// Description of the error.
        reason: String,
    },

    /// Invalid state specification.
    InvalidState {
        /// Description of the issue.
        reason: String,
    },

    /// Time index out of bounds.
    TimeIndexOutOfBounds {
        /// The requested time.
        time: f64,
        /// Valid time range.
        valid_range: (f64, f64),
    },

    /// Observation sequence incompatible with model.
    InvalidObservation {
        /// Description of the issue.
        reason: String,
    },

    /// Matrix not invertible or singular.
    SingularMatrix {
        /// Context about which matrix.
        context: String,
    },
}


impl fmt::Display for MarkovError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkovError::InvalidProbability { value } => {
                write!(
                    f,
                    "Invalid probability value: {} (must be in [0, 1])",
                    value
                )
            }
            MarkovError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            MarkovError::NotStochastic { reason } => {
                write!(f, "Matrix is not stochastic: {}", reason)
            }
            MarkovError::InvalidGenerator { reason } => {
                write!(f, "Invalid generator matrix: {}", reason)
            }
            MarkovError::NumericalError { reason } => {
                write!(f, "Numerical error: {}", reason)
            }
            MarkovError::InvalidState { reason } => {
                write!(f, "Invalid state: {}", reason)
            }
            MarkovError::TimeIndexOutOfBounds { time, valid_range } => {
                write!(
                    f,
                    "Time {} is out of bounds (valid range: [{}, {}])",
                    time, valid_range.0, valid_range.1
                )
            }
            MarkovError::InvalidObservation { reason } => {
                write!(f, "Invalid observation: {}", reason)
            }
            MarkovError::SingularMatrix { context } => {
                write!(f, "Singular matrix: {}", context)
            }
        }
    }
}


impl std::error::Error for MarkovError {}


impl Diagnostic for MarkovError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "MarkovError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Ornstein-Uhlenbeck process.


/// Errors that can occur during OU process simulation.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum OuError {
    /// Invalid mean reversion rate (must be positive).
    #[error("Invalid mean reversion rate: {value} (must be positive)")]
    InvalidMeanReversionRate { value: f64 },
    /// Invalid volatility (must be non-negative).
    #[error("Invalid volatility: {value} (must be non-negative)")]
    InvalidVolatility { value: f64 },
    /// Invalid time step (must be positive).
    #[error("Invalid time step: {value} (must be positive)")]
    InvalidTimeStep { value: f64 },
    /// Invalid simulation parameters.
    #[error("Invalid simulation parameters: {reason}")]
    InvalidSimulationParams { reason: String },
    /// Insufficient data for parameter estimation.
    #[error("Insufficient data: required at least {required}, got {actual}")]
    InsufficientData { required: usize, actual: usize },
}


impl Diagnostic for OuError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "OuError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Zero-Inflated Poisson regression.


/// Errors that can occur during ZIP regression analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ZipError {
    /// Invalid probability parameter (must be in [0, 1]).
    InvalidProbability { value: f64, parameter: String },
    /// Invalid rate parameter (must be positive).
    InvalidRate { value: f64 },
    /// Invalid count value (must be non-negative integer).
    InvalidCount { value: f64 },
    /// Insufficient data for regression.
    InsufficientData { required: usize, actual: usize },
    /// Regression convergence failed.
    ConvergenceFailed { iterations: usize, reason: String },
    /// Invalid predictor matrix dimensions.
    InvalidDimensions { expected: String, actual: String },
}


impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidProbability { value, parameter } => {
                write!(
                    f,
                    "Invalid probability for {}: {} (must be in [0, 1])",
                    parameter, value
                )
            }
            Self::InvalidRate { value } => {
                write!(f, "Invalid rate parameter: {} (must be positive)", value)
            }
            Self::InvalidCount { value } => {
                write!(
                    f,
                    "Invalid count value: {} (must be non-negative integer)",
                    value
                )
            }
            Self::InsufficientData { required, actual } => {
                write!(
                    f,
                    "Insufficient data: required at least {}, got {}",
                    required, actual
                )
            }
            Self::ConvergenceFailed { iterations, reason } => {
                write!(
                    f,
                    "Regression failed to converge after {} iterations: {}",
                    iterations, reason
                )
            }
            Self::InvalidDimensions { expected, actual } => {
                write!(
                    f,
                    "Invalid matrix dimensions: expected {}, got {}",
                    expected, actual
                )
            }
        }
    }
}


impl std::error::Error for ZipError {}


impl Diagnostic for ZipError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ZipError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Topological Data Analysis.


/// Errors that can occur in TDA computations.
#[derive(Debug, Clone, PartialEq)]
pub enum TdaError {
    /// Empty point cloud provided.
    EmptyPointCloud,
    /// Invalid radius (must be non-negative).
    InvalidRadius { value: f64 },
    /// Invalid dimension for Betti number computation.
    InvalidDimension { dimension: usize },
    /// Insufficient points for the requested operation.
    InsufficientPoints { required: usize, actual: usize },
    /// Invalid simplex (e.g., duplicate vertices).
    InvalidSimplex { reason: String },
    /// Matrix computation error.
    MatrixError { reason: String },
}


impl fmt::Display for TdaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyPointCloud => {
                write!(f, "Empty point cloud provided")
            }
            Self::InvalidRadius { value } => {
                write!(f, "Invalid radius: {} (must be non-negative)", value)
            }
            Self::InvalidDimension { dimension } => {
                write!(
                    f,
                    "Invalid dimension: {} (only 0 and 1 are supported)",
                    dimension
                )
            }
            Self::InsufficientPoints { required, actual } => {
                write!(
                    f,
                    "Insufficient points: required at least {}, got {}",
                    required, actual
                )
            }
            Self::InvalidSimplex { reason } => {
                write!(f, "Invalid simplex: {}", reason)
            }
            Self::MatrixError { reason } => {
                write!(f, "Matrix computation error: {}", reason)
            }
        }
    }
}


impl std::error::Error for TdaError {}


impl Diagnostic for TdaError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "TdaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for copula operations.


/// Errors that can occur during copula operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CopulaError {
    /// Invalid probability (must be in [0, 1]).
    InvalidProbability { value: f64 },
    /// Invalid correlation (must be in [-1, 1]).
    InvalidCorrelation { value: f64 },
    /// Dimension mismatch in matrices or vectors.
    DimensionMismatch { expected: usize, actual: usize },
    /// Matrix is not positive definite.
    NotPositiveDefinite,
    /// Matrix is not symmetric.
    NotSymmetric,
    /// Numerical computation failed.
    NumericalError { reason: String },
}


impl fmt::Display for CopulaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidProbability { value } => {
                write!(f, "Invalid probability: {} (must be in [0, 1])", value)
            }
            Self::InvalidCorrelation { value } => {
                write!(f, "Invalid correlation: {} (must be in [-1, 1])", value)
            }
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::NotPositiveDefinite => {
                write!(f, "Correlation matrix is not positive definite")
            }
            Self::NotSymmetric => {
                write!(f, "Correlation matrix is not symmetric")
            }
            Self::NumericalError { reason } => {
                write!(f, "Numerical error: {}", reason)
            }
        }
    }
}


impl std::error::Error for CopulaError {}


impl Diagnostic for CopulaError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "CopulaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Kelly Criterion calculations.


/// Errors that can occur in Kelly Criterion calculations.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum KellyError {
    /// Invalid probability (must be between 0 and 1).
    #[error("Invalid probability: {value} (must be between 0 and 1)")]
    InvalidProbability { value: f64 },
    /// Invalid odds (must be greater than 1.0 for decimal odds).
    #[error("Invalid odds: {value} (must be > 1.0)")]
    InvalidOdds { value: f64 },
    /// Invalid fraction (must be between 0 and 1).
    #[error("Invalid fraction: {value} (must be between 0 and 1)")]
    InvalidFraction { value: f64 },
    /// No edge (negative expected value - should not bet).
    #[error("No edge: p={probability}, odds={odds} results in negative expectation")]
    NoEdge { probability: f64, odds: f64 },
    /// Invalid bankroll amount (must be positive).
    #[error("Invalid bankroll: {value} (must be positive)")]
    InvalidBankroll { value: f64 },
}


impl Diagnostic for KellyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "KellyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for Glicko-2 rating system.


/// Errors that can occur in Glicko-2 rating calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum Glicko2Error {
    /// Invalid rating value.
    InvalidRating { value: f64 },
    /// Invalid rating deviation (must be positive).
    InvalidRatingDeviation { value: f64 },
    /// Invalid volatility (must be positive).
    InvalidVolatility { value: f64 },
    /// Invalid system constant tau (must be between 0.3 and 1.2).
    InvalidSystemConstant { value: f64 },
    /// Invalid opponent count (must be at least 1).
    InvalidOpponentCount { count: usize },
    /// Volatility convergence failed after maximum iterations.
    VolatilityConvergenceFailed { iterations: usize },
    /// Invalid score (must be between 0 and 1).
    InvalidScore { value: f64 },
    /// Empty match results provided.
    EmptyMatchResults,
}


impl fmt::Display for Glicko2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidRating { value } => {
                write!(f, "Invalid rating: {} (must be finite)", value)
            }
            Self::InvalidRatingDeviation { value } => {
                write!(
                    f,
                    "Invalid rating deviation: {} (must be positive and finite)",
                    value
                )
            }
            Self::InvalidVolatility { value } => {
                write!(
                    f,
                    "Invalid volatility: {} (must be positive and finite)",
                    value
                )
            }
            Self::InvalidSystemConstant { value } => {
                write!(
                    f,
                    "Invalid system constant tau: {} (must be between 0.3 and 1.2)",
                    value
                )
            }
            Self::InvalidOpponentCount { count } => {
                write!(f, "Invalid opponent count: {} (must be at least 1)", count)
            }
            Self::VolatilityConvergenceFailed { iterations } => {
                write!(
                    f,
                    "Volatility convergence failed after {} iterations",
                    iterations
                )
            }
            Self::InvalidScore { value } => {
                write!(f, "Invalid score: {} (must be between 0 and 1)", value)
            }
            Self::EmptyMatchResults => {
                write!(f, "Empty match results provided")
            }
        }
    }
}


impl std::error::Error for Glicko2Error {}


impl Diagnostic for Glicko2Error {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "Glicko2Error".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}
