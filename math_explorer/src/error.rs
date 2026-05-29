use thiserror::Error;
use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;
#[cfg(feature = "physics")]
use crate::physics::solid_state::types::ElectronVolts;
#[cfg(feature = "pure_math")]
use crate::pure_math::analysis::roots::AnalysisError;


#[cfg(feature = "ai")]
/// Errors related to AI/Machine Learning calculations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AIError {
    /// Dimension mismatch between tensors/matrices.
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: String, got: String },

    /// Invalid parameter value (e.g. negative learning rate).
    #[error("Invalid parameter {name}: {value}")]
    InvalidParameter { name: String, value: f64 },

    /// Optimization failed or diverged.
    #[error("Optimization diverged")]
    OptimizationDivergence,

    /// Type conversion failed.
    #[error("Conversion error: {reason}")]
    ConversionError { reason: String },

    /// State is uninitialized.
    #[error("Uninitialized state: {name}")]
    UninitializedState { name: String },

    /// Mismatched data distribution lengths.
    #[error("Distributions have different lengths: expected {expected}, got {got}")]
    DistributionLengthMismatch { expected: usize, got: usize },
}


#[cfg(feature = "pure_math")]
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

/// Error types for Kelly Criterion calculations.


#[cfg(feature = "pure_math")]
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

/// Error types for Markov chain operations.


#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
impl std::error::Error for MarkovError {}

/// Type alias for Results with MarkovError.
#[cfg(feature = "pure_math")]
pub type Result<T> = std::result::Result<T, MarkovError>;

/// Error types for Ornstein-Uhlenbeck process.


#[cfg(feature = "pure_math")]
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

/// Error types for Zero-Inflated Poisson regression.


#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
impl std::error::Error for ZipError {}

/// Error types for Topological Data Analysis.


#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
impl std::error::Error for TdaError {}

/// Error types for copula operations.


#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
impl std::error::Error for CopulaError {}

/// Error types for Glicko-2 rating system.


#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
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

#[cfg(feature = "pure_math")]
impl std::error::Error for Glicko2Error {}


#[cfg(feature = "biology")]
/// Errors related to Hodgkin-Huxley neuron modeling.
#[derive(Error, Debug, PartialEq)]
pub enum HodgkinHuxleyError {
    #[error("Invalid gating variable value: {0} (must be between 0 and 1)")]
    InvalidGatingVariable(f64),
    #[error("Invalid conductance value: {0} (must be non-negative)")]
    InvalidConductance(f64),
}


#[cfg(feature = "physics")]
/// Errors that can occur during chaos analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ChaosError {
    /// The simulation parameters are invalid (e.g., negative time step).
    InvalidParameter(String),
    /// The calculation failed (e.g., trajectories converged completely).
    CalculationError(String),
}

#[cfg(feature = "physics")]
impl fmt::Display for ChaosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChaosError::InvalidParameter(msg) => write!(f, "Invalid chaos parameter: {}", msg),
            ChaosError::CalculationError(msg) => write!(f, "Chaos calculation error: {}", msg),
        }
    }
}

#[cfg(feature = "physics")]
impl std::error::Error for ChaosError {}


#[cfg(feature = "physics")]
/// Errors related to Standard Model calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum StandardModelError {
    /// Energy scale must be positive.
    InvalidEnergyScale { scale: f64, context: String },
    /// Coupling constant must be positive.
    InvalidCouplingConstant { alpha: f64 },
    /// The coupling constant diverges at this scale (Landau Pole).
    LandauPole { scale: f64 },
    /// Invalid number of quark flavors (e.g., negative).
    InvalidFlavors { nf: f64 },
}

#[cfg(feature = "physics")]
impl fmt::Display for StandardModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEnergyScale { scale, context } => write!(
                f,
                "Invalid energy scale in {}: got {}, expected > 0",
                context, scale
            ),
            Self::InvalidCouplingConstant { alpha } => {
                write!(f, "Coupling constant must be positive, got {}", alpha)
            }
            Self::LandauPole { scale } => write!(
                f,
                "Landau pole encountered: coupling diverges at scale {}",
                scale
            ),
            Self::InvalidFlavors { nf } => {
                write!(f, "Number of flavors must be non-negative, got {}", nf)
            }
        }
    }
}

#[cfg(feature = "physics")]
impl std::error::Error for StandardModelError {}


#[cfg(feature = "physics")]
/// Errors that can occur during Solid State Physics calculations.
#[derive(Error, Debug, PartialEq)]
pub enum SolidStateError {
    /// The iterative solver failed to converge within the maximum number of iterations.
    #[error("Convergence failed after {0} iterations. Last value: {1}")]
    ConvergenceFailure(usize, ElectronVolts),

    /// A parameter provided to the model or solver was invalid.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// A general computation error occurred.
    #[error("Computation error: {0}")]
    ComputationError(String),
}


#[cfg(feature = "physics")]
#[derive(Error, Debug, PartialEq)]
pub enum DoseFluenceError {
    #[error("Radius cannot be zero (singularity at r=0)")]
    Singularity,
    #[error("Radius must be non-negative")]
    NegativeRadius,
    #[error("Physical quantity must be non-negative: {0}")]
    InvalidPhysicalQuantity(String),
}


#[cfg(feature = "physics")]
/// Errors for Radar Gating processing.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum RadarError {
    /// Chirp length mismatch.
    #[error("Chirp length {actual} does not match expected {expected}")]
    ChirpLengthMismatch { expected: usize, actual: usize },

    /// Insufficient snapshots for MUSIC algorithm.
    #[error("Not enough snapshots to compute stable Covariance Matrix: {actual} < {required}")]
    InsufficientSnapshots { required: usize, actual: usize },

    /// Signal subspace dimension error (e.g. >= samples).
    #[error("Signal subspace dimension {subspace} equals or exceeds sample size {samples}")]
    InvalidSignalSubspace { samples: usize, subspace: usize },

    /// Invalid configuration parameters.
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Numerical instability detected (e.g. NaN/Inf).
    #[error("Numerical instability detected: {0}")]
    NumericalInstability(String),
}


#[cfg(feature = "physics")]
/// Errors related to High Energy Physics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum HighEnergyError {
    /// Mass must be positive.
    InvalidMass { mass: f64 },
    /// Radius must be greater than Schwarzschild radius.
    InvalidRadius { radius: f64, limit: f64 },
    /// Energy density cannot be negative.
    InvalidEnergyDensity { u_b: f64 },
    /// Lorentz factor must be >= 1.
    InvalidLorentzFactor { gamma: f64 },
    /// Power law index p must be > 1.
    InvalidPowerLawIndex { p: f64 },
    /// Adiabatic index must be > 1.
    InvalidAdiabaticIndex { gamma: f64 },
    /// Density must be positive.
    InvalidDensity { rho: f64 },
    /// Pressure cannot be negative.
    InvalidPressure { p: f64 },
    /// Velocity is invalid (e.g. >= c or < 0 if speed).
    InvalidVelocity { v: f64 },
    /// Momentum is invalid (e.g. m^2 + p^2 < 0 somehow? or just checking validity).
    InvalidMomentum,
    /// Invalid statistics parameters (e.g. negative counts).
    InvalidStatisticsParams { reason: String },
    /// Error during calculation (e.g. negative sqrt).
    CalculationError { reason: String },
}

#[cfg(feature = "physics")]
impl fmt::Display for HighEnergyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMass { mass } => write!(f, "Mass must be positive, got {}", mass),
            Self::InvalidRadius { radius, limit } => write!(
                f,
                "Radius {} must be greater than Schwarzschild radius {}",
                radius, limit
            ),
            Self::InvalidEnergyDensity { u_b } => {
                write!(f, "Energy density must be non-negative, got {}", u_b)
            }
            Self::InvalidLorentzFactor { gamma } => {
                write!(f, "Lorentz factor must be >= 1, got {}", gamma)
            }
            Self::InvalidPowerLawIndex { p } => write!(f, "Power law index must be > 1, got {}", p),
            Self::InvalidAdiabaticIndex { gamma } => {
                write!(f, "Adiabatic index must be > 1, got {}", gamma)
            }
            Self::InvalidDensity { rho } => write!(f, "Density must be positive, got {}", rho),
            Self::InvalidPressure { p } => write!(f, "Pressure must be non-negative, got {}", p),
            Self::InvalidVelocity { v } => write!(f, "Velocity {} is invalid (must be < c)", v),
            Self::InvalidMomentum => write!(f, "Momentum vector is invalid"),
            Self::InvalidStatisticsParams { reason } => {
                write!(f, "Invalid statistics parameters: {}", reason)
            }
            Self::CalculationError { reason } => write!(f, "Calculation error: {}", reason),
        }
    }
}

#[cfg(feature = "physics")]
impl std::error::Error for HighEnergyError {}

/// Errors for Fluid Dynamics.


#[cfg(feature = "physics")]
/// Errors related to fluid properties and dynamics.
#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// Density must be strictly positive.
    InvalidDensity { value: f64 },
    /// Dynamic viscosity must be non-negative.
    InvalidViscosity { value: f64 },
}

#[cfg(feature = "physics")]
impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::InvalidDensity { value } => {
                write!(f, "Density must be strictly positive, got {}", value)
            }
            FluidError::InvalidViscosity { value } => {
                write!(f, "Dynamic viscosity must be non-negative, got {}", value)
            }
        }
    }
}

#[cfg(feature = "physics")]
impl std::error::Error for FluidError {}


#[cfg(feature = "physics")]
/// Errors related to Statistical Mechanics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum StatMechError {
    /// Temperature must be non-negative (absolute zero is allowed as a limit).
    InvalidTemperature(f64),
    /// Chemical potential is invalid for the particle type and state (e.g., > Energy for Bosons).
    InvalidChemicalPotential {
        chemical_potential: f64,
        energy: f64,
        reason: String,
    },
    /// Numerical instability (e.g. division by zero).
    NumericalInstability(String),
}

#[cfg(feature = "physics")]
impl fmt::Display for StatMechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatMechError::InvalidTemperature(t) => write!(f, "Invalid temperature: {} K", t),
            StatMechError::InvalidChemicalPotential {
                chemical_potential,
                energy,
                reason,
            } => write!(
                f,
                "Invalid chemical potential mu={} for E={}: {}",
                chemical_potential, energy, reason
            ),
            StatMechError::NumericalInstability(msg) => write!(f, "Numerical instability: {}", msg),
        }
    }
}

#[cfg(feature = "physics")]
impl std::error::Error for StatMechError {}


#[cfg(feature = "applied")]
/// Errors that can occur in pharmacokinetic modeling.
#[derive(Debug, Error)]
pub enum PharmacokineticsError {
    /// An invalid parameter was provided (e.g., negative volume or rate constant).
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// A numerical analysis method failed (e.g., root finding did not converge).
    #[error("Analysis error: {0}")]
    AnalysisError(#[from] AnalysisError),
}

/// Error types for battery degradation modeling.


#[cfg(feature = "applied")]
/// Errors that can occur when instantiating battery parameters.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum BatteryError {
    /// Depth of Discharge must be between 0.0 and 100.0.
    #[error("DepthOfDischarge must be between 0.0 and 100.0, got {0}")]
    InvalidDepthOfDischarge(f64),
    /// Capacity must be between 0.0 and 1.0.
    #[error("Capacity must be between 0.0 and 1.0, got {0}")]
    InvalidCapacity(f64),
    /// Cycles cannot be negative.
    #[error("Cycles cannot be negative, got {0}")]
    NegativeCycles(f64),
}


#[cfg(feature = "applied")]
/// Errors for Game Theory calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum GameTheoryError {
    /// Payoff matrix is not square.
    NonSquarePayoffMatrix { rows: usize, cols: usize },
    /// Invalid parameter value (e.g. frequency outside [0, 1]).
    InvalidParameter { name: String, value: f64 },
}

#[cfg(feature = "applied")]
impl fmt::Display for GameTheoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonSquarePayoffMatrix { rows, cols } => write!(
                f,
                "Payoff matrix must be square, but dimensions are ({}, {})",
                rows, cols
            ),
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
        }
    }
}

#[cfg(feature = "applied")]
impl std::error::Error for GameTheoryError {}


#[cfg(feature = "applied")]
/// Errors related to Engineering Calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineeringError {
    /// Invalid parameter value (e.g. TotalBits = 0).
    InvalidParameter { name: String, value: f64 },
}

#[cfg(feature = "applied")]
impl fmt::Display for EngineeringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
        }
    }
}

#[cfg(feature = "applied")]
impl std::error::Error for EngineeringError {}


#[cfg(feature = "applied")]
/// Errors that can occur during LoRA ensemble combination.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoraError {
    #[error("Ensemble is empty; cannot combine.")]
    EmptyEnsemble,
    #[error("Weights cannot be empty.")]
    EmptyWeights,
    #[error("The number of weights must match the number of modules in the ensemble.")]
    WeightModuleMismatch,
    #[error("Mismatched tensor shapes for the same key.")]
    TensorShapeMismatch,
    #[error("Mismatched keys between LoRA modules.")]
    KeyMismatch,
}


#[cfg(feature = "applied")]
/// Errors that can occur during isosurface extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum IsosurfaceError {
    /// The grid dimensions are too small (must be at least 2x2x2).
    InvalidGrid(String),
    /// The data buffer size matches the grid dimensions.
    DataMismatch { expected: usize, actual: usize },
}

#[cfg(feature = "applied")]
impl fmt::Display for IsosurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsosurfaceError::InvalidGrid(msg) => write!(f, "Invalid grid dimensions: {}", msg),
            IsosurfaceError::DataMismatch { expected, actual } => {
                write!(f, "Data mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

#[cfg(feature = "applied")]
impl std::error::Error for IsosurfaceError {}


#[cfg(feature = "epidemiology")]
/// Errors related to Epidemiology calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EpidemiologyError {
    /// Matrix V (Transition Matrix) is singular and cannot be inverted.
    SingularTransitionMatrix,
    /// Invalid Parameter (e.g., negative rate).
    InvalidParameter { name: String, value: f64 },
    /// Missing Parameter (e.g., required field not set in builder).
    MissingParameter { name: String },
    /// Matrix dimensions mismatch.
    DimensionMismatch {
        f_rows: usize,
        f_cols: usize,
        v_rows: usize,
        v_cols: usize,
    },
}

#[cfg(feature = "epidemiology")]
impl fmt::Display for EpidemiologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingularTransitionMatrix => write!(
                f,
                "Transition matrix V is singular, Next Generation Matrix cannot be computed."
            ),
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
            Self::MissingParameter { name } => {
                write!(f, "Missing parameter: {}", name)
            }
            Self::DimensionMismatch {
                f_rows,
                f_cols,
                v_rows,
                v_cols,
            } => write!(
                f,
                "Matrix dimensions mismatch: F=({}, {}), V=({}, {})",
                f_rows, f_cols, v_rows, v_cols
            ),
        }
    }
}

#[cfg(feature = "epidemiology")]
impl std::error::Error for EpidemiologyError {}


#[cfg(feature = "ai")]
impl Diagnostic for AIError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "AIError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for NumberTheoryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "NumberTheoryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for KellyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "KellyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for MarkovError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "MarkovError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for OuError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "OuError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for ZipError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ZipError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for TdaError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "TdaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for CopulaError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "CopulaError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "pure_math")]
impl Diagnostic for Glicko2Error {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "Glicko2Error".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "biology")]
impl Diagnostic for HodgkinHuxleyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "HodgkinHuxleyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for ChaosError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ChaosError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for StandardModelError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StandardModelError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for SolidStateError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "SolidStateError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for DoseFluenceError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "DoseFluenceError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for RadarError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "RadarError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for HighEnergyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "HighEnergyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for FluidError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "FluidError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "physics")]
impl Diagnostic for StatMechError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StatMechError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for PharmacokineticsError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "PharmacokineticsError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for BatteryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "BatteryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for GameTheoryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "GameTheoryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for EngineeringError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "EngineeringError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for LoraError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "LoraError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "applied")]
impl Diagnostic for IsosurfaceError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "IsosurfaceError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

#[cfg(feature = "epidemiology")]
impl Diagnostic for EpidemiologyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "EpidemiologyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}
