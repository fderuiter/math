//! # Pure Mathematics
//!
//! This module focuses on theoretical structures and rigorous proofs implemented as code.
//! Unlike the `applied` module, which prioritizes utility, `pure_math` prioritizes
//! structural correctness and the exploration of abstract concepts.
//!
//! ## Domains
//!
//! - **Number Theory**: `number_theory`, `elliptic_curves`, `algebra`.
//!   - Partitions, Primes, Q-Series, and Modular Forms.
//! - **Geometry & Topology**: `differential_geometry`, `graph_theory`.
//!   - Surface Calculus, Curvature Flow, and Graph algorithms.
//! - **Information Theory**: `algorithmic_information`.
//!   - Kolmogorov complexity and combinatorial bounds.
//! - **Analysis**: `analysis`.
//!   - Differential Equations (ODE/PDE) and core calculus tools.

/// Abstract algebraic structures (Rings, Fields, Groups).
pub mod algebra;

/// Algorithmic Information Theory (AIT), including Kolmogorov complexity
/// and symmetry deficiency.
pub mod algorithmic_information;

/// Elliptic curves, modular forms, and p-adic valuation bounds.
pub mod elliptic_curves;

/// Core number theoretic functions: Partitions, Primes, and Class Numbers.
pub mod number_theory;

/// Graph theory algorithms and parameters (Dijkstra, Treewidth).
pub mod graph_theory;

/// Differential geometry on surfaces, including Heat Equation and Mean Curvature Flow.
pub mod differential_geometry;

/// Mathematical analysis, specifically Ordinary and Partial Differential Equations.
pub mod analysis;
