//! # Pure Mathematics
//!
//! This module implements foundational structures and theorems from abstract and analytical mathematics.
//!
//! ## Domains
//!
//! ### 🔢 Number Theory & Algebra
//! - **`number_theory`**: Partition functions, Q-Series (Euler/Pentagonal), and Primality testing.
//! - **`elliptic_curves`**: Modular polynomials, p-adic valuations, and Theorem bounds from Breuer.
//! - **`algebra`**: Abstract traits (Rings, Fields) and basic structures.
//!
//! ### 📐 Geometry & Topology
//! - **`differential_geometry`**: Surface calculus, Heat Equation on Manifolds, and Mean Curvature Flow.
//! - **`graph_theory`**: Shortest path algorithms (Dijkstra) and Graph parameters.
//!
//! ### 📈 Analysis
//! - **`analysis`**: Ordinary Differential Equations (Runge-Kutta 4, Euler) and PDE classifications.
//! - **`algorithmic_information`**: Kolmogorov complexity, combinatorics, and information bounds.

/// Abstract Algebra (Groups, Rings, Fields).
pub mod algebra;

/// Algorithmic Information Theory (Kolmogorov Complexity).
pub mod algorithmic_information;

/// Analysis (ODE/PDE Solvers).
pub mod analysis;

/// Differential Geometry (Curvature Flow, Heat Equation).
pub mod differential_geometry;

/// Elliptic Curves and Modular Forms.
pub mod elliptic_curves;

/// Graph Theory (Algorithms, Parameters).
pub mod graph_theory;

/// Number Theory (Partitions, Primes, Q-Series).
pub mod number_theory;
