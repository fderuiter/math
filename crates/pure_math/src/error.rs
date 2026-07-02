use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use thiserror::Error;

pub type Result<T> = math_commons::error::MathResult<T>;

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

#[derive(Debug, Clone, PartialEq)]
pub enum MarkovError {
    /// Invalid probability value (must be in [0, 1] and finite).
    InvalidProbability {
        /// The invalid probability value.
        value: f64,
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

    Math(math_commons::error::MathError),
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum CopulaError {
    /// Invalid probability (must be in [0, 1]).
    InvalidProbability { value: f64 },
    /// Invalid correlation (must be in [-1, 1]).
    InvalidCorrelation { value: f64 },
    
    /// Matrix is not positive definite.
    NotPositiveDefinite,
    /// Matrix is not symmetric.
    NotSymmetric,
    

    Math(math_commons::error::MathError),
}

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

impl Diagnostic for NumberTheoryError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "NumberTheoryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for KellyError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "KellyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for MarkovError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "MarkovError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for OuError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "OuError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for ZipError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ZipError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for TdaError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "TdaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for CopulaError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "CopulaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for Glicko2Error {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "Glicko2Error".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl std::fmt::Display for MarkovError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MarkovError {}

impl std::fmt::Display for ZipError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ZipError {}

impl std::fmt::Display for TdaError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TdaError {}

impl std::fmt::Display for CopulaError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CopulaError {}

impl std::fmt::Display for Glicko2Error {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Glicko2Error {}


impl From<math_commons::error::MathError> for MarkovError {
    fn from(err: math_commons::error::MathError) -> Self {
        MarkovError::Math(err)
    }
}

impl From<math_commons::error::MathError> for CopulaError {
    fn from(err: math_commons::error::MathError) -> Self {
        CopulaError::Math(err)
    }
}
