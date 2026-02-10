//! # Pure Mathematics
//!
//! Foundational algorithms and structures from abstract mathematics.
//!
//! Unlike the `applied` or `physics` modules, which focus on modeling real-world phenomena,
//! this module provides the rigorous mathematical building blocks. It covers fields from
//! Number Theory to Differential Geometry.
//!
//! ## Modules
//!
//! *   **[Algebra](algebra)**: Abstract algebraic structures (Groups, Rings, Fields).
//! *   **[Algorithmic Information](algorithmic_information)**: Kolmogorov complexity and information theory.
//! *   **[Analysis](analysis)**: Calculus, Integration, and Differential Equations.
//! *   **[Differential Geometry](differential_geometry)**: Manifolds and curvature.
//! *   **[Elliptic Curves](elliptic_curves)**: Cryptography primitives and geometric arithmetic.
//! *   **[Graph Theory](graph_theory)**: Network analysis and traversal algorithms.
//! *   **[Number Theory](number_theory)**: Primes, Partitions, and Modular Arithmetic.
//! *   **[Statistics](statistics)**: Competitive modeling (Glicko-2), Stochastic processes (Markov), and Data Analysis.
//!
//! ## Philosophy
//!
//! Implementations prioritize *correctness* and *readability* over raw performance optimization,
//! serving as reference implementations for educational purposes.

pub mod algebra;
pub mod algorithmic_information;
pub mod analysis;
pub mod differential_geometry;
pub mod elliptic_curves;
pub mod graph_theory;
pub mod logic;
pub mod number_theory;
pub mod special_functions;
pub mod statistics;
pub mod tensor;
pub mod vector_calculus;
